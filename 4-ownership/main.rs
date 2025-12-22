fn main() {
    let my_string = String::from("This is Mine!");

    let str_ref = &my_string;

    println!("{my_string} {str_ref}");

    let give_me = my_string;

    println!("{give_me}");
}