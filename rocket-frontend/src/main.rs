#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

fn get_classes() -> Vec<String> {
    vec!["math", "science"]
        .into_iter()
        .map(ToString::to_string)
        .collect()
}

fn get_remarks() -> Vec<String> {
    vec!["programmering i norsk timen"]
        .into_iter()
        .map(ToString::to_string)
        .collect()
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            title: "Skole boller",
            classes: get_classes(),
            remarks: get_remarks(),
        },
    )
}

#[get("/calendar")]
fn calendar() -> Template {
    Template::render(
        "calendar",
        context! {
            classes: get_classes(),
        },
    )
}

#[get("/remarks")]
fn remarks() -> Template {
    Template::render(
        "remarks",
        context! {
            remarks: get_remarks(),
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
