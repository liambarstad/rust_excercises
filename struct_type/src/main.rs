fn main() {
    // entire must be mutable to change any values
    let mut user1 = User {
        active: true,
        username: String::from("your mom"),
        email: String::from("your_mom@your_mom.com"),
        sign_in_count: 1
    };

    // set new value
    user1.email = String::from("someone_elses_mom@your_mom.com");

    // struct update syntax: similar to js/python
    let user2 = User {
        email: String::from("something@new.com"),
        ..user1
    };

    println!("{}", user1.email);
    println!("{}", user2.email);
}

fn build_user(username: String, email: String) -> User {
    // can use field shorthand (similar to javascript)
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
// can't set a field to be a reference to a slice (&str) without lifetime parameter

// tuple-like structs, only collection of ordered values (but named)
struct Color(i32, i32, i32);
struct Point(i32, i32);


