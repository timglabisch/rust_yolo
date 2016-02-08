# Rust Yolo

this crate allows using `yolo` instead of the `unwrap` function for Option<T> and Result<T, E>
example:

```
extern crate yolo;
use yolo::Yolo;

fn get_result(v : i32) -> Result<i32, ()> {
  Ok(v)
}

fn main() {
  assert_eq!(Some(123).yolo(), 123);
  assert_eq!(get_result(123).yolo(), 123);

  println!("hello world!");
}
```

installation
