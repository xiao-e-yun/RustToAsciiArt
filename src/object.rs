use std::{collections::BTreeMap, vec};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Frames {
    pub size: (u16,u16),
    pub frames: BTreeMap<u32,Vec<FramePixel>>,
    pub fps: u16,
}

impl Frames {

    pub fn len(&self)-> u32 {
        self.frames.len() as u32
    }

    pub fn as_frame(mut self)-> Option<Frame> {

        if self.frames.len() != 1 { return None; }

        Some(Frame {
          size: self.size,
          pixel: self.frames.remove(&0).unwrap()
        })

    }

    pub fn concat(&mut self,mut other:Frames) {

        if self.size != other.size { panic!("diff size") }

        self.frames.append(&mut other.frames)

    }

    pub fn insert(&mut self,index:u32,text:String){

        let mut res = vec![];
        let mut curr = '\u{0203}';
        let mut len = 0_u32;

        for ch in text.chars() {

            if ch == curr { len += 1 }
            else {
                if len == 1 { res.push(FramePixel::Single(ch)) }
                else if len > 1 { res.push(FramePixel::Multiple(ch,len)) }
                curr = ch;
                len = 1;
            }

        }

        self.frames.insert(index, res);
    }

}



#[derive(Serialize, Deserialize)]
pub struct Frame {
    pub size: (u16,u16),
    pub pixel: Vec<FramePixel>,
}



#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FramePixel {
    Single(char),
    Multiple(char,u32)
}
impl FramePixel {
  pub fn to_string(self)-> String {
    match self {
      Self::Single(v) => v.to_string(),
      Self::Multiple(v, l) => v.to_string().repeat(l as usize),
    }
  }
}