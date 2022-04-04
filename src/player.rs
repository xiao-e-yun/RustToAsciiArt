use std::{
  fs,
  io::{stdin, stdout, Write},
  thread::sleep,
  time::Duration,
};

use crate::{
  get_file,
  object::Saved,
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

    print!("\nPress any key to play");
    stdout().flush().unwrap();
    stdin().read_line(&mut String::new()).unwrap();
    
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
  let width = width * 2; // !TEST

  let mut output:Vec<char> = frame.chars().collect();
  for i in 1..height+1 {
    let index = (width * i + i - 1) as usize;
    output.insert(index, '\n');
  }

  output.iter().for_each(|v|print!("{}", v));
  stdout().flush().unwrap();
}
