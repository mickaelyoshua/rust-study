struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = build_user(String::from("user@name.com"), String::from("username"));

    user1.email = String::from("user1@name.com");
    println!("New email: {}", user1.email);

    let user2 = User {
        email: String::from("user2@name.com"),
        ..user1
    };

    println!("user1 email: {}\nuser2 email: {}", user1.email, user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("point: {}, {}, {}", origin.0, origin.1, origin.2);
    let Color(r, g, b) = black;
    println!("color: {}, {}, {}", r, g, b);

    let subject = AlwaysEqual;
}
