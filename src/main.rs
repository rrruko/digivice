#[macro_use] extern crate rocket;

use std::result::Result;
use std::io::Result as IOResult;
use std::path::Path;
use rocket::http::ContentType;
use rocket::fs::NamedFile;
use rocket::State;
use std::fs;

#[get("/")]
async fn index() -> IOResult<NamedFile> {
  let path = Path::new("frontend").join("index.html");
  let named_file = NamedFile::open(path).await?;
  Ok(named_file)
}

#[get("/main.js")]
async fn js() -> IOResult<NamedFile> {
  let path = Path::new("frontend").join("main.js");
  let named_file = NamedFile::open(path).await?;
  Ok(named_file)
}

#[get("/vendor/three.js")]
async fn three() -> IOResult<NamedFile> {
  let path = Path::new("frontend").join("vendor").join("three.js");
  let named_file = NamedFile::open(path).await?;
  Ok(named_file)
}

#[get("/vendor/GLTFLoader.js")]
async fn gltf_loader() -> IOResult<NamedFile> {
  let path = Path::new("frontend").join("vendor").join("GLTFLoader.js");
  let named_file = NamedFile::open(path).await?;
  Ok(named_file)
}

#[get("/model/<ix>")]
async fn get_model(ix: usize, models: &State<ModelList>) -> IOResult<(ContentType, NamedFile)> {
  let model_id = &models.models[ix % models.models.len()];
  let path = Path::new("digimons").join(model_id).join("out.gltf");
  let custom = ContentType::new("model", "gtlf+json");
  let named_file = NamedFile::open(path).await?;
  Ok((custom, named_file))
}

fn list_digimons<'a>() -> Result<ModelList, String> {
  let read_dir = fs::read_dir(Path::new("digimons"))
    .map_err(|e| format!("{:?}", e))?;
  read_dir
    .map(|x| {
       match x {
         Ok(entry) => entry
           .file_name()
           .into_string()
           .map_err(|_| String::from("Couldn't convert OsString to String")),
         Err(bad) => Err(format!("{:?}", bad))
       }
    })
    .collect::<Result<Vec<String>, String>>()
    .map(|models| ModelList { models })
}

struct ModelList {
  models: Vec<String>
}

#[launch]
fn rocket() -> _ {
  match list_digimons() {
    Ok(models) =>
      rocket::build()
        .mount("/", routes![index, js, three, gltf_loader, get_model])
        .manage(models),
    Err(error) =>
      panic!("Failed to enumerate models: {}", error)
  }
}
