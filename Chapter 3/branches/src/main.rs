fn main() {
    let x = 5;
    if x < 5
    {
        println!("Less than 5");
    }
    else if x == 5
    {
        println!("Equal to 5");
    }
    else
    {
        println!("Greater than 5");
    }

    let condition = false;
    let number = if condition {5} else {6}; //if in a let, has to have same types

    println!("The value of number is {number}");
}
