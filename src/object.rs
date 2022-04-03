use std::{collections::{HashMap, BTreeMap}, vec};

pub struct Frames {
    size: (u16,u16),
    frames: BTreeMap<u32,Vec<FramePixel>>,
    fps: u16,
}

impl Frames {

    fn len(&self)-> u32 {
        self.index.len()
    }

    fn as_frame(&self)-> Frame {

        if self.frames.len() != 1 { None }

        Frame {
            size: self.size,
            pixel: self.frames.get(&0)
        }

    }

    fn concat(&mut self,other: Frames) {

        if self.size != other.size { expect("diff size") }

        self.frames.append(&mut other.frames)

    }

    fn insert (&mut self,index:u32,text:String){

        let res = vec![];
        let mut curr = '\u{0203}';
        let mut len = 0_u32;

        for ch in text.chars() {

            if ch == curr { len += 1 }
            else {
                if len == 1 { res.push(Single(ch)) }
                else if len > 1 { res.push(Multiple(ch,len)) }
                curr = ch;
                len = 1;
            }

        }

        self.frames.insert(index, res);
    }

}



pub struct Frame {
    size: (u16,u16),
    pixel: Vec<FramePixel>,
}



pub enum FramePixel {
    Single(char),
    Multiple(char,u32)
}
