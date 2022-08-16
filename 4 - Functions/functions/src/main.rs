fn main() {
    println!("Hello, world!");
    let x = five();

    let x = plus_one(5); 
    println!("The value of x is: {x}");
}
 
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
 