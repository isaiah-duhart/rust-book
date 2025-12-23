fn main() {
    let x = "a";
    {
        let y = "ab";
        let z = longest(x, y);
        println!("{}", z);
    }

    let str = "str";
    {
        let str_1 = str_slice(str);
        println!("{}", str_1);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn str_slice(x: &str) -> &str {
    &x[1..]
}
