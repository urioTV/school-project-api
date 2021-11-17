#[macro_use] extern crate rocket;
use serde_json::{Value};
use rand::Rng;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}
#[get("/reddit/<sub>")]
async fn reddit(sub: String) -> String {
    let suburl = format!("https://www.reddit.com/r/{}.json?sort=hot", sub);
    let subjson = reqwest::get(&suburl).await.unwrap().text().await.unwrap();
    let subdata: Value = serde_json::from_str(&subjson).unwrap();
    let sublen = subdata["data"]["dist"].as_u64().unwrap() as usize;
    if sublen == 0 {
        return format!("No posts found for {} or you typed it wrong", sub);
    }
    let rndnum = rand::thread_rng().gen_range(0..sublen);
    format!("{}", subdata["data"]["children"][rndnum]["data"]["url"])
}


#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index, reddit])
                .launch()
                .await.expect("Failed to launch rocket");
}