fn explicit_vector() -> Vec<i32>{
    let v: Vec<i32> = Vec::new();
    v
}

fn implicit_vector() -> Vec<i32>{
    let v = vec![1,2,3];
    v
}

fn update_vector(mut v: Vec<i32>) -> Vec<i32>{
    v.push(5);
    v
}

fn access_elements() {
    let v = vec ![1,2,3,4,5,];

    let third = &v[2];
    println!("The third element is {}", third);



    let third = v.get(2);
    match third {
        Some(val) => println!("The third element is {}", val),
        None => println!("There is no third element")
    }
} // v and its items are being dropped here


fn out_of_index_crash() {
    let v = vec![1,2,3,4,5];

    let does_not_exist = &v[100];
}

fn borrowing() {
    let mut v = vec![1,2,3,4,5];

    let first = v.get(0);
    // Remove the comment for borow error
    // v.push(6);
    println!("The first element is: {}", first.unwrap());
}

fn iterate() {
    let v = vec![100,32,57];

    for i in &v {
        println!("{}", i);
    }
}

fn muatable_iteration() {
    let mut v = vec![100,32,57];

    for i in &mut v {
        *i += 50;
    }
}

fn spread_sheet() {
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text("aaa".to_string()),
        SpreadSheetCell::Float(3.141592),
    ];
}
