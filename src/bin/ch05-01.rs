#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    /* Structs */

    let mut user_01 = User {
        email: String::from("user01@example.com"), // wrong email
        username: String::from("user_01"),
        active: true,
        sign_in_count: 1,
    };

    user_01.email = String::from("user_01@example.com"); // correcting email

    println!("\n{:#?}", user_01);

    let user_02 = build_user("user_02@example.com".to_string(), "user_02".to_string());

    println!("\n{:#?}", user_02);

    let user_03 = User {
        email: "user_03@example.com".to_string(),
        username: "user_03".to_string(),
        ..user_02
    };

    println!("\n{:#?}", user_03);

    /* Tuple Structs */

    let black = Color(0, 0, 0);
    println!("\n{:?}", black);

    let origin = Point(0, 0, 0);
    println!("\n{:?}", origin);

    println!("");
}
