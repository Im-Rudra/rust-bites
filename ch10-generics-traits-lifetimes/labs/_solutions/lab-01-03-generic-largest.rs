// SOLUTION — lab-01-03-generic-largest

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// TODO(1): the bound the compiler demanded
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// TODO(2)
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// TODO(3): only f32 points get this
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest_i32(&numbers));

    // TODO(4): one function, two stamped copies
    println!("largest (generic, i32): {}", largest(&numbers));
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest (generic, char): {}", largest(&chars));

    // TODO(5)
    let p_int = Point { x: 5, y: 10 };
    let p_float = Point { x: 3.0_f32, y: 4.0_f32 };
    println!("p_int.x = {}", p_int.x());
    println!("p_float distance = {}", p_float.distance_from_origin()); // 5.0
    // p_int.distance_from_origin(); // ❌ method exists only for Point<f32>

    // THINK: Point { x: 5, y: 4.0 } fails because T was inferred as i32
    // from x, and 4.0 is not an i32 — Point<T> promised both fields share T.
}
