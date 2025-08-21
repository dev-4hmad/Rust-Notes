// Define a trait
trait Summary {
    fn summarize(&self) -> String;
}

// Implement the trait for a struct
struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn main() {
    let article = Article {
        title: String::from("Rust is Awesome"),
        author: String::from("Ahmad"),
    };

    println!("Article summary: {}", article.summarize());
}
