use std::fmt;

pub trait Yolo<T> {
    fn yolo(self) -> T;
}

impl<T> Yolo<T> for Option<T> {
    fn yolo(self) -> T {
        self.unwrap()
    }
}

impl<T, E> Yolo<T> for Result<T, E> where E: fmt::Debug {
    fn yolo(self) -> T {
        self.unwrap()
    }
}

#[cfg(test)]
mod tests {
    fn get_result(v: i32) -> Result<i32, ()> {
        Ok(v)
    }

    #[test]
    fn it_works() {
        assert_eq!(Some(123).yolo(), 123);
        assert_eq!(get_result(123).yolo(), 123)
    }
}
    
