#![feature(asm)]

fn f() {
    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
        "mov {0}, {1}",
        "add {0}, {number}",
        out(reg) o,
        in(reg) i,
        number = const 5,
        );
    }
    assert_eq!(o, 8);
}

fn f1(){
    #![feature(asm)]
    let mut a: u64 = 4;
    let b: u64 = 4;
    let c: u64 = 4;
    unsafe {
        asm!(
        "add {0}, {1}",
        "add {0}, {2}",
        inout(reg) a,
        in(reg) b,
        in(reg) c,
        );
    }
    assert_eq!(a, 12);
}

fn cache(){
    let esi: u32;
    let ecx: u32;

    unsafe {
        asm!(
        "cpuid",
        // EAX 4 selects the "Deterministic Cache Parameters" CPUID leaf
        inout("eax") 4 => _,
        // ECX 0 selects the L0 cache information.
        inout("ecx") 0 => ecx,
        lateout("esi") esi,
        lateout("edx") _,
        );
    }

    println!(
        "L1 Cache: {}",
        ((esi >> 22) + 1) * (((esi >> 12) & 0x3ff) + 1) * ((esi & 0xfff) + 1) * (ecx + 1)
    );
}

fn main() {
    println!("Hello, world!");
    f();
    f1();
    cache();
}
