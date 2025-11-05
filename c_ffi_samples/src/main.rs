use crate::jump::{test_goto, test_jump};
use crate::misc::nop;
use crate::point::{Point, distance};
use crate::print::c_print;

mod jump;
mod misc;
mod point;
mod print;

#[allow(unused)]
mod jump_c {
    include!(concat!(env!("OUT_DIR"), "/jump.rs"));
}

fn main() {
    // Using the original print function
    c_print("Hello, World!\n");

    // Using the cprintln! macro (like println!)
    cprintln!("Hello from cprintln!");
    cprintln!("Formatted output: {} + {} = {}", 5, 3, 5 + 3);
    cprintln!("Hexadecimal: 0x{:X}", 255);

    // Using cprint! macro (without newline)
    cprint!("This is ");
    cprint!("on the ");
    cprintln!("same line!");

    // Using the FFI function to calculate distance between two points
    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 3, y: 4 };
    let dist = unsafe { distance(&p1, &p2) };
    println!("Distance between points: {dist}");

    cprintln!("Calling nop function...");
    unsafe {
        nop();
    }

    test_jump();

    test_goto(5);
}
