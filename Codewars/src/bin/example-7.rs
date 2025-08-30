// .enumerate() - I find this so important, It gives the INDEX with the ITEM

fn accum(s:&str)->String {
    // Ok so my code
    s.chars().enumerate().map(|(i, c)|{
        format!("{}{}", c.to_ascii_uppercase(), c.to_ascii_lowercase().to_string().repeat(i))
    }).collect::<Vec<String>>()
    .join(" ")
}
fn main(){
    let word = "Convert your string into very fun characters, whoooow, I just want to include some more words so my string would look like a waves of waterrrrrrrr";
    println!("{}", accum(word));
}
