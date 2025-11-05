#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

unsafe extern "C" {
    pub fn distance(p1: *const Point, p2: *const Point) -> f32;
}
