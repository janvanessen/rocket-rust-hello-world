#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello {}!", name)
}

#[get("/")]
fn home() -> &'static str {
    "Hello world!"
}

fn main() {
    rocket::ignite().mount("/", routes![home, hello]).launch();
}
