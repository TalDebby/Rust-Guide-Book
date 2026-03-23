use std::collections::HashMap;

use crate::strings;

fn example() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}

fn iterate() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid at this point, try using them and
    // println!("{field_value}");
    // println!("{field_name}");
}

fn overwriting() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}", );
}

fn entry() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}

fn count_words() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

pub fn mode(vec: &Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();
    let mut max = match vec.get(0) {
        Some(val) => *val,
        None => return None
    };
    let mut max_num = 0;
    

    for num in vec {
        let count = map.entry(num).or_insert(0);
        *count += 1;

        if max < *count {
            max = *count;
            max_num = *num
        }
    }

    return Some(max_num);
}