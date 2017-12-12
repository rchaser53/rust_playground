#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::local::Client;
use rocket::http::{Header, Method};

#[get("/")]
fn hello() -> String {
    let ignited_rocket = rocket::ignite();
    let client = Client::new(ignited_rocket).expect("valid rocket");
    let mut req = client.get("hoge");
    req.add_header(Header::new("User-Agent", "nyan"));
    let mut res = req.dispatch();
    res.body_string().unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}

    // let mut req = client.get("https://api.github.com/repos/rchaser53/vue-table-playground/commits");