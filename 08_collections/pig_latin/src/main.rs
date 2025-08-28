// Problem
// Convert strings to pig latin. The first consonant of each word
// is moved to the end of the word and 'ay' is added, so 'first'
// becomes 'irst-fay'. Words that start with a vowel have 'hay'
// added to the end instead ('apple' becomes 'apple-hay').
// Keep in mind details about UTF-9 encoding.

const VOWELS: [char; 6] = ['a', 'e', 'o', 'y', 'u', 'i'];

fn main() {
    let some_string = String::from("whose woods these are i think i know");
    let words = some_string.split_whitespace();
    let mut new_words: Vec<String> = vec![];
    for word in words {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        let mut new_word = String::new();
        if VOWELS.contains(&first_char) {
            // if first letter is a vowel
            new_word = [word, "hay"].join("");
        } else {
            // if first letter is a consonant
            new_word = [chars.collect::<String>(), first_char.to_string(), String::from("ay")].join("");
        }
        new_words.push(new_word);
    }

    let new_string = new_words.join(" ");
    println!("{}", new_string);
}
