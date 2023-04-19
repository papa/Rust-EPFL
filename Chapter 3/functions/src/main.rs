fn main() 
{
    //statements - don't return a value
    //expressions - return a value
    println!("Hello, world!");
    some_f();
    f_params(5);


    let y = {
        let x = 3;
        x + 1 // no semicolon here, like a return statement
    };

    println!("The value of y is {y}");

    let x = five();
    println!("The value of x is {x}");
}

//not important where it is defined
fn some_f(){
    println!("Some f");
}

fn f_params(x : i32){
    println!("value of x is {x}");
}

fn five() -> i32{
    5
}