#![no_std]
#![no_main]
#![feature(asm)]


#[no_mangle]
pub extern "C" fn _start() -> ! {
    boot_main();
    unsafe {
        asm!("hlt");
    }
    loop {}
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


fn boot_main() {
    let hello = "hello world";
    let vga_addr = 0xB8000;
    let vga_ptr = vga_addr as *mut u8;
    unsafe {
        for (i, c) in hello.bytes().enumerate() {
            *(vga_ptr.offset(i as isize * 2)) = c;
            *(vga_ptr.offset(i as isize * 2 + 1)) = 0xb;
        }
    }


}

fn sw(){
    mod_switch(g_320x200x256);

    let vga_graphic_addr = 0xA0000;
    // let vga_graphic_addr = get_fb_seg();
    let vga_graphic_ptr = vga_graphic_addr as *mut u8;
    loop {
        unsafe {
            for i in 0..(320 * 200) {
                *(vga_graphic_ptr.offset(i as isize * 2)) = i as u8;
            }
        }
    }
}

fn get_fb_seg() -> u32 {
    outport(VGA_GC_INDEX, 6);
    let mut seg = inport(VGA_GC_DATA);
    seg >>= 2;
    seg &= 3;
    match seg {
        0 | 1 => 0xA0000,
        2 => 0xB0000,
        3 => 0xB8000,
        _ => 0
    }
}

const g_640x480x2: [u8; 61] = [
    /* MISC */
    0xE3,
    /* SEQ */
    0x03, 0x01, 0x0F, 0x00, 0x06,
    /* CRTC */
    0x5F, 0x4F, 0x50, 0x82, 0x54, 0x80, 0x0B, 0x3E,
    0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xEA, 0x0C, 0xDF, 0x28, 0x00, 0xE7, 0x04, 0xE3,
    0xFF,
    /* GC */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x0F,
    0xFF,
    /* AC */
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x14, 0x07,
    0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
    0x01, 0x00, 0x0F, 0x00, 0x00
];

const g_320x200x256: [u8; 61] = [
    /* MISC */
    0x63,
    /* SEQ */
    0x03, 0x01, 0x0F, 0x00, 0x0E,
    /* CRTC */
    0x5F, 0x4F, 0x50, 0x82, 0x54, 0x80, 0xBF, 0x1F,
    0x00, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x9C, 0x0E, 0x8F, 0x28,	0x40, 0x96, 0xB9, 0xA3,
    0xFF,
    /* GC */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x05, 0x0F,
    0xFF,
    /* AC */
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x41, 0x00, 0x0F, 0x00,	0x00
];

const g_320x200x4: [u8; 61] = [
    /* MISC */
    0x63,
    /* SEQ */
    0x03, 0x09, 0x03, 0x00, 0x02,
    /* CRTC */
    0x2D, 0x27, 0x28, 0x90, 0x2B, 0x80, 0xBF, 0x1F,
    0x00, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x9C, 0x0E, 0x8F, 0x14, 0x00, 0x96, 0xB9, 0xA3,
    0xFF,
    /* GC */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x02, 0x00,
    0xFF,
    /* AC */
    0x00, 0x13, 0x15, 0x17, 0x02, 0x04, 0x06, 0x07,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x01, 0x00, 0x03, 0x00, 0x00
];

const VGA_AC_INDEX: u32 = 0x3C0;
const VGA_AC_WRITE: u32 = 0x3C0;
const VGA_AC_READ: u32 = 0x3C1;
const VGA_MISC_WRITE: u32 = 0x3C2;
const VGA_SEQ_INDEX: u32 = 0x3C4;
const VGA_SEQ_DATA: u32 = 0x3C5;
const VGA_DAC_READ_INDEX: u32 = 0x3C7;
const VGA_DAC_WRITE_INDEX: u32 = 0x3C8;
const VGA_DAC_DATA: u32 = 0x3C9;
const VGA_MISC_READ: u32 = 0x3CC;
const VGA_GC_INDEX: u32 = 0x3CE;
const VGA_GC_DATA: u32 = 0x3CF;
const VGA_CRTC_INDEX: u32 = 0x3D4;
const VGA_CRTC_DATA: u32 = 0x3D5;
const VGA_INSTAT_READ: u32 = 0x3DA;
const VGA_NUM_SEQ_REGS: u32 = 5;
const VGA_NUM_CRTC_REGS: u32 = 25;
const VGA_NUM_GC_REGS: u32 = 9;
const VGA_NUM_AC_REGS: u32 = 21;
const VGA_NUM_REGS: u32 = (1 + VGA_NUM_SEQ_REGS + VGA_NUM_CRTC_REGS + VGA_NUM_GC_REGS + VGA_NUM_AC_REGS);

fn mod_switch(desc: [u8; 61]) {
    let mut offset = 0;
    outport(VGA_MISC_WRITE, desc[offset]);
    offset += 1;

    for _ in 0..VGA_NUM_SEQ_REGS {
        outport(VGA_SEQ_INDEX, offset as u8);
        outport(VGA_SEQ_DATA, desc[offset]);
        offset += 1;
    }

    outport(VGA_CRTC_INDEX, 0x03);
    outport(VGA_CRTC_DATA, inport(VGA_CRTC_DATA) | 0x80);
    outport(VGA_CRTC_INDEX, 0x11);
    outport(VGA_CRTC_DATA, inport(VGA_CRTC_DATA) & !0x80);

    // desc[0x03] |= 0x80;
    // desc[0x11] &= ~0x80;

    for _ in 0..VGA_NUM_CRTC_REGS {
        outport(VGA_CRTC_INDEX, offset as u8);
        outport(VGA_CRTC_DATA, desc[offset]);
        offset += 1;
    }
    /* write GRAPHICS CONTROLLER regs */
    for _ in 0..VGA_NUM_GC_REGS {
        outport(VGA_GC_INDEX, offset as u8);
        outport(VGA_GC_DATA, desc[offset]);
        offset += 1;
    }
    /* write ATTRIBUTE CONTROLLER regs */
    for _ in 0..VGA_NUM_AC_REGS {
        inport(VGA_INSTAT_READ);
        outport(VGA_AC_INDEX, offset as u8);
        outport(VGA_AC_WRITE, desc[offset]);
        offset += 1;
    }
    /* lock 16-color palette and unblank display */
    inport(VGA_INSTAT_READ);
    outport(VGA_AC_INDEX, 0x20);
}

fn outport(port: u32, data: u8) {
    unsafe {
        asm! {
        "out dx, al",
        in("dx") port,
        in("al") data
        }
    }
}

fn inport(port: u32) -> u8 {
    let res: u8;
    unsafe {
        asm! {
        "in al, dx",
        in("dx") port,
        out("al") res
        }
    }
    return res;
}