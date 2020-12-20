mod preview;
mod refdata;

use crate::Options;
use preview::Preview;
use refdata::Reference;

use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use anyhow::{ensure, Context, Result};
use itertools::Itertools;
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use wordfun::{plural, Dictionary, Popularity, Results};

/// Request type for a preview query (anagram, find-word, or thesaurus)
#[derive(Deserialize)]
struct PreviewQuery {
    /// The search term being requested. Each of the searches will interpret this string
    /// differently, but what they have in common is that they all take a single string as input.
    ///
    /// (At some point, there may be an "advanced search" feature that will take a richer data
    /// object as input, but this is what we use for now!)
    q: String,
}

/// Result type for a full anagram or find-word search
#[derive(Serialize)]
struct FullResults {
    /// The result set is just a list of entries.
    words: Vec<Match>,
}

/// A single word in the full result for anagram or find-word
#[derive(Serialize)]
struct Match {
    /// The word we found
    word: String,
    /// If we have a definition for the word, it goes here
    definition: Option<String>,
    /// Used for highlighting rows in the full results.  The current rule is that if
    /// score is set to something greater than 0, the row is highlighted.
    score: Option<u32>,
}

/// The response for a thesaurus query
#[derive(Serialize)]
struct ThesaurusResponse {
    /// Total number of matches, as a string like "1 match" or "137 matches"
    count: String,
    /// The word we looked up
    query: String,
    /// Matching words, grouped by word length. The key is a string showing word lengths in the
    /// group, like "10" or "8 (3, 5)", and the value is a list of words.
    ///
    /// The groups are in numerical order, so 3-letter words come before 4-letter words. When there
    /// are multi-word alternatives, they are listed in lexical order after the single word.  For
    /// example, words with 13 letters might have groups in this order:
    ///   +------------+--------------------------------+
    ///   | 13         | affenpinscher                  |
    ///   | 13 (5,3,5) | pluck the beard                |
    ///   | 13 (5,8)   | Great Pyrenees                 |
    ///   | 13 (6,7)   | Border terrier, turkey gobbler |
    ///   | 13 (8,5)   | Siberian husky                 |
    ///   +------------+--------------------------------+
    ///
    words: Vec<(String, Vec<String>)>,
}

/// Serve a static HTML page for the root path when running in API mode
async fn api_index() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./api-index.html"))
}

/// Serve an anagram preview
async fn preview_an(query: web::Query<PreviewQuery>, r: web::Data<Reference>) -> HttpResponse {
    let results = r.lexicon().anagram(&query.q);
    let preview = Preview::new(20, 5, r.popularity());
    HttpResponse::Ok().json(preview.build(&results.key, &results.words))
}

/// Serve a find-word preview
async fn preview_fw(query: web::Query<PreviewQuery>, r: web::Data<Reference>) -> HttpResponse {
    let results = r.lexicon().find_word(&query.q);
    let preview = Preview::new(20, 5, r.popularity());
    HttpResponse::Ok().json(preview.build(&results.key, &results.words))
}

/// Serve the full anagram lookup (including definitions)
async fn full_an(query: web::Query<PreviewQuery>, r: web::Data<Reference>) -> HttpResponse {
    full_results(
        r.lexicon().anagram(&query.q),
        r.dictionary(),
        r.popularity(),
    )
}

/// Serve the full find-word lookup (including definitions)
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

/// Serve the thesaurus lookup
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

/// Serve the build ID from the `SOURCE_COMMIT` environment variable.
async fn version() -> HttpResponse {
    let version = std::env::var("SOURCE_COMMIT").unwrap_or_else(|_| "".to_string());
    HttpResponse::Ok().body(version)
}

/// Start up a web server
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

        app = if let Some(assets_dir) = reference.assets_dir() {
            app.service(fs::Files::new("/", assets_dir).index_file("index.html"))
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
