pub mod vectors;
pub mod strings;
pub mod maps;

fn main() {
    let vec = vec![1,6,3,2,4,5,6,6,6,7,8,9,11,12,13,14,15,16,17,18,19,20];
    println!("median: {}",vectors::median(&vec));
    if let Some(val) = maps::mode(&vec) {
        println!("mode: {}", val);
    }
}