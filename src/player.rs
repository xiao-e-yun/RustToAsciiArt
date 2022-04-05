use std::{
  fs,
  io::{stdout, Write},
  thread::sleep,
  time::Duration,
};

use crate::{
  get_file,
  object::Saved, wait_press,
};

#[test]
fn test() {main()}

pub fn main() {
  let path = get_file("`output.art.json`", Some(".art.json"));

  let data = fs::read(&path).unwrap();

  let saved: Result<Saved, _> = serde_json::from_slice(&data);
  if let Ok(saved) = saved {
    let fps = saved.fps.unwrap_or_default();
    let size = saved.size;
    let frames = saved.unzip();

    println!("Playing {}", &path);
    println!("Size {}x{}", size.0, size.1);
    println!("FPS {}", fps);

    wait_press("Player is ready.","play");
    
    for frame in frames.into_iter() {
      print!("\x1B[2J\x1B[H");
      stdout().flush().unwrap();
      draw(frame.1, size);
      sleep(Duration::from_millis(1000 / fps as u64));
    }
  }
}

fn draw(frame: String, size: (u32, u32)) {
  let (width, height) = size;

  let mut output:Vec<char> = frame.chars().collect();
  for i in 1..height+1 {
    let index = (width * i + i - 1) as usize;
    output.insert(index, '\n');
  }

  output.iter().for_each(|v|print!("{}", v));
  stdout().flush().unwrap();
}
