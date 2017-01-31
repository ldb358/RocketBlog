use rocket::response::NamedFile;
use rocket::request::Form;
use rocket::response::Redirect;

use std::io;
use std::path::{Path, PathBuf};

#[derive(FromForm)]
struct UserLogin<'r> {
    username: &'r str,
    password: String
}


#[get("/")]
fn index() -> io::Result<NamedFile> {
        NamedFile::open("static/index.html")
}

#[get("/static/<file..>", rank = 5)]
fn files(file: PathBuf) -> io::Result<NamedFile> {
        NamedFile::open(Path::new("static/").join(file))
}

#[post("/login", data = "<user_form>")]
fn login<'a>(user_form: Form<'a, UserLogin<'a>>) -> Result<Redirect, String> {
    let user = user_form.get();

    if user.username == "test" {
        match user.password.as_str() {
            "test" => Ok(Redirect::to("/user/test")),
            _ => Err("Wrong password!".to_string())
        }
    } else {
        Err(format!("Unrecognized user, '{}'.", user.username))
    }
}

#[get("/user/<username>")]
fn user_page(username: &str) -> String {
    format!("Welcome {}!", username)
}
