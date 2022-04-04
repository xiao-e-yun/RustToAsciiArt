use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap};

#[derive(Serialize, Deserialize)]
pub struct Saved {
  pub size: (u32, u32),
  pub fps: Option<u16>,
  pub frames: BTreeMap<u32, String>,
}
impl Saved {
  pub fn concat(&mut self, mut other: Saved) {
    if self.size != other.size {
      panic!("diff size")
    }

    self.frames.append(&mut other.frames)
  }

  pub fn new(fps: Option<u16>) -> Self {
    Self {
      fps,
      size: (0, 0),
      frames: BTreeMap::new(),
    }
  }

  pub fn resize(&mut self, size: (u32, u32)) {
    self.size = size;
  }

  pub fn insert(&mut self, index: u32, text: String) {
    let mut res = String::new();
    let mut curr = '\u{0201}';
    let mut len = 0_u32;

    for ch in text.chars() {
      if ch == curr {
        len += 1
      } else {
        if len == 1 {
          res += &(curr.to_string() + "F");
        } else if len > 1 {
          res += &curr.to_string();
          res += &(len.to_string() + "F");
        }
        curr = ch;
        len = 1;
      }
    }

    if len > 1 {
      res += &curr.to_string();
      res += &len.to_string();
    } else {
      res += &curr.to_string();
    }

    res += "F";

    self.frames.insert(index, res);
  }

  pub fn unzip(self)-> BTreeMap<u32, String> {
    self
      .frames
      .into_iter()
      .map(|(i, fr)| {
        let mut out = String::new();
        let mut chs: Vec<char> = fr.chars().collect();
        chs.reverse();

        loop {
          let ch = match chs.pop() {
            Some(ch) => ch,
            None => break, //END
          };

          let mut length = String::new();
          loop {
            let le = chs.pop().unwrap();
            if le == 'F' {
              break;
            }
            length.push(le)
          }

          let st = ch.to_string();
          if length.is_empty() { out += &st }
          else { out += &st.repeat(length.parse().unwrap())}
        }
        (i, out)
      })
    .collect()
  }
}

#[derive(Clone)]
pub struct Pixel {
  pub lt: u8,
  pub lb: u8,
  pub rt: u8,
  pub rb: u8,
  pub x: u32,
  pub y: u32,
}

impl Pixel {
  pub fn new(xy: (u32, u32)) -> Self {
    Self {
      lt: 0,
      lb: 0,
      rt: 0,
      rb: 0,
      x: xy.0,
      y: xy.1,
    }
  }

  pub fn get(&self, lr: bool) -> (u16, u16) {
    match lr {
      true => (self.lt.into(), self.lb.into()),
      false => (self.rt.into(), self.rb.into()),
    }
  }
}
