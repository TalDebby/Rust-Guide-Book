use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait SummaryWithDefault {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait SummaryInternalRefrence {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn generic_notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn combined_trait_bounds(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item);
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{
    println!("nothing");
    0
}

fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    }
}

// Can't return diffrent types that implement Summary trait

// fn return_diffrent_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             repost: false,
//         }
//     }
// }

// Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {0}", self.x);
        } else {
            println!("The largest member is y = {0}", self.y);
        }
    }
}

fn use_pair() {
    let p1 = Pair::new(1, 2);
    p1.cmp_display();

    let p2 = Pair::new(
        SocialPost { 
            username: "Tal".to_string(), 
            content: "I love pairs".to_string(), 
            reply: true, 
            repost: false 
        }, 
        SocialPost { 
            username: "Itay".to_string(), 
            content: "They call me Itay and my name is Itay".to_string(), 
            reply: false, 
            repost: true 
        }
    );

    // cmp_dispalt wan't work on p2
    // p2.cmp_display();
}


// blanket implementations 

// impl<T: Display> ToString for T {
//     // --snip--
// }