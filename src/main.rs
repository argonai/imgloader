use std::{fmt::format, io::stdout};

#[macro_use] extern crate rocket;
// #[macro_use] extern crate serde_json;
// static mut name: &str = "World";

#[get("/")]
fn index() -> String {
    let name = "world";
    format!("Hello, {}!", name)
    
}

#[post("/", data="<name>")]
fn create_index(name : String) -> String {
    print!("{}", name);
    format!("hello, {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/" , routes![index, create_index])
}