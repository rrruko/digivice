#[macro_use] extern crate rocket;

use std::result::Result;
use std::io::Result as IOResult;
use std::path::{Path, PathBuf};
use rocket::http::ContentType;
use rocket::fs::NamedFile;
use rocket::Rocket;
use rocket::Build;
use rocket::State;
use rocket::fs::FileServer;
use std::fs;

#[get("/model/<ix>")]
async fn get_model(ix: usize, models: &State<ModelList>, digimons_path: &State<PathBuf>) -> IOResult<(ContentType, NamedFile)> {
  let model_id = &models.models[ix % models.models.len()];
  let path = digimons_path.join(model_id).join("out.gltf");
  let custom = ContentType::new("model", "gtlf+json");
  let named_file = NamedFile::open(path).await?;
  Ok((custom, named_file))
}

fn list_digimons<'a>(digimons_path: &Path) -> Result<ModelList, String> {
  let read_dir = fs::read_dir(digimons_path)
    .map_err(|e| format!("{:?}", e))?;
  let mut models = read_dir
    .map(|x| {
       match x {
         Ok(entry) => entry
           .file_name()
           .into_string()
           .map_err(|_| String::from("Couldn't convert OsString to String")),
         Err(bad) => Err(format!("{:?}", bad))
       }
    })
    .collect::<Result<Vec<String>, String>>()?;
  models.sort();
  Ok(ModelList { models })
}

struct ModelList {
  models: Vec<String>
}

fn launch_app(digimons_path: &Path, frontend_path: &Path) -> Rocket<Build> {
  match list_digimons(digimons_path) {
    Ok(models) =>
      rocket::build()
        .mount("/", routes![get_model])
        .mount("/", FileServer::from(frontend_path))
        .manage(models)
        .manage(digimons_path.to_path_buf()),
    Err(error) =>
      panic!("Failed to enumerate models: {}", error)
  }
}

#[launch]
fn rocket() -> _ {
  let args: Vec<String> = std::env::args().collect();
  match &args[..] {
    [_, digimons_path, frontend_path] =>
      launch_app(
        Path::new(&digimons_path),
        Path::new(&frontend_path)),
    _ => panic!("Usage: digivice <DIGIMONS_PATH> <FRONTEND_PATH>")
  }
}
