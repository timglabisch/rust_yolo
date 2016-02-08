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

use

```
[dependencies]
yolo = "*"
```

make sure you DONT'T pin it to a version -> YOLO!.

# special thanks
special thanks to (matthiasendler)[https://github.com/mre] who recognized (how important)[https://twitter.com/matthiasendler/status/696769903022497792] this crate is 
