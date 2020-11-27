#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use std::path::{PathBuf, Path};

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![files]).launch();
}