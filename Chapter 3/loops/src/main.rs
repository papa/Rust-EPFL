fn main() {
    let mut x = 0;
    loop {
        println!("again!");
        x = x + 1;
        if x == 5{
            break;
        }
    }

    //returning values from loop
    let mut cnt = 0 ;
    let result = loop {
        cnt+=1;
        if cnt == 10 {
            break 2*cnt;
        } 
    };
    println!("The result is {result}");


    let mut count =  0;
    'counting_up : loop {
        println!("count = {count}");
        let mut rem = 10;

        loop {
            println!("rem = {rem}");
            if rem == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            rem-=1;
        }
        count+=1;
    }
    println!("End count = {count}");

    //while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number-=1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a{ // for iterating collection
        println!("element {element}");
    }

    for num in 1..4 {
        println!("{num}");
    }

    println!("End");
}   
