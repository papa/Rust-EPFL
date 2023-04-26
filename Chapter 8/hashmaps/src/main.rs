//hash maps not included by default
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"),50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key} : {value}");
    }
    //hash maps become owners of values that are moved not copied

    //overwriting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    //or_insert, if key not present, insert value, otherwise don't do anything
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; //count is mutable reference
    }

    println!("{:?}", map);

    //by default SipHash is used
}
