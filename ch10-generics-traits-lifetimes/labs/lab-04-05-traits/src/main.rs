// LAB 04–05 — Contracts and Bounds
// Matching notes: 04-traits, 05-trait-bounds
//
// Goal: define a contract, sign it twice, get a method for free,
// and write functions that accept "anything that signed."

use std::fmt::Display;

// TODO(1): define the contract:
//   pub trait Summary {
//       fn summarize_author(&self) -> String;              // required
//       fn summarize(&self) -> String {                    // default, uses the required one
//           format!("(Read more from {}...)", self.summarize_author())
//       }
//   }

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
}

// TODO(2): impl Summary for NewsArticle — implement BOTH methods
// (override the default: "HEADLINE, by AUTHOR").

// TODO(3): impl Summary for SocialPost — implement ONLY summarize_author
// (returning "@username"). summarize comes free. Feel the default kick in.

// TODO(4): write notify using the `impl Trait` sugar:
//   pub fn notify(item: &impl Summary) { println!("Breaking! {}", item.summarize()); }
// BONUS: rewrite it in the full form  pub fn notify<T: Summary>(item: &T)
// — same meaning, note 05's "three spellings."

// TODO(5): conditional method — only printable+comparable pairs can cmp_display:
//   struct Pair<T> { x: T, y: T }
//   impl<T: Display + PartialOrd> Pair<T> {
//       fn cmp_display(&self) { /* print the larger of x, y */ }
//   }

fn main() {
    // TODO(6): build one NewsArticle and one SocialPost; call notify on BOTH.
    // Same function, two types — the contract made it possible.

    // TODO(7): Pair { x: 5, y: 10 }.cmp_display();
    // THINK: would Pair { x: vec![1], y: vec![2] } get cmp_display? Why not?
    // (Which of the two required contracts does Vec fail?)
}
