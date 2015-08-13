use std::mem;

pub struct Point { x: i32, y: i32 }

impl Point {
  pub fn distance_to(&self, p: &Point) -> f64 {
    let xdiff = self.x - p.x;
    let ydiff = self.y - p.y;
    ((xdiff.pow(2) + ydiff.pow(2)) as f64).sqrt()
  }
}

impl Drop for Point {
    fn drop(&mut self) {
        use std::io::{Write, stderr};
        writeln!(stderr(), "PKill {{ x: {}, y: {} }}", self.x, self.y).ok();
    }
}

#[no_mangle]
pub extern "C" fn make_point(x: i32, y: i32) -> Box<Point> {
    Box::new(Point { x: x, y: y })
}

#[no_mangle]
pub extern "C" fn get_distance(p1: Box<Point>, p2: Box<Point>) -> f64 {
    let res = p1.distance_to(&p2);
    mem::forget(p1);
    mem::forget(p2);
    res
}

#[test]
fn point_distance() {
    let p1 = Point { x: 1, y: 2 };
    assert_eq!(p1.distance_to(&p1), 0.0);

    let p2 = Point { x: 2, y: 2 };
    assert_eq!(p1.distance_to(&p2), 1.0);
}   