use crate::cprintln;

mod jump_c {
    include!(concat!(env!("OUT_DIR"), "/jump.rs"));
}

pub fn test_jump() {
    cprintln!("Calling C with no error:");
    unsafe {
        jump_c::call_with_jump(0);
    }

    cprintln!("Calling C with error (longjmp):");
    let res = unsafe { jump_c::call_with_jump(1) };
    cprintln!("Rust: Result = {}", res);
}

pub fn test_goto(val: i32) {
    unsafe { jump_c::loop_with_goto(val) };
}
