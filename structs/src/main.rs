fn main() {
    let user1 = build_user(String::from("nigga"), String::from("nigga@gmail.com"));
    
    println!("{} {}", user1.email, user1.username);

    let user2 = User{
        username: String::from("WhiteCrack"),
        ..user1
    };

    println!("{}", user2.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let test = AlwaysEqual;

}

fn build_user(username: String, email: String) -> User{
    User{
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;