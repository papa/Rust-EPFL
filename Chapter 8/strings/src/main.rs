fn main() {
    let mut s1 = String::new();
    let data = "some string";
    let mut s2 = data.to_string(); // string from string literal
    
    s1.push_str("str start"); //adding part of the string
    s2.push_str(" new part");
    println!("{s1}");
    println!("{s2}");

    let s3 = s1 + &s2; // + takes ownership of s1, can't be used anymore

    let _s4 = format!("{s3}-{s2}"); // doesn't take the ownership of any string

    //UTF-8 representation
    //indexing not possible because of representation
    //creating slices with caution - size of one letter is not known at compile time

    let hello = "Здравствуйте";

    //iterating over characters
    for c in hello.chars() {
        println!("{c}");
    }

    //iterating over bytes
    for c in hello.bytes() {
        println!("{c}");
    }


}
