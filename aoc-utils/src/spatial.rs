pub struct Point2d<T> {
    pub x: T,
    pub y: T
}


impl<T: Default + Clone> Point2d<T> {
    pub fn new() -> Point2d<T> {
        Point2d::<T>{
            x: T::default(),
            y: T::default()
        }
    }

    pub fn to_tuple(&self) -> (T, T) {
        (self.x.clone(), self.y.clone())
    }
}


pub trait Distance<T> {
    fn manhattan_distance(&self, other: &Point2d<T>) -> T;
}


impl Distance<i32> for Point2d<i32> {
    fn manhattan_distance(&self, other: &Point2d<i32>) -> i32 {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();

        // return
        x_diff + y_diff
    }
}


impl Distance<f32> for Point2d<f32> {
    fn manhattan_distance(&self, other: &Point2d<f32>) -> f32 {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();

        // return
        x_diff + y_diff
    }
}
