#[macro_use] extern crate rocket;

use serde_json::{Value};
use rand::Rng;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
  
    }
}


#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}
#[get("/reddit/<sub>")]
async fn reddit(sub: String) -> String {
    let suburl = format!("https://www.reddit.com/r/{}.json?sort=hot", sub);
    let subjson = reqwest::get(&suburl).await.unwrap().text().await.unwrap();
    let subdata: Value = serde_json::from_str(&subjson).unwrap();
    if !subdata["error"].is_null() {
        return "".to_string();
    }
    let sublen = subdata["data"]["dist"].as_u64().unwrap() as usize;
    if sublen == 0 {
        return "".to_string();
    }
    let mut rndnum = rand::thread_rng().gen_range(0..sublen);
    let mut urlimg = format!("{}", subdata["data"]["children"][rndnum]["data"]["url"]);
    let mut count = 0;
    while urlimg.contains("comments") || urlimg.contains("gallery") || urlimg.contains("v.redd") {
        rndnum = rand::thread_rng().gen_range(0..sublen);
        urlimg = format!("{}", subdata["data"]["children"][rndnum]["data"]["url"]);
        count += 1;
        if count > 15 {
            return "".to_string();
        }
    }
    return urlimg;
}


#[rocket::main]
async fn main() {
    rocket::build().attach(CORS)
                .mount("/", routes![index, reddit])
                .launch()
                .await.expect("Failed to launch rocket");
}