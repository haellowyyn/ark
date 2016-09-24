pub fn main() -> ! {
    println!("Entered user mode.");

    for i in 0_u8..128_u8 {
        println!("{}", i as char);
    }

    // Don't return.
    loop {}
}
