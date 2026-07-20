// LAB 06–09 — Leashes for References
// Matching notes: 06-why-lifetimes, 07-lifetime-annotations,
//                 08-elision-and-static, 09-all-together
//
// Goal: cause a dangle, fix `longest` yourself, put a reference in a struct,
// and finish with the chapter-boss signature.

use std::fmt::Display;

// TODO(1): type this WITHOUT lifetimes first and read error E0106:
//   fn longest(x: &str, y: &str) -> &str {
//       if x.len() > y.len() { x } else { y }
//   }
// Then fix it the way the compiler asks: add <'a> and tie all three
// references to it. Say out loud what 'a promises (note 07).

// TODO(2): a struct that BORROWS its data:
//   struct ImportantExcerpt<'a> { part: &'a str }
// Why does it need <'a>? One sentence, out loud.

// TODO(3): the boss — all three vocabularies in one signature:
//   fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str
//   (print the announcement, then return the longer input)

fn main() {
    // TODO(4): prove your longest() works:
    //   let s1 = String::from("long string is long");
    //   let s2 = String::from("xyz");
    //   println!("longest: {}", longest(s1.as_str(), s2.as_str()));

    // TODO(5): now BREAK it on purpose (then comment the breakage out):
    //   let s1 = String::from("long string is long");
    //   let result;
    //   {
    //       let s2 = String::from("xyz");
    //       result = longest(s1.as_str(), s2.as_str());
    //   }
    //   println!("{result}");   // ← read E0597: why rejected, even though
    //                           //   result HAPPENS to point at s1? (note 07)

    // TODO(6): build an ImportantExcerpt borrowing the first sentence of a
    // String (hint: novel.split('.').next().unwrap()), print its part.

    // TODO(7): call longest_with_an_announcement with any printable announcement.

    // THINK (note 08): why does fn first_word(s: &str) -> &str need NO
    // annotations, while longest does? Which elision rule saves it?
}
