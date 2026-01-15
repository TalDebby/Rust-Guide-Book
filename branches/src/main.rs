fn main() {
    twelve_days()
}

// Excersise 1: Celcuise to Fahrenheit and vice versa

fn to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// Excersise 2: nth fibonacci number
fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    let mut result = 0;
    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        result = a + b;
        a = b;
        b = result;
    }

    return result;
}

// Excersise 3: Twelve Days of Christmas

fn twelve_days() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts: [&str; 12] = [
        "a partridge in a pear tree.",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for i in 0..gifts.len() {
        println!("On the {} day of Christmas,", days[i]);
        println!("my true love gave to me:");

        for j in (0..=i).rev() {
            // Add "and" before the very first gift on any day after the first
            let prefix = if j == 0 && i > 0 { "and " } else { "" };
            println!("{}{}", prefix, gifts[j]);
        }

        println!();
    }
}

// EXAMPLES

fn range_loop() {
    for nuber in (1..4).rev() {
        println!("{nuber}!");
    }
    println!("LIFTOFF!!!");
}

fn iterate_over_array() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn iterate_over_array_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        // relatively slow due to rut runtime check
        println!("the value is: {}", a[index]);

        index += 1
    }
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_lable() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn let_if() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn basic_if() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn divisible() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
