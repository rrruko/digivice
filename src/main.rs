#[macro_use] extern crate rocket;

use std::io::Result;
use std::path::Path;
use rocket::http::ContentType;
use rocket::fs::NamedFile;

#[get("/model/<model_id>")]
async fn index(model_id: &str) -> Result<(ContentType, NamedFile)> {
  let path = Path::new("digimons").join(model_id).join("out.gltf");
  let custom = ContentType::new("model", "gtlf+json");
  let named_file = NamedFile::open(path).await?;
  Ok((custom, named_file))
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index])
}
