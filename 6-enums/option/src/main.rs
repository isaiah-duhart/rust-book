use rand::random;

fn main() {
    match_main();
    if_let_main();
    let_else_main();
}

fn match_main() {
    for _ in 0..10 {
        match maybe() {
            Some(num) => println!("Some value: {num}"),
            None => println!("None returned")
        };
    }
}

fn if_let_main() {
    for _ in 0..10 {
        let val = maybe();
        if let Some(num) = val {
            println!("Some value: {num}");
        };
    }
}

fn let_else_main() {
    let mut sum: u32 = 0;
    for _ in 0..10 {
        let Some(num) = maybe() else {
            continue;
        };

        sum += num as u32;
    }

    println!("Total sum is {sum}");
}

fn maybe() -> Option<u8> {
    let random = random::<u8>();

    match random % 2 == 0 {
        true => Some(random),
        false => None
    }
}