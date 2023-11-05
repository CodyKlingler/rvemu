mod risc;
use risc::RiscV;

// Not sure if my code works on Big-Endian machines

pub fn main() {
    
    let cpu = RiscV::<u32, 1024>::new();

    cpu.lb(0).expect("lb");
    cpu.lbu(0).expect("lbu");
    cpu.lw(0).expect("lw");
    cpu.lh(0).expect("lh");
    cpu.lhu(0).expect("lhu");

    for i in 0.. 10 {
        println!("{:}", i);
        println!("{:}", u8::MAX.wrapping_shl(i));
    }
}
