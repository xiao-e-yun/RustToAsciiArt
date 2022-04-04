use std::{
  io::{stdin, stdout, Write},
  path::Path,
};

mod object;
mod resize;
mod player;
mod parse;
mod text;

fn main() {
  loop {
    //clear screen
    print!("\x1B[2J\x1B[H");
    stdout().flush().unwrap();

    let quit = get_input("Menu",||{
      println!("Mode Select:");
      println!("0.parse");
      println!("1.player");
      println!("2.exit");
    },|select|{
      if select.is_empty() { return Err("") }

      let select = select.parse::<u8>();
      if select.is_err() { return Err("Incorrect format") }

      let main = match select.unwrap() {
        0 => parse::main,
        1 => player::main,
        2 => (return Ok(true)),
        _ => (return Err("Not found")),
      };
      print!("\x1B[2J\x1B[H");
      stdout().flush().unwrap();
      main();
      Ok(false)
    });

    if quit { break; }
  }

}

pub fn get_input<T>(
  title: &str,
  pre: impl Fn() -> (),
  after: impl Fn(String) -> Result<T, &'static str>,
) -> T {
  let mut err = "";
  loop {
    println!("RustToAsciiArt - {}", title);
    pre();

    let mut frist = true;
    loop {
      if frist {
        frist = false
      } else {
        print!("\x1B[F\x1B[2K");
      }

      let mut input = String::new();
      print!("{}> ", err);
      stdout().flush().unwrap();
      stdin().read_line(&mut input).unwrap();

      match after(input.trim().to_string()) {
        Err(e) => err = e,
        Ok(value) => {
          print!("\x1B[2J\x1B[H");
          stdout().flush().unwrap();
          return value;
        }
      }
    }
  }
}

pub fn get_file(example: &str, ends_with: Option<&str>) -> String {
  get_input(
    "Path",
    || {
      println!("File Path:");
      println!("Ex. {}", example);
    },
    |txt| {
      let path = Path::new(&txt);
      if !path.is_file() {
        return Err("Not found");
      }

      if ends_with.is_none() {
        return Ok(txt);
      }

      let filename = path.file_name().unwrap().to_str().unwrap().to_string();

      if !filename.ends_with(ends_with.unwrap()) {
        return Err("Incorrect format");
      }

      Ok(txt)
    },
  )
}