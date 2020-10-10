mod preview;
mod refdata;

use crate::Options;
use preview::Preview;
use refdata::Reference;

use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use anyhow::{ensure, Context, Result};
use itertools::Itertools;
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use wordfun::{plural, Dictionary, Popularity, Results};

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

fn asset_file(r: &Reference, name: &str) -> actix_web::Result<NamedFile> {
    if let Some(dir) = r.assets_dir() {
        Ok(NamedFile::open(dir.join(name))?)
    } else {
        Err(actix_web::error::ErrorNotFound("Not found"))
    }
}

async fn api_index() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./api-index.html"))
}

async fn index(r: web::Data<Reference>) -> actix_web::Result<NamedFile> {
    asset_file(&r, "index.html")
}

async fn asset(req: HttpRequest, r: web::Data<Reference>) -> actix_web::Result<NamedFile> {
    asset_file(&r, &req.path()[1..])
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
pub async fn serve(options: Options) -> Result<()> {
    if let Some(dir) = &options.assets_dir {
        ensure!(
            dir.exists(),
            "ASSETS_DIR is set to {:?}, but that directory does not exist.",
            dir
        );
    }

    let reference = Reference::new(&options).context("Could not load reference data")?;
    let mut server = HttpServer::new(move || {
        let mut app = App::new()
            .data(reference.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .route("/preview/an", web::get().to(preview_an))
            .route("/preview/fw", web::get().to(preview_fw))
            .route("/preview/thesaurus", web::get().to(preview_thesaurus))
            .route("/words/an", web::get().to(full_an))
            .route("/words/fw", web::get().to(full_fw))
            .route("/version.txt", web::get().to(version));

        app = if reference.assets_dir().is_some() {
            app.route("/", web::get().to(index))
                .route("/{app}.js", web::get().to(asset))
                .route("/{app}.css", web::get().to(asset))
                .route("/{app}.svg", web::get().to(asset))
                .route("/{app}.map", web::get().to(asset))
        } else {
            app.route("/", web::get().to(api_index))
        };

        app
    });

    // The ListenFd thing is for auto-reloading the development server:
    //   https://actix.rs/docs/autoreload/
    let mut listenfd = ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind((options.bind_addr, options.server_port))?
    };

    Ok(server.run().await?)
}
