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
    
    for i in 0..size {
        let pixel = src_img[i];
        let red = pixel & 0xff;
        let green = (pixel>>8) & 0xff;
        let blue = (pixel>>16) & 0xff;
        let grey = ((red+green+blue) / 3) & 0xff;
        dst_img[i] = grey | (grey<<8) | (grey<<16) | (0xff<<24);
    }
}