#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

unsafe extern "C" {
    pub fn distance(p1: *const Point, p2: *const Point) -> f32;
}

pub fn safe_distance(p1: &Point, p2: &Point) -> f32 {
    unsafe { distance(p1 as *const Point, p2 as *const Point) }
}
