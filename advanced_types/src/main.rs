pub mod generics;
pub mod traits;
pub mod lifetimes;

use traits::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    // to use summarize we must bring in the trait Summary into scope
    println!("1 new post: {}", post.summarize());
}