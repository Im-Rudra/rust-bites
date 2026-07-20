// SOLUTION — lab-06-09-lifetimes

use std::fmt::Display;

// TODO(1): all three references tied to the same leash 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// 'a promises: the returned reference is valid only while BOTH inputs are.

// TODO(2): the struct cannot outlive whatever `part` points at
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// TODO(3): the boss — lifetime + generic + trait bound in one signature
fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // TODO(4)
    let s1 = String::from("long string is long");
    let s2 = String::from("xyz");
    println!("longest: {}", longest(s1.as_str(), s2.as_str()));

    // TODO(5): the breakage, kept as a comment —
    // {
    //     let s1 = String::from("long string is long");
    //     let result;
    //     {
    //         let s2 = String::from("xyz");
    //         result = longest(s1.as_str(), s2.as_str());
    //     }                       // s2 dies; 'a (the shorter life) ends here
    //     println!("{result}");   // ❌ E0597 — result MIGHT be s2; the compiler
    // }                           //    must assume the worst case, so: rejected.

    // TODO(6)
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("excerpt: {}", excerpt.part);

    // TODO(7)
    let winner = longest_with_an_announcement(s1.as_str(), s2.as_str(), "today's race!");
    println!("winner: {winner}");

    // THINK: first_word(s: &str) -> &str is saved by elision rule 2 —
    // exactly ONE input lifetime, so it's assigned to the output automatically.
    // longest has TWO inputs and no &self: no rule applies → you annotate.
}
