fn main() {
    
    let mut user1 = User {
        active : true, 
        username : String::from("username"),
        email : String::from("email"),
        sign_in_count : 1,
    };

    //cannot mark only field as mutable, whole variable must be mutable
    user1.email = String::from("email2");

    let user2 = User{
        active : user1.active,
        username : user1.username, 
        email : String::from("another"),
        sign_in_count : user1.sign_in_count,
    };

    let user3 = User {
        email : String::from("another 3"),
        ..user2 // other params same as from user2, must be last
    };
    //note : strings are moving, not copying, so some instances may become unusable
    let black = Color(0,0,0);


    
}

struct Color(i32,i32,i32); // tuple struct

struct unitStruct; // unit-like struct, ()

struct User
{
    active: bool,
    username: String, 
    email: String, 
    sign_in_count : u64,
}

fn build_user(email: String, username : String) -> User {
    User {
        active: true,
        username, //username from function params
        email, // email from functions params
        sign_in_count : 1,
    }
}