fn main() {
    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a-Laying",
        "seven Swans a-Swimming",
        "eight Maids a-Milking",
        "nine Ladies Dancing",
        "ten Lords a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me:",
            ordinals[day]
        );

        for i in (0..=day).rev() {
            if i == 0 {
                if day == 0 {
                    println!("{}", gifts[0]);
                } else {
                    println!("and {}", gifts[0]);
                }
            } else {
                println!("{},", gifts[i]);
            }
        }

        println!(); // blank line between verses
    }
}
