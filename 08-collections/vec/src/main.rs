fn main() {
    let mut vec = vec![String::from("Hi"), String::from("New"), String::from("Collection!")];

    let fourth = String::from("Will this");
    vec.push(fourth);
    vec[3].push_str(" work?");

    if let Some(str) = vec.get(2) {
        println!("Third is {str}");
    }

    vec[0].push_str(" Rust!");

    for str in &vec {
        println!("{str}");
    }

    println!("{vec:?}");

    
}
