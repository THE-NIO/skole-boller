#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hei, verden!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
