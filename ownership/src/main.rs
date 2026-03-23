fn main() {
    // 1. simple use of heap example
    // let mut s = String::from("hello");
    // s.push_str(", world");

    // println!("{s}",)

    // 2. invalidated reference example
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{s1}, world!");

    // 3. clone example
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {s1}, s2 = {s2}");

    // 4. ownership and functions
    // let s = String::from("hello"); // s comes into scope

    // takes_ownership(s); // s's value moves into the function...
    // // ... and so is no longer valid here

    // let x = 5; // x comes into scope

    // makes_copy(x); // Because i32 implements the Copy trait,
    // // x does NOT move into the function,
    // // so it's okay to use x afterward.
    // println!("{x}")

    // 5. return values and scope
    // let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    // let s2 = String::from("hello"); // s2 comes into scope

    // let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back,
    // // which also moves its return value into s3
    // // value into s1

    // 6. References and Borrowing
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);

    // println!("The length of '{s1}' is {len}");

    // 7. dangaling references
    // let reference_to_nothing = dangle();
}

// 4.
// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
// // memory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.

// 5.
// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// 6.
// fn calculate_length(s: &String) -> usize {
//     // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because s does not have ownership of what
// // it refers to, the String is not dropped.

// 7.
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
