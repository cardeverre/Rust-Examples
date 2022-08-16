

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//Variables
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

//Shadowing
fn main1() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

//Data Types
fn main2() {
    let guess: u32 = "42".parse().expect("Not a number!");
}

//Numeric Operations
fn main3() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

//The Boolean Type
fn main4() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

//The Character Type
fn main5() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

//Compound Types
//Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

//The Tuple Type
//A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
//Tuples have a fixed length: once declared, they cannot grow or shrink in size.
fn main6() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

//The variable tup binds to the entire tuple, because a tuple is considered a single compound element. 
//To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
fn main7() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

/*
    This program first creates a tuple and binds it to the variable tup. 
    It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. This is called destructuring,
    because it breaks the single tuple into three parts. Finally, the program prints the value of y, which is 6.4.
*/
fn main8() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}


//The Array Type 
/*
Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type.
Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.
*/
fn main9() {
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    /*
    You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, 
    and then the length of the array in square brackets, as shown here: 
    */
    let a = [3; 5];
    /*The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way. */

}

//Accessing Array Elements
fn main10() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
 

