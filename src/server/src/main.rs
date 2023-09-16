#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { field: "value" })
}

#[launch]
async fn rocket() -> _ {
    server::services::wss::start_server("127.0.0.1", 3000);
    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/", routes![hello, index])
}
