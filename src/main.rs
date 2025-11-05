use std::ffi::CString;

mod print {
    include!(concat!(env!("OUT_DIR"), "/print.rs"));
}

pub fn print(out: &str) {
    let message = CString::new(out).unwrap();
    unsafe { print::print_message(message.as_ptr()) };
}

// cprintln! macro that mimics println!
#[macro_export]
macro_rules! cprintln {
    () => {
        cprintln!("")
    };
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        let c_msg = std::ffi::CString::new(format!("{}\n", msg)).unwrap();
        unsafe { $crate::print::print_message(c_msg.as_ptr()) }
    }};
}

// cprint! macro (without newline)
#[macro_export]
macro_rules! cprint {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        let c_msg = std::ffi::CString::new(msg).unwrap();
        unsafe { $crate::print::print_message(c_msg.as_ptr()) }
    }};
}

fn main() {
    // Using the original print function
    // print("Hello, World!");

    // // Using the cprintln! macro (like println!)
    // cprintln!("Hello from cprintln!");
    // cprintln!("Formatted output: {} + {} = {}", 5, 3, 5 + 3);
    // cprintln!("Hexadecimal: 0x{:X}", 255);

    // // Using cprint! macro (without newline)
    // cprint!("This is ");
    // cprint!("on the ");
    // cprintln!("same line!");

    // Using the FFI function to calculate distance between two points
    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 3, y: 4 };
    let dist = unsafe { distance(&p1, &p2) };
    cprintln!("Distance between points: {}", dist);

    cprintln!("Calling nop function...");
    unsafe {
        nop();
    }
}

#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

unsafe extern "C" {
    fn distance(p1: *const Point, p2: *const Point) -> f32;
}

unsafe extern "C" {
    fn nop();
}
