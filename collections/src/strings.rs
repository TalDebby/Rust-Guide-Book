fn new_string() {
    let s = String::new();
}

fn create_string_from() {
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
}

fn utf8_examples() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn push() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");
}

fn concat() {
     let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    // println!("s1 is {s1}"); // plus sign took ownership of s1
    println!("s2 is {s2}");
    println!("s3 is {s3}");
}

fn format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}

fn slice_panic() {
    let s = String::from("Здравствуйте");

    let slice = &s[0..1];
}

fn iterate_string() {
    for c in "Зд".chars() {
        println!("{c}");
    }
}