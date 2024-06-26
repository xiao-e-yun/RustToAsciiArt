use std::{
  cell::RefCell,
  collections::BTreeMap,
  fs,
  io::{stdout, Write},
  path::Path,
  sync::Arc,
  thread,
};

use image::GenericImageView;

use crate::{
  get_file, get_input,
  object::{Pixel, Saved},
  resize::resize,
  text, wait_press,
};

#[test]
fn test() {
  main()
}

pub fn main() {
  let (path, size, double, fps) = menu();

  let map_thread = thread::spawn(map);

  let mut files = resize(
    path,
    format!("fps={},scale={}", fps, size_mix(size.clone(), double)),
  );
  let mapping = map_thread.join().unwrap();
  println!("Resize complete.");

  let files_len = files.len();
  let is_single = files_len == 1;
  println!(
    "Find {} frame{}",
    files_len,
    if is_single { "" } else { "s" }
  );

  {
    // parser
    let mapping = Arc::new(mapping);
    let mut threads = vec![];

    let cores = num_cpus::get().min(files_len);
    let mut over = files_len % cores;
    let pre = (files_len - over) / cores;

    for core_id in 1..=cores {
      let len = if over == 0 {
        pre
      } else {
        over -= 1;
        pre + 1
      };

      let files = files.split_off(files.len() - len);
      let mapping = Arc::clone(&mapping);
      let size: (u32,u32) = (size.0 * if double { 2 } else { 1 },size.1);

      threads.push(thread::spawn(move || {
        println!("Core-{} started", core_id);

        let mut frames = Saved::new(size, Some(fps));

        for path in files.into_iter() {
          let frame = image::open(&path).unwrap();
          let pixels = frame.pixels();
          let height = frame.height();
          let mut new_pixels = BTreeMap::new();

          for (x, y, rgba) in pixels {
            let [r, g, b, _] = rgba.0;
            let lightness = ((0.299 * r as f32) + (0.587 * g as f32) + (0.114 * b as f32)) as u8;

            let is_right = x % 2 == 1;
            let is_bottom = y % 2 == 1;

            let pos = (get_pos(x, is_right), get_pos(y, is_bottom));
            let id = pos.0 + pos.1 * height;
            let pixel = new_pixels.entry(id).or_insert(Pixel::new(pos));

            match (is_bottom, is_right) {
              (false, false) => pixel.lt = lightness,
              (true, false) => pixel.lb = lightness,
              (false, true) => pixel.rt = lightness,
              (true, true) => pixel.rb = lightness,
            };

            fn get_pos(len: u32, over: bool) -> u32 {
              (len - if over { 1 } else { 0 }) / 2
            }
          }

          let mut out = String::new();
          new_pixels.into_values().for_each(|pixel| {
            for i in 0..2 {
              let mut best = '\u{0203}';

              let mut total_offset = u16::MAX;
              let (t, b) = pixel.get(i == 0);

              (&mapping).iter().for_each(|txt| {
                let mut offset = (t as i32 - txt.t as i32).abs() as u16;
                offset += (b as i32 - txt.b as i32).abs() as u16;

                if &offset >= &total_offset {
                  return;
                }

                total_offset = offset;

                best = txt.txt;
              });

              out.push(best)
            }
          });

          let id: u32 = Path::new(&path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();

          frames.insert(id, out);
        }

        println!("Core-{} done", core_id);
        frames
      }));
    }

    let mut frist = Saved::from(threads.pop().unwrap().join().unwrap());
    threads.into_iter().for_each(|thread| {
      let frames = thread.join().unwrap();
      *&mut frist.concat(frames.into())
    });

    println!("Convert done");
    let data = serde_json::to_string(&frist);
    fs::write("output.art.json", data.unwrap().as_bytes()).unwrap();
    println!("Saved in `output.art.json`");

    wait_press("All done.","back");
  }
}

//
fn map() -> Vec<text::Text> {
  let path = get_input(
    "Mapping",
    || {
      println!("- ffmpeg running background");
      println!("Select mapping file")
    },
    |txt| {
      let path = Path::new(&txt);
      if !path.is_file() {
        return Err("Not found");
      }
      Ok(txt)
    },
  );

  let data = fs::read_to_string(path).unwrap();
  let text = text::paser(data);
  println!("Load map complete.");
  text
}

//
fn menu() -> (String, (u32, u32, String), bool, u16) {
  let path = RefCell::new(get_path());
  let size = RefCell::new(get_size());
  let double = RefCell::new(get_double());
  let fps = RefCell::new(get_fps());

  loop {
    if get_input(
      "Confirm",
      || {
        println!("0.Confirm");
        println!("1.Path: {}", path.borrow());
        println!("2.Size: {}:{} ({})", size.borrow().0, size.borrow().1, size.borrow().2);
        println!("3.Double: {}", double.borrow());
        println!("4.FPS: {}", fps.borrow());
      },
      |txt| {
        if txt.is_empty() {
          return Ok(true);
        }
        let mode = txt.parse::<u16>();
        let mode = if let Ok(mode) = mode {
          mode
        } else {
          return Err("Incorrect value");
        };

        print!("\x1B[2J\x1B[H");
        stdout().flush().unwrap();
        match mode {
          0 => return Ok(true),
          1 => *path.borrow_mut() = get_path(),
          2 => *size.borrow_mut() = get_size(),
          3 => *double.borrow_mut() = get_double(),
          4 => *fps.borrow_mut() = get_fps(),
          _ => return Err("Incorrect value"),
        };
        Ok(false)
      },
    ) {
      break;
    }
  }

  (path.take(), size.take(), double.take(), fps.take())
}

fn get_path() -> String {
  get_file("Allow any `video` and `image`", None)
}

fn get_fps() -> u16 {
  get_input(
    "FPS",
    || {
      println!("Default: 20");
      println!("FPS is int");
      println!("Ex. 12");
    },
    |txt| {
      if txt.is_empty() {
        return Ok(20);
      }
      let int: u16 = txt.parse().unwrap_or(0);
      if int == 0 {
        return Err("Incorrect value");
      }
      Ok(int)
    },
  )
}

fn get_size() -> (u32, u32, String) {
  get_input(
    "Size",
    || {
      println!("{{width}}:{{height}}");
      println!("width, height > 0");
      println!("Ex. 32:18");
    },
    |txt| {
      let mut scale = None;
      for br in [":", "x", "X"] {
        let list: Vec<&str> = txt.split(br).collect();
        if list.len() == 2 {
          scale = Some(list)
        };
      }

      let scale: Vec<&str> = if let Some(v) = scale {
        if v.len() != 2 {
          return Err("Incorrect format");
        }
        v
      } else {
        return Err("Incorrect format");
      };

      let mut parse_err = false;
      let (width, height) = {
        let mut sc: Vec<u32> = scale
          .iter()
          .map(|vec| {
            let val: u32 = vec.parse().unwrap_or(0);
            if val == 0 {
              parse_err = true
            }
            val
          })
          .collect();

        if parse_err {
          return Err("Incorrect int");
        }

        let height = sc.pop().unwrap();
        let width = sc.pop().unwrap();

        (width, height)
      };

      fn gcd(a:u32,b:u32)->String {
        let (mut max,mut min) = if a >= b { (a,b) } else { (b,a) };
        while min != 0 {
    
          let tmax = min;
          min = max % min;
          max = tmax;
    
        }
        (a/max).to_string() + ":" + &(b/max).to_string()
      }

      Ok((width,height,gcd(width,height)))
    },
  )
}

fn get_double() -> bool {
  get_input(
    "Double width",
    || {
      println!("4 pixel           ██");
      println!("Yes. Double width @@ < Default");
      println!("No.  Single width 繭");
      println!("Yes / no (Y,n)");
    },
    |txt| {
      let txt = txt.to_lowercase();
      if txt.is_empty() {
        return Ok(true);
      }
      match &txt[..] {
        "yes" => Ok(true),
        "y" => Ok(true),
        "no" => Ok(false),
        "n" => Ok(false),
        _ => Err("Incorrect format"),
      }
    },
  )
}

fn size_mix(mut size: (u32, u32, String), double: bool) -> String {
  size.1 *= 2;
  if double {
    size.0 *= 2
  }

  size.0.to_string() + ":" + &size.1.to_string()
}
