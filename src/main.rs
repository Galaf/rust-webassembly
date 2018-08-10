#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::env;
use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    let path = env::current_dir()?;
    println!("cwd = {:?}", path.display());
    println!("Path = {:?}", Path::new("./websrc").as_os_str());
    NamedFile::open("./websrc/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("websrc/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files])
}

fn main() {
    rocket().launch();
}
