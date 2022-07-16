#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

use backend::Backend;

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Skole boller",
            classes: Backend::classes(),
            remarks: Backend::remarks(),
        },
    )
}

#[get("/calendar")]
fn calendar() -> Template {
    Template::render(
        "calendar",
        context! {
            classes: Backend::classes(),
        },
    )
}

#[get("/remarks")]
fn remarks() -> Template {
    Template::render(
        "remarks",
        context! {
            remarks: Backend::remarks(),
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, calendar, remarks])
        .mount("/", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
