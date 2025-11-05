use std::ffi::CString;

pub mod print_c {
    include!(concat!(env!("OUT_DIR"), "/print.rs"));
}

pub fn c_print(out: &str) {
    let message = CString::new(out).unwrap();
    unsafe { print_c::print_message(message.as_ptr()) };
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
        unsafe { $crate::print::print_c::print_message(c_msg.as_ptr()) }
    }};
}

// cprint! macro (without newline)
#[macro_export]
macro_rules! cprint {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        let c_msg = std::ffi::CString::new(msg).unwrap();
        unsafe { $crate::print::print_c::print_message(c_msg.as_ptr()) }
    }};
}
