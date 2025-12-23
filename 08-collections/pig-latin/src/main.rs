fn main() {
    match pig_latin("first") {
        None => println!("first cannot be translated to pig latin"),
        Some(str) => println!("first in pig latin is {}", str)
    }
    match pig_latin("apple") {
        None => println!("apple cannot be translated to pig latin"),
        Some(str) => println!("apple in pig latin is {}", str)
    }
    match pig_latin("") {
        None => println!("'' cannot be translated to pig latin"),
        Some(str) => println!("'' in pig latin is {}", str)
    }
    match pig_latin("f") {
        None => println!("f cannot be translated to pig latin"),
        Some(str) => println!("f in pig latin is {}", str)
    }
}


fn pig_latin(word: &str) -> Option<String> {
    let Some(first_letter) = word.chars().next() else {
        return None;
    };

    if !first_letter.is_ascii() {
        return None;
    }

    if matches!(first_letter.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') {
        return Some(String::from(format!("{}-hay", word)))
    } else {
        let Some(word_slice) = word.get(1..) else {
            return None;
        };
        return Some(String::from(format!("{}-{}ay", word_slice, first_letter)))
    }
}
