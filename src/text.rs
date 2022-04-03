use rusttype::{point, Font, Scale, PositionedGlyph};

#[derive(Clone, Copy, Debug)]
pub struct Text {
    pub t: u8,
    pub b: u8,
    pub txt: char,
}
impl Text {
    pub fn new(txt: char) -> Self {
        Self {
            t: 0,
            b: 0,
            txt,
        }
    }
}

pub fn paser(string: String) -> Vec<Text> {
    let chars:Vec<char> = string.replace("\n", "").replace(" ", "\u{2001}").chars().collect();

    let font = {
        let font_data = include_bytes!("../assets/wqy-microHei.ttf");
        Font::try_from_bytes(font_data as &[u8]).expect("error constructing a Font from bytes")
    };

    let scale = Scale { x: 1., y: 2. };

    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);


    let mut texts = vec![];
    let mut max = 0;

    for ch in chars.into_iter() {
        let glyph = font.glyph(ch).scaled(scale).positioned(offset);
        texts.push(core(ch, glyph,&mut max));
    }

    for text in texts.iter_mut() {
        let size = 255. / max as f32;
        text.t = (text.t as f32 * size) as u8;
        text.b = (text.b as f32 * size) as u8;
    }

    texts
}

fn core(txt: char,glyph: PositionedGlyph, max: &mut u8) -> Text {

    let mut pixel_data = Text::new(txt);


    glyph.draw(|_x,y,v|{

        let i = (v * 255. - 0.5) as u8;
        if i> *max { *max = i }

        match y {
            0 => pixel_data.t += i,
            1 => pixel_data.b += i,
            _ => {}
        }

    });

    pixel_data
}
