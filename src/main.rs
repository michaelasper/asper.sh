#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
use rocket_contrib::Template;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext { name: "Michael Asper".to_string() };
    Template::render("index", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index]).attach(
        Template::fairing(),
    )
}

fn main() {
    rocket().launch();
}
