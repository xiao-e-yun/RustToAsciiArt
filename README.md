# RustToAsciiArt
## From `Video` To `AsciiArt`.
Mini and Quick.  
- Allow custom mapping.
- High performance.
- Easy to anywhere.

∆. Need Install `ffmpeg` in environment

[Example](https://static.xiaoeyun.me/rust-to-ascii-art/)
Implement the player for js
```JS
const data = JSON.parse("OUTPUT.art.json")

const frames = data.frames
const [width, height] = data.size

let i = 1;
setInterval(() => {
  const frame = frames[i++].split("")
  let output = ""

  for (;;) {
    const ch = frame.shift()
    if (ch === undefined) break

    let length = ""
    for (;;) {
      let le = frame.shift()
      if (le == "F") break;
      length += le
    }

    if (length === "") output += ch
    else output += ch.repeat(parseInt(length))
  }

  let l = height
  while (l--) {
    output = insert(output, width * 2 * l, "\n");
  }
  el.innerHTML = output;
}, 50);

function insert(o, i, s) {
  return o.slice(0, i) + s + o.slice(i);
}
```
