// LAB 01–03 — One Function to Rule All Types
// Matching notes: 01-why-generics, 02-generic-data-types, 03-monomorphization
//
// Goal: feel the de-duplication ladder — and meet your first trait bound
// the way everyone does: because the compiler demanded it.

// This works, but only for i32:
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// TODO(1): write `largest<T>` — same body, generic signature.
//   fn largest<T>(list: &[T]) -> &T
// Then `cargo run` and READ the error. The compiler will tell you T needs
// a contract before `>` is allowed... and even suggest the exact fix.
// Apply it: <T: std::cmp::PartialOrd>. (That's note 05's topic arriving early —
// bounds exist because bare T can't DO anything.)

// TODO(2): a generic struct:
//   struct Point<T> { x: T, y: T }
// plus a method that returns a reference to x:
//   impl<T> Point<T> { fn x(&self) -> &T { ... } }

// TODO(3): give ONLY f32 points a special power:
//   impl Point<f32> { fn distance_from_origin(&self) -> f32 { (self.x.powi(2) + self.y.powi(2)).sqrt() } }

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest_i32(&numbers));

    // TODO(4): call YOUR largest() on `numbers` AND on vec!['y', 'm', 'a', 'q'].
    // One function, two types — monomorphization stamps two copies at compile time.

    // TODO(5): make a Point<i32> and a Point<f32); print p.x().
    // Call distance_from_origin() on the f32 one.
    // Then try calling it on the i32 one and READ that error too. (Note 02.)

    // THINK: try Point { x: 5, y: 4.0 }. Why exactly does the compiler refuse?
}
