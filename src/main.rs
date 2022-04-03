use std::io::{Write, stdout, stdin};

mod object;
mod text;
mod player;
mod parse;

fn main() {
  //clear screen
  print!("\x1B[2J\x1B[H");
  stdout().flush().unwrap();

  'menu: loop {
    //send title
    println!("RustToAsciiArt - Menu");
    println!("Mode Select:");
    println!("0.parse");
    println!("1.player");
    println!("2.exit");

    let mut frist = true;
    let mut err = "";
    loop {

      if frist { frist = false }
      else { print!("\x1B[F\x1B[2K"); }

      print!("{}> ",err);
      stdout().flush().unwrap();

      let mut choose = String::new();
      stdin().read_line(&mut choose).unwrap();

      let choose = choose.trim();
      if choose.len() == 0 { continue; }

      let choose = choose.parse::<u8>();
      if choose.is_err() {
        err = "Please send index";
        continue;
      }
      let choose = choose.unwrap();

      let main = match choose {
        0 => parse::main,
        1 => player::main,
        2 => break 'menu,
        _ => {
          err = "Not found";
          continue;
        },
      };

      print!("\x1B[2J\x1B[H");
      stdout().flush().unwrap();
      main();
      break;

    }
  }
}

pub fn get_input<T>(title:&str,pre:impl Fn() -> (),after:impl Fn(String, &mut &str) -> Option<T>)-> T {
  let mut err = "";
  loop {
    println!("RustToAsciiArt - {}",title);
    pre();
    
    let mut frist = true;
    loop {
      if frist { frist = false }
      else { print!("\x1B[F\x1B[2K"); }

      let mut input = String::new();
      print!("{}> ",err);
      stdout().flush().unwrap();
      stdin().read_line(&mut input).unwrap();

      if let Some(res) = after(input,&mut err) { return  res; }
    }
  }
}

    // if TEST { println!(" 使用測試模式，自動控制")}
    // let file = loop {
    //     if TEST { break String::from("data/badApple.mp4")}
    //     println!("輸入影片(僅限mp4)");
    //     let input = get_input("檔案位置");
    //     if !input.ends_with(".mp4") {
    //         println!("錯誤格式");
    //         continue;
    //     }

    //     if !Path::new(&input).is_file() {
    //         println!("找不到檔案");
    //         continue;
    //     }
    //     break input
    // };

    // let fps = loop {
    //     if TEST { break 20 }
    //     let input = get_input("FPS");
    //     if input.is_empty() { break 20 }
    //     let o = input.parse::<u8>().unwrap_or(0);
    //     if o != 0 { break o }
    // };

    // let scale = format!("scale={}",loop {
    //     if TEST { break String::from("64:36") }
    //     println!("格式 {{width}}x{{height}}");
    //     let input = get_input("大小");
    //     if input.is_empty() { break String::from("32:18") }

    //     let mut type_err = false;
    //     let mut sc:Vec<String> = input.trim().split("x").map(|vec|{
    //         let mut val = vec.parse::<i16>().unwrap_or(-2);
    //         if val > 0 { val *= 2 }
    //         else if val != -1 { type_err = true }
    //         val.to_string()
    //     }).collect();

    //     if type_err || sc.len()!=2 { continue; }

    //     println!("使用寬度雙倍(1pixel = 2char)");
    //     let input = if TEST { String::new() } else { get_input("Y/n") };
    //     if input == "n" {
    //         let width = sc.get_mut(0).unwrap();
    //         let iwidth:i16 = width.parse().unwrap();
    //         if iwidth != -1 { *width = (iwidth / 2).to_string() }
    //     }

    //     break sc.join(":").to_string()
    // });

    // let tmp = Path::new("./tmp");
    // if tmp.exists() {
    //     println!("清空緩存");
    //     if tmp.is_file() { fs::remove_file("./tmp").unwrap() }
    //     else { fs::remove_dir_all("./tmp").unwrap() }
    // }
    // fs::create_dir("./tmp").unwrap();

    // let path = loop {
    //     if TEST { break String::from("map.txt") }
    //     println!("輸入映射表(文字)");
    //     let input = get_input("檔案位置");
    //     if !Path::new(&input).is_file() {
    //         println!("找不到檔案");
    //         continue;
    //     }

    //     break input
    // };

    // let mapping = thread::spawn(move||->Vec<text::Text>{

    //     let words = text::paser(fs::read_to_string(path).unwrap());
    //     println!("映射表完成");
    //     words

    // });

    // let args = format!("fps={}",&fps) + "," + &scale[..];
    // Command::new("ffmpeg")
    //     .args([
    //         "-i",
    //         &file[..],
    //         "-vf",
    //         &args[..],
    //         "./tmp/%d.png"
    //     ])
    //     .output().expect("no ffmpeg");

    // println!("切分圖片完成");

    // let mut frames = fs::read_dir("./tmp").unwrap()
    //     .filter_map(|n|{
    //         let path = n.unwrap().path();
    //         let path = path.to_str().unwrap();
    //         if path.ends_with(".png") { Some(path.to_string()) } else { None }
    //     }).collect::<Vec<String>>();

    // let frames_len = frames.len();

    // println!("總共{}幀",frames_len);

    // let mut threads = vec![];

    // let cores = num_cpus::get();
    // let mut over = frames_len % cores;
    // let pre = ( frames_len - over ) / cores;

    // println!("創建線程{}",cores);

    // let mapping = Arc::new(mapping.join().unwrap());
    // for core in 0..cores {

    //     let frames = frames.split_off(frames.len() - if over == 0 { pre } else {
    //         over -= 1;
    //         pre + 1
    //     });

    //     let mapping = Arc::clone(&mapping);
    //     threads.push(thread::spawn(move||{

    //         println!("core {} start",core);

    //         let mut done = vec![];
    //         for frame in frames.iter() {

    //             let frame = &frame[..];
    //             let img = image::open(frame).unwrap();
    //             let index:u32 = Path::new(frame).file_stem().unwrap().to_str().unwrap().parse().unwrap();

    //             let height = img.height();
    //             let pixels = img.pixels();
    //             let mut new_pixels = BTreeMap::new();

    //             for (x, y, rgba) in pixels {
    //                 let [ r, g, b ,_ ] = rgba.0;

    //                 let lightness = ( (0.299 * r as f32) + (0.587 * g as f32) + (0.114 * b as f32) ) as u8;

    //                 let is_right = x % 2 == 1;
    //                 let is_bottom = y % 2 == 1;

    //                 let pos = (get_pos(x,is_right),get_pos(y, is_bottom));
    //                 let id = pos.0 + pos.1 * height;
    //                 let pixel = new_pixels.entry(id).or_insert(Pixel::new(pos));

    //                 match (is_bottom, is_right) {
    //                     (false,false) => pixel.lt = lightness,
    //                     (true,false) => pixel.lb = lightness,
    //                     (false,true) => pixel.rt = lightness,
    //                     (true,true) => pixel.rb = lightness,
    //                 };


    //                 fn get_pos(len: u32, over: bool)-> u32 {
    //                     ( len - if over { 1 } else { 0 } ) / 2
    //                 }
    //             };

    //             let mut frame: Vec<(u32,u32,String)> = new_pixels.into_values().map(|pixel|{

    //                 let mut ch = String::new();

    //                 for i in 0..2 {
    //                     let mut best = '\u{0203}';

    //                     let mut total_offset = u16::MAX;
    //                     let (t,b) = pixel.get(i == 0);

    //                     (&mapping).iter().for_each(|txt|{
    //                         let mut offset = (t as i32 - txt.t as i32).abs() as u16;
    //                         offset += (b as i32 - txt.b as i32).abs() as u16;

    //                         if &offset >= &total_offset { return }

    //                         total_offset = offset;

    //                         best = txt.txt;
    //                     });

    //                     ch.push(best)
    //                 }

    //                 let (x, y) = (pixel.x,pixel.y);
    //                 (x,y,ch)

    //             }).collect();

    //             frame.sort_by(|a,b|if a.1 == b.1 { a.0.cmp(&b.0) } else { a.1.cmp(&b.1) });
    //             done.push((index,frame));

    //         }

    //         println!("core {} done",core);
    //         done

    //     }));

    // }

    // let mut frames = threads.into_iter()
    //     .map(|t|t.join().unwrap())
    //     .collect::<Vec<_>>().concat();

    // frames.sort_by(|a,b|a.0.cmp(&b.0));

    // println!("轉譯完成");

    // let mut play = String::new();
    // println!("1.播放");
    // println!("2.保存(默認)");
    // print!("選擇 >");
    // std::io::stdout().flush().unwrap();
    // if !TEST { std::io::stdin().read_line(&mut play).unwrap(); }

    // let play: bool = play.trim() == "1";

    // let file = if play { None } else {
    //     let path = "output.txt";
    //     if Path::new(path).is_file() {
    //         fs::remove_file(path).unwrap();
    //     }
    //     Some(File::create(path).unwrap())
    // };

    // let mut buffer = vec![];

    // for frame in frames.iter() {
    //     let mut out = String::new();
    //     if play { sleep(Duration::from_millis(1000 / fps as u64)) };
    //     out += if play { "\n\n\n\n\n\n\n\n\n" } else { "\nEOF\n" };
    //     for pixel in frame.1.iter() {
    //         if pixel.1 != 0 && pixel.0 == 0 { out += "\n"; }
    //         out += &pixel.2[..];
    //     }
    //     if play {
    //         let mut stdout = stdout();
    //         stdout.write(out.as_bytes()).unwrap();
    //         stdout.flush().unwrap();
    //     } else {
    //         buffer.push(out);
    //     }
    // };

    // if let Some(mut file) = file {
    //     file.write(buffer.concat().as_bytes()).unwrap();
    //     file.sync_all().unwrap();
    //     println!("done");
    // }

#[derive(Clone)]
struct Pixel {
    lt: u8,
    lb: u8,
    rt: u8,
    rb: u8,
    x: u32,
    y: u32,
}

impl Pixel {
    pub fn new(xy:(u32,u32)) -> Self {
        Self {
            lt: 0,
            lb: 0,
            rt: 0,
            rb: 0,
            x: xy.0,
            y: xy.1
        }
    }

    pub fn get(&self,lr: bool) -> (u16, u16) {
        match lr {
            true => (self.lt.into(),self.lb.into()),
            false => (self.rt.into(),self.rb.into()),
        }
    }
}
