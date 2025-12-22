use rand::random;

enum IpAddrKind {
    V4(String),
    V6(String)
}

impl IpAddrKind {
    fn print(&self) {
        match self {
            IpAddrKind::V4(addr) => println!("{addr} is a v4 addr"),
            IpAddrKind::V6(addr) => println!("{addr} is a v6 addr")
        };
    }
}


fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    four.print();
    six.print();
}

fn maybe() -> Option<String> {
    let rng = random::<u8>();

    match rng % 2 == 0 {
        true => Some(String::from("Is even")),
        false => None
    }
}
