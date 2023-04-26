fn main() {

    let mut v1 : Vec<i32> = Vec::new();
    let mut v2 = vec![1,2,3];

    v1.push(-1);
    v2.push(4);

    let third : &i32 = &v2[2];
    println!("third element is {third}");

    let third : Option<&i32> = v1.get(2); // get returns an option<&T>
    match third {
        Some(third) => println!("{third}"), 
        None => println!("no third el"),
    }

    // let x : &i32 = &v2[100]; //code would panic

    //iterating over elements ----------------
    for i in &v2 {
        println!("{i}");
    }

    for i in &mut v2 {
        *i += 50;
    }

    for i in &v2 {
        println!("{i}");
    }
}
