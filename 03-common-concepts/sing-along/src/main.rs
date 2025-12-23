fn main() {
    sing_christmas_carol();
}

fn sing_christmas_carol() {
    let lyrics = [
        ("first", "a partridge in a pear tree"),
        ("second", "two turtle doves"),
        ("thrid", "three French hens"),
        ("fourth", "four calling birds"),
        ("fifth", "five gold rings"),
        ("sixth", "six geese a-laying"),
        ("seventh", "seven swans a-swimming"),
        ("eigth", "eight maids a-milking"),
        ("ninth", "nine ladies dancing"),
        ("tenth", "ten lords a-leaping"),
        ("eleventh", "eleven pipers piping"),
        ("twelfth", "twelve drummers drumming")
    ];
    for x in 0..=11 {
        let day = lyrics[x].0;
        let mut lyric = String::new();
        for y in (0..=x).rev() {
            lyric.push_str(&format!("{}, ", lyrics[y].1));
        }
        println!("On the {day} of Chirstmas, my true love gave to me");
        println!("{}", capitalize_string(&lyric));
    }
}

fn capitalize_string(str: &str) -> String {
    let mut chars: Vec<char> = str.chars().collect();

    chars[0] = chars[0].to_ascii_uppercase();

    return chars.into_iter().collect()
}
