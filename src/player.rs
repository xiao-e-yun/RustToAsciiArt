use std::{path::Path, fs, thread::sleep, time::Duration, io::{stdout, Write, stdin}};

use crate::{get_input, object::{Frames, Frame, FramePixel}};

pub fn main() {
  let path = get_input("Path",||{
    println!("File Path:");
    println!("Ex. `output.art.json`");
  },|txt,err|{
    let path = Path::new(&txt);
    if !path.is_file() {
      *err = "Not found";
      return None
    }

    let filename =  path.file_name().unwrap().to_str().unwrap().to_string();
    if filename.ends_with(".art.json") {
      *err = "Incorrect format";
      return None  
    }
    
    Some(txt)
  });

  let data = fs::read(&path).unwrap();



  let frames: Result<Frames,_> = serde_json::from_slice(&data);
  if let Ok(frames) = frames {

    let fps = frames.fps;
    let size = frames.size;

    println!("playing {}",&path);
    println!("size {}x{}",size.0,size.1);
    println!("fps {}",fps);

    print!("press any key to play");
    stdout().flush().unwrap();
    stdin().read_line(&mut String::new()).unwrap();

    for frame in frames.frames {
      sleep(Duration::from_millis(1000 / fps as u64));
      draw(frame.1,size);
    }

  } else { drop(frames) }

  
  let frame: Result<Frame,_> = serde_json::from_slice(&data);
  if let Ok(frame) = frame {
    draw(frame.pixel,frame.size);
  } else { drop(frame) }



}

fn draw(frame:Vec<FramePixel>,size:(u16,u16)){
  let (width,height) = size;

  print!("\x1B[2J\x1B[H");
  stdout().flush().unwrap();

  let mut render = String::new();
  for pixel in frame.into_iter() { render += &pixel.to_string() }

  for _ in 0..width {
    let (out,last) = render.split_at(height as usize);
    println!("{}",out);
    render = last.to_string();        
  }
}