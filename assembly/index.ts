export function grey_scale(src: u32, dst: u32, w: u32, h: u32): void {
  for (let i: u32 = 0; i < w; i++) {
    for (let j: u32 = 0; j < h; j++) {
      let pixel1: u32 = load<u32>(src + (((i - 1) * w * 4) + ((j - 1) * 4)));
      let pixel2: u32 = load<u32>(src + (((i - 1) * w * 4) + ((j * 4))));
      let pixel3: u32 = load<u32>(src + (((i - 1) * w * 4) + ((j + 1) * 4)));
      let pixel4: u32 = load<u32>(src + ((i * w * 4) + ((j - 1) * 4)));
      let pixel5: u32 = load<u32>(src + ((i * w * 4) + (j * 4)));
      let pixel6: u32 = load<u32>(src + ((i * w * 4) + ((j + 1) * 4)));
      let pixel7: u32 = load<u32>(src + (((i + 1) * w * 4) + ((j - 1) * 4)));
      let pixel8: u32 = load<u32>(src + (((i + 1) * w * 4) + (j * 4)));
      let pixel9: u32 = load<u32>(src + (((i + 1) * w * 4) + ((j + 1) * 4)));

      let redSum = 0;
      redSum += pixel1 & 0xff;
      redSum += pixel2 & 0xff;
      redSum += pixel3 & 0xff;
      redSum += pixel4 & 0xff;
      redSum += pixel5 & 0xff;
      redSum += pixel6 & 0xff;
      redSum += pixel7 & 0xff;
      redSum += pixel8 & 0xff;
      redSum += pixel9 & 0xff;
      redSum = <i32>Math.floor(redSum / 9);

      let greenSum = 0;
      greenSum += (pixel1 >> 8) & 0xff;
      greenSum += (pixel2 >> 8) & 0xff;
      greenSum += (pixel3 >> 8) & 0xff;
      greenSum += (pixel4 >> 8) & 0xff;
      greenSum += (pixel5 >> 8) & 0xff;
      greenSum += (pixel6 >> 8) & 0xff;
      greenSum += (pixel7 >> 8) & 0xff;
      greenSum += (pixel8 >> 8) & 0xff;
      greenSum += (pixel9 >> 8) & 0xff;
      greenSum = <i32>Math.floor(greenSum / 9);


      let blueSum = 0;
      blueSum += (pixel1 >> 16) & 0xff;
      blueSum += (pixel2 >> 16) & 0xff;
      blueSum += (pixel3 >> 16) & 0xff;
      blueSum += (pixel4 >> 16) & 0xff;
      blueSum += (pixel5 >> 16) & 0xff;
      blueSum += (pixel6 >> 16) & 0xff;
      blueSum += (pixel7 >> 16) & 0xff;
      blueSum += (pixel8 >> 16) & 0xff;
      blueSum += (pixel9 >> 16) & 0xff;
      blueSum = <i32>Math.floor(blueSum / 9);

      let red = pixel5 & 0xff;
      let green = (pixel5 >> 8) & 0xff;
      let blue = (pixel5 >> 16) & 0xff;
      pixel5 = redSum | (greenSum << 8) | (blueSum << 16) | (0xff << 24);
      store<u32>(dst + ((i * w * 4) + (j * 4)), pixel5);
    }
  }
}
