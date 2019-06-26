#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::slice::from_raw_parts_mut;
use core::slice::from_raw_parts;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
fn grey_scale( src: usize, dst : usize, w : usize, h :usize ) {
    let src_img: &[u32];
    let dst_img: &mut [u32];
    let size =  w*h;

    unsafe {
        src_img = from_raw_parts::<u32>(src as *const u32, size);
        dst_img = from_raw_parts_mut::<u32>(dst as *mut u32, size);
    }
    
    for i in 0..w { 
        for j in 0..h { 
            let pixel1 = src_img[((i - 1) * w) + (j - 1)];
            let pixel2 = src_img[((i - 1) * w) + j];
            let pixel3 = src_img[((i - 1) * w) + (j + 1)];
            let pixel4 = src_img[(i * w) + (j - 1)];
            let pixel5 = src_img[(i * w) + j];
            let pixel6 = src_img[(i * w) + (j + 1)];
            let pixel7 = src_img[((i + 1) * w) + (j - 1)];
            let pixel8 = src_img[((i + 1) * w) + j];
            let pixel9 = src_img[((i + 1) * w) + (j + 1)];

            let mut red_sum = 0;
            red_sum += pixel1 & 0xff;
            red_sum += pixel2 & 0xff;
            red_sum += pixel3 & 0xff;
            red_sum += pixel4 & 0xff;
            red_sum += pixel5 & 0xff;
            red_sum += pixel6 & 0xff;
            red_sum += pixel7 & 0xff;
            red_sum += pixel8 & 0xff;
            red_sum += pixel9 & 0xff;
            red_sum = red_sum / 9;

            let mut green_sum = 0;
            green_sum += (pixel1 >> 8) & 0xff;
            green_sum += (pixel2 >> 8) & 0xff;
            green_sum += (pixel3 >> 8) & 0xff;
            green_sum += (pixel4 >> 8) & 0xff;
            green_sum += (pixel5 >> 8) & 0xff;
            green_sum += (pixel6 >> 8) & 0xff;
            green_sum += (pixel7 >> 8) & 0xff;
            green_sum += (pixel8 >> 8) & 0xff;
            green_sum += (pixel9 >> 8) & 0xff;
            green_sum = green_sum / 9;

            let mut blue_sum = 0;
            blue_sum += (pixel1 >> 16) & 0xff;
            blue_sum += (pixel2 >> 16) & 0xff;
            blue_sum += (pixel3 >> 16) & 0xff;
            blue_sum += (pixel4 >> 16) & 0xff;
            blue_sum += (pixel5 >> 16) & 0xff;
            blue_sum += (pixel6 >> 16) & 0xff;
            blue_sum += (pixel7 >> 16) & 0xff;
            blue_sum += (pixel8 >> 16) & 0xff;
            blue_sum += (pixel9 >> 16) & 0xff;
            blue_sum = blue_sum / 9;

            let red = pixel1 & 0xff;
            let green = (pixel1 >> 8) & 0xff;
            let blue = (pixel1 >> 16) & 0xff;
            dst_img[(i * w) + j] = red_sum | (green_sum<<8) | (blue_sum<<16) | (0xff<<24);
        }
    }
}