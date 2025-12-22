struct User {
    email: String,
    username: String,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("email"),
        username: String::from("username"),
        active: true
    };

    let user2 = User {
        email: user1.email,
        username: user1.username,
        active: true
    };

    println!("{}", user1.active);
    println!("{}", user2.email);
}
