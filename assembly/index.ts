export function grey_scale(src: u32, dst: u32, w: u32, h: u32): void {
  for (let i: u32 = 0; i < w; i++) {
    for (let j: u32 = 0; j < h; j++) {
      let kern = new Uint32Array(3 * 3);
      let off: u32 = 0;
      for (let r: u32 = i - 1; r <= i + 1; r++) {
        for (let c: u32 = j - 1; c <= j + 1; c++) {
          kern[off] = load<u32>(src + ((r * w * 4) + (c * 4)));;
          off++;
        }
      }

      let rsum = 0, gsum = 0, bsum = 0;
      for (let r: i32 = 0; r < kern.length; r++) {
        rsum += kern[r] & 0xff;
        gsum += (kern[r] >> 8) & 0xff;
        bsum += (kern[r] >> 16) & 0xff;
      }

      rsum = (rsum | 0) / 9;
      gsum = (gsum | 0) / 9;
      bsum = (bsum | 0) / 9;
      store<u32>(dst + ((i * w * 4) + (j * 4)), rsum | (gsum << 8) | (bsum << 16) | (0xff << 24));
    }
  }
}
