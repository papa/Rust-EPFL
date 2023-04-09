const CONSTANT : u32 = 1000;

fn main() {

    println!("The value of CONSTANT is: {CONSTANT}"); //1000

    let mut x = 5; // let mut - mutable variable
    let _y = 5; // _ prefix for unused variables, not to get a warning
    println!("The value of x is: {x}"); //5
    x = 6;
    //y = 6;  //would produce compiler error
    println!("The value of x is: {x}"); //6

    let x = 2*x;
    println!("The value of x is: {x}"); //12
    {
        let x = 3; // shadowing of x
        println!("The value of x is: {x}");
    } // returns to 12

    println!("The value of x is: {x}"); //12

    // change of type allowed
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}"); //3
    //change of type not allowed
    // let mut spaces_mut = "   ";
    // spaces_mut = spaces_mut.len(); 

    //---------------------------------------------------------------------------------

    //four scalar types
    //integers, floats, booleans, chars

    //for integers default is i32, arch type based on architecture, 32 or 64
    //for floats default is f64
    //char - 4 bytes
    let _sum = 5 + 10;
    let _prod = 4*30;
    let _trunc = -5/3;//-1

    let _f: bool = true;

    let _character : char = 'a';
    let character : char = '😻';
    println!("The value of character is: {character}"); //emoji

    //compound types
    
    //tuples
    let tup : (i32, f64,u8)  = (500, 6.4,1);
    let (_x, y, _z) = tup;

    println!("The value of y is : {y}");

    let _zz = tup.0; // indexing tuple elements, starting from 0
    //empty tuple (unit) is default for return if the expression doesn't return anything

    //arrays
    let _arr = [1,2,3,4,5];
    let _arr : [i32; 5] = [1,2,3,4,5];
    let arr = [3;5]; // [3,3,3,3,3]

    let x = arr[0]; // accessing array elements same as in C/C++
    println!("{x}");
    //accessing elememt at index > len is going to produce 
    //panic (runtime error) in code
    //more in error handling
}
