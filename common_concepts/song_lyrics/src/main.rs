fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let rl1 = "On the";
    let rl2 = "day of Christmas\nMy true love sent to me";

    let days_lines = ["A partridge in a pear tree",
    "Two turtle-doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings (five golden rings)",
    "Six geese a laying",
    "Seven swans a swimming",
    "Eight maids a milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
    ];

    for i in 0..days.len() {
        println!("{} {} {}", rl1, days[i], rl2);
        for j in (0..i).rev() {
            println!("{}", days_lines[j]);
        }
        println!();
    }
}
