export function grey_scale(src: u32, dst: u32, w: u32, h: u32): void {
  const size: u32 = w * h;
  for (let i: u32 = 0; i < size; i++) {
    let pixel: u32 = load<u32>(src + i * 4);
    let red = pixel & 0xff;
    let green = (pixel >> 8) & 0xff;
    let blue = (pixel >> 16) & 0xff;
    let grey = ((red + green + blue) / 3) & 0xff;
    pixel = grey | (grey << 8) | (grey << 16) | (0xff << 24);
    store<u32>(dst + i * 4, pixel);
  }
}
