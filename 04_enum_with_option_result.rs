enum LoginResult {
    Success(String), // username
    Failed(String),  // reason
}

fn main() {
    let res1 = LoginResult::Success("Ahmad".to_string());
    let res2 = LoginResult::Failed("Wrong password".to_string());

    match res1 {
        LoginResult::Success(user) => println!("Welcome, {}", user),
        LoginResult::Failed(reason) => println!("Login failed: {}", reason),
    }

    match res2 {
        LoginResult::Success(user) => println!("Welcome, {}", user),
        LoginResult::Failed(reason) => println!("Login failed: {}", reason),
    }
}
