use std::collections::{HashMap, HashSet};

use actix_web::{get, web, App, HttpServer, Responder};
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    thread_rng,
};

#[get("/random")]
async fn random(data: web::Data<HashMap<Character, Quotes>>) -> impl Responder {
    let quot = data.values().choose(&mut thread_rng()).unwrap();
    let rand_quote = quot.choose(&mut thread_rng()).unwrap().to_owned();
    web::Json(rand_quote)
}

#[get("/quotes")]
async fn quotes(data: web::Data<HashMap<Character, Quotes>>) -> impl Responder {
    let mut rng = thread_rng();
    let mut random_quotes = HashSet::<&'static str>::new();
    while random_quotes.len() != 5 {
        let quot = data.values().choose(&mut rng).unwrap();
        let rand_quote = quot.choose(&mut rng).unwrap().to_owned();
        random_quotes.insert(rand_quote);
    }

    web::Json(random_quotes)
}

#[get("/characters")]
async fn characters(data: web::Data<HashMap<Character, Quotes>>) -> impl Responder {
    let chr: Vec<&'static str> = data.keys().cloned().collect();
    web::Json(chr)
}

#[get("/characters/{name}")]
async fn quote_by_name(
    data: web::Data<HashMap<Character, Quotes>>,
    name: web::Path<String>,
) -> impl Responder {
    if let Some(quote) = data.get(&*name.to_owned()) {
        let random_quote = quote.choose(&mut thread_rng()).unwrap().to_owned();
        web::Json(random_quote)
    } else {
        web::Json("No Such Character")
    }
}

type Character = &'static str;
type Quotes = Vec<&'static str>;

fn get_quotes() -> std::io::Result<HashMap<Character, Quotes>> {
    let file = include_str!("../quotes.json");
    let all_quotes: HashMap<Character, Quotes> = serde_json::from_str(file)?;
    Ok(all_quotes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                get_quotes().expect("Failed to parse the quotes"),
            ))
            .service(
                web::scope("/api")
                    .service(random)
                    .service(quotes)
                    .service(characters)
                    .service(quote_by_name),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
