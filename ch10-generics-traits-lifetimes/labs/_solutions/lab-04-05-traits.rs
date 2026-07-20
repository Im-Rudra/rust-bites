// SOLUTION — lab-04-05-traits

use std::fmt::Display;

// TODO(1): the contract
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
}

// TODO(2): override the default
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

// TODO(3): only the required method — summarize comes free
impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// TODO(4): sugar form...
pub fn notify(item: &impl Summary) {
    println!("Breaking! {}", item.summarize());
}
// ...and the full form (same meaning):
pub fn notify_full<T: Summary>(item: &T) {
    println!("Breaking! {}", item.summarize());
}

// TODO(5): conditional method
struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    // TODO(6)
    let article = NewsArticle {
        headline: String::from("Penguins Win the Cup"),
        author: String::from("Iceburgh"),
    };
    let post = SocialPost {
        username: String::from("rudra"),
        content: String::from("learning traits!"),
    };
    notify(&article); // overridden summarize
    notify(&post);    // default summarize, via summarize_author
    notify_full(&post);
    let _ = &post.content;

    // TODO(7)
    Pair { x: 5, y: 10 }.cmp_display();
    // Pair { x: vec![1], y: vec![2] } would NOT get cmp_display:
    // Vec is PartialOrd but not Display — it fails the Display half of the bound.
}
