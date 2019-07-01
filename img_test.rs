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
            let mut kern: [u32; 9] = [0;9];
            let mut off = 0;
            for r in i-1..=i+1 {
                for c in j-1..=j+1 {
                    kern[off] = src_img[ r*w + c];
                    off += 1;
                }
            }

            let mut rsum = 0;
            let mut gsum = 0;
            let mut bsum = 0;
            for v in &kern {
                rsum += v & 0xff;
                gsum += (v >> 8 as u32) & 0xff;
                bsum += (v >> 16 as u32) & 0xff;
            }

            rsum = rsum / (kern.len() as u32);
            gsum = gsum / (kern.len() as u32);
            bsum = bsum / (kern.len() as u32);
            dst_img[(i * w) + j] = rsum | (gsum<<8) | (bsum<<16) | (0xff<<24);
        }
    }
}