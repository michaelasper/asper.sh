#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;

use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    rocket().launch();
}
