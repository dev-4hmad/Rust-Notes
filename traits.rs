//First defined a trait
trait Summary {
    fn summarize(&self) -> String;
}
// First Struct
struct Article {
    headline: String,
    author: String,
    content: String,
}
// Second Struct
struct Tweet {
    username: String,
    content: String,
}
// Trait used
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}
// Trait used
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// Generic function which will accept every type
// Must impliment the summary trait
fn notify<T: Summary>(item: T) {
    println!("Breaking News: {}", item.summarize());
}

fn main() {
    let article = Article {
        headline: String::from("Rust is getting popular!"),
        author: String::from("Muhammad Ahmad"),
        content: String::from("Rust ownership and safety are making it famous."),
    };

    let tweet = Tweet {
        username: String::from("ahmad123m"),
        content: String::from("Learning traits in Rust 🚀"),
    };

    notify(article);
    notify(tweet);
}
