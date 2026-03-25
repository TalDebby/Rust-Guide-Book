// In this example x is refrenced 
// fn dangaling_refrence() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {r}");
// }


fn lifetime_example() {
    let x = 5;       // ----------+-- 'b
                          //           |
    let r = &x;     // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+

fn longest_string() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

// longest doesn't know what the refrence of the return type is

// fn longest(x:&str, y:&str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn when_does_combined_lifetime_expire() {
    let string1 = String::from("long string is long");
    let result: &str;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // wan't work here since string2 has been droped
    // println!("The longest string is {result}");
}

// fn longest_lifetime_in_return<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//
//      // The result variable is cleard after it goes out of scope (eventhough we defined liftime)
//     result.as_str()
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn struct_with_lifetime() {
    let novel = String::from("Call me Shuli. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// traits & lifetimes
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
