use std::{fs, path::Path, process::Command};

pub fn resize(path: String, args: String) -> Vec<String> {
  let tmp = Path::new("tmp");
  if tmp.is_dir() {
    fs::remove_dir_all(tmp).unwrap();
  }
  fs::create_dir(tmp).unwrap();

  Command::new("ffmpeg")
    .args(["-i", &path, "-vf", &args, "./tmp/%d.png"])
    .output().unwrap();

  let files = fs::read_dir(tmp)
    .unwrap()
    .filter_map(|n| {
      let path = n.unwrap().path();
      let path = path.to_str().unwrap();
      if path.ends_with(".png") {
        Some(path.to_string())
      } else {
        None
      }
    })
    .collect::<Vec<String>>();

  files
}