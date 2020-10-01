mod preview;
mod reference;

use preview::Preview;
use reference::Reference;
use wordfun::{plural, Dictionary, Popularity, Results};

use actix_files::NamedFile;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use anyhow::Result;
use env_logger::Env;
use itertools::Itertools;
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PreviewQuery {
    q: String,
}

#[derive(Serialize)]
struct FullResults {
    words: Vec<Match>,
}

#[derive(Serialize)]
struct Match {
    word: String,
    definition: Option<String>,
    score: Option<u32>,
}

#[derive(Serialize)]
struct ThesaurusResponse {
    count: String,
    query: String,
    words: Vec<(String, Vec<String>)>,
}

async fn index(r: web::Data<Reference>) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(r.assets_dir().join("index.html"))?)
}

async fn asset(req: HttpRequest, r: web::Data<Reference>) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(r.assets_dir().join(&req.path()[1..]))?)
}

async fn preview_an(query: web::Query<PreviewQuery>, r: web::Data<Reference>) -> HttpResponse {
    let results = r.lexicon().anagram(&query.q);
    let preview = Preview::new(20, 5, r.popularity());
    HttpResponse::Ok().json(preview.build(&results.key, &results.words))
}

async fn preview_fw(query: web::Query<PreviewQuery>, r: web::Data<Reference>) -> HttpResponse {
    let results = r.lexicon().find_word(&query.q);
    let preview = Preview::new(20, 5, r.popularity());
    HttpResponse::Ok().json(preview.build(&results.key, &results.words))
}

async fn full_an(query: web::Query<PreviewQuery>, r: web::Data<Reference>) -> HttpResponse {
    full_results(
        r.lexicon().anagram(&query.q),
        r.dictionary(),
        r.popularity(),
    )
}

async fn full_fw(query: web::Query<PreviewQuery>, r: web::Data<Reference>) -> HttpResponse {
    full_results(
        r.lexicon().find_word(&query.q),
        r.dictionary(),
        r.popularity(),
    )
}

fn full_results<'a>(rs: Results<'a>, dict: &Dictionary, popularity: &Popularity) -> HttpResponse {
    HttpResponse::Ok().json(FullResults {
        words: rs
            .into_iter()
            .map(|word| {
                let definition = dict.lookup(word).map(|s| s.to_string());
                let score = if popularity.is_ranked(word) {
                    Some(1)
                } else {
                    None
                };
                Match {
                    word: word.to_string(),
                    score,
                    definition,
                }
            })
            .collect(),
    })
}

async fn preview_thesaurus(
    params: web::Query<PreviewQuery>,
    r: web::Data<Reference>,
) -> HttpResponse {
    let query = params.q.to_string();
    let mut result = r.thesaurus().lookup(&query).collect::<Vec<_>>();
    result.sort();
    result.sort_by_cached_key(|word| &word.word_lengths);
    let mut num_words = 0;
    let mut grouped_words = Vec::new();
    for (lengths, group) in result
        .into_iter()
        .group_by(|word| &word.word_lengths)
        .into_iter()
    {
        let words: Vec<_> = group.map(|word| word.to_string()).collect();
        num_words += words.len();
        grouped_words.push((lengths.format(), words));
    }

    let count = plural(num_words, "match", "matches");

    HttpResponse::Ok().json(ThesaurusResponse {
        count,
        query,
        words: grouped_words,
    })
}

async fn version() -> HttpResponse {
    let version = std::env::var("COMMIT_ID").unwrap_or_else(|_| "".to_string());
    HttpResponse::Ok().body(version)
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let reference = Reference::new()?;

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(reference.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(index))
            .route("/{app}.js", web::get().to(asset))
            .route("/{app}.css", web::get().to(asset))
            .route("/{app}.svg", web::get().to(asset))
            .route("/{app}.map", web::get().to(asset))
            .route("/preview/an", web::get().to(preview_an))
            .route("/preview/fw", web::get().to(preview_fw))
            .route("/preview/thesaurus", web::get().to(preview_thesaurus))
            .route("/words/an", web::get().to(full_an))
            .route("/words/fw", web::get().to(full_fw))
            .route("/version.txt", web::get().to(version))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        let port = std::env::var("WORDFUN_API_PORT").unwrap_or_else(|_| "3000".to_string());
        let addr = format!("0.0.0.0:{}", &port);
        server.bind(&addr)?
    };

    Ok(server.run().await?)
}
