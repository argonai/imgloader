use rocket::fs::{TempFile, FileServer, relative};
use rocket::form::Form;
#[macro_use] extern crate rocket;
// #[macro_use] extern crate serde_json;
// static mut name: &str = "World";

#[derive(FromForm)]
struct Upload<'r> {
    save : bool,
    file: TempFile<'r>,
}
//  persist to needs a file not directory
//  write function to determine name
#[post("/upload",format = "multipart/form-data", data="<upload>")]
async fn upload_form(mut upload: Form<Upload<'_>>) -> std::io::Result<()> {
    upload.file.persist_to("C:\\Users\\aaron\\Desktop\\temp\\file.png").await?;
    Ok(())
}

#[get("/")]
fn index() -> String {
    let name = "world";
    format!("Hello, {}!", name)
    
}

#[post("/", data="<name>")]
fn create_index(name : String) -> String {
    print!("{}", name);
    format!("hello, {}", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/" , routes![index, create_index, upload_form])
    .mount("/public", FileServer::from(relative!("/static/images")))
}