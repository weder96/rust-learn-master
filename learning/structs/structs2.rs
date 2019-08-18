struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = build_user("weder96".to_string(), "weder96".to_string());
    println!("teste user {}",user2.email);
    user2.email = String::from("someone@example.com");
    println!("teste user {}",user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1,
    }
}