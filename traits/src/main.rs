struct Point {
    x: i32,
    y: i32,
}

trait PointOps {
    fn is_x_greater(&self) -> bool;
    fn is_y_greater(&self) -> bool;
    fn are_points_equal(&self) -> bool;
}

impl PointOps for Point {
    fn is_x_greater(&self) -> bool {
        self.x > self.y
    }

    fn is_y_greater(&self) -> bool {
        self.y > self.x
    }

    fn are_points_equal(&self) -> bool {
        self.x == self.y
    }
}

fn main() {
    let my_point: Point = Point { x: 5, y: 3 };
    println!("{}", my_point.is_x_greater());
}
