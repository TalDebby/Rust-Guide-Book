fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Tal"),
        email: String::from("myemail@gmail.com"),
        sign_in_count: 1
    };

    user1.email = String::from("other@gmail.com");

    let user2 = build_user(String::from("email@gmail.com"), String::from("username"));

    let user3 = User {
        email: String::from("gmail@gmail.com"),
        ..user2
    };
    
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    let Point(x,y,z) = origin ;

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email: email,
        sign_in_count: 1
    }
}


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;