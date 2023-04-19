fn main() {
    
    //string are generally imutable

    { //scope starts
        let _s = "hello";
    } // s no longer valid

    //mutable strings
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    //mutable strings come with price, slower because stored on heap

    //variables anda data interacting with move-----------------

    //all ok, because integers are simple values and are copied
    let _x = 5;
    let _y = _x;

    let s1 = String::from("hello");
    let s2 = s1; 
    // copying pointer to heap memory, like copying pointer in C, s1 is no longer valid
    // s2 is owner of that part of the memory, know like move operation

    // println!("{}", s1); // compile error
    println!("{}", s2);
    // no automatic deep copies

    //clone----------
    let s3 = String::from("hello 2");
    let s4 = s3.clone(); // this is actually something like deep copy

    //passing ownership------------------------
    let s5 = f2(s4);

    // println!("{}", s4); // s4 is not valid here anymore
    println!("{}", s5); // but s5 is valid\

    //references--------------------------------------
    let mut sx = String::from("hello ref");
    let len = calc_len(&sx); 
    // passing the reference, doesn't give ownership, know as borrowin

    println!("The lenght of '{}' is {len}", sx);

    change_str(&mut sx); // mutable reference
    println!("{}", sx);

    let mut s = String::from("hello");

    let _r1 = &mut s;
    // let _r2 = &mut s;// would produce compiler error, only one mutable reference at a time
    // println!("{}, {}", _r1,_r2);
    //also cannot have immutable and mutable ref at the same time

    //dangling refs---------------------
    //dangle();


    //string slices--------------------
    //&s[i..j] - substring from i to j-1
    let s = String::from("hello world");
    let s2 = first_word(&s);
    println!("{}", s2);
}


fn f2(str : String) -> String{
    println!("{}", str);
    str
}

fn calc_len(s : &String) -> usize
{
    //changig something through reference is not allowed
    s.len()
}

fn change_str(s: & mut String) { // mutable reference
    s.push_str(" ref");
}

//returns a dangling reference, can be fixed, but later
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i ,&item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}