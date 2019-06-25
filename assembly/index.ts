export function grey_scale(src: u32, dst: u32, w: u32, h: u32): void {
  const size: u32 = w * h;
  for (let i: u32 = 0; i < w; i++) {
    for (let j: u32 = 0; j < h; j++) {
      let pixel: u32 = load<u32>(src + ((i * w * 4) + (j * 4)));
      let red = pixel & 0xff;
      let green = (pixel >> 8) & 0xff;
      let blue = (pixel >> 16) & 0xff;
      let grey = ((red + green + blue) / 3) & 0xff;
      pixel = 0 | (grey << 8) | (0 << 16) | (0xff << 24);
      store<u32>(dst + ((i * w * 4) + (j * 4)), pixel);
    }
  }
}
