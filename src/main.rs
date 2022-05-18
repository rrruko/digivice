#[macro_use] extern crate rocket;

use std::io::Result;
use std::path::Path;
use rocket::http::ContentType;
use rocket::fs::NamedFile;

#[get("/")]
async fn index() -> Result<NamedFile> {
  let path = Path::new("frontend").join("index.html");
  let named_file = NamedFile::open(path).await?;
  Ok(named_file)
}

#[get("/main.js")]
async fn js() -> Result<NamedFile> {
  let path = Path::new("frontend").join("main.js");
  let named_file = NamedFile::open(path).await?;
  Ok(named_file)
}

#[get("/vendor/three.js")]
async fn three() -> Result<NamedFile> {
  let path = Path::new("frontend").join("vendor").join("three.js");
  let named_file = NamedFile::open(path).await?;
  Ok(named_file)
}

#[get("/vendor/GLTFLoader.js")]
async fn gltf_loader() -> Result<NamedFile> {
  let path = Path::new("frontend").join("vendor").join("GLTFLoader.js");
  let named_file = NamedFile::open(path).await?;
  Ok(named_file)
}

#[get("/model/<model_id>")]
async fn get_model(model_id: &str) -> Result<(ContentType, NamedFile)> {
  let path = Path::new("digimons").join(model_id).join("out.gltf");
  let custom = ContentType::new("model", "gtlf+json");
  let named_file = NamedFile::open(path).await?;
  Ok((custom, named_file))
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index, js, three, gltf_loader, get_model])
}
