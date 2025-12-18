fn main() {
    loop {
        println!("Please enter a number\nor quit to exit");

        let mut num = String::new();

        std::io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        if num.trim().eq_ignore_ascii_case("quit") {
            println!("Exiting...");
            break
        }

        let num: usize = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Computed fibonacci is {}", compute_fibb(num))
    }
    
}

fn compute_fibb(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut arr = vec![0; n + 1];
    arr[0] = 0;
    arr[1] = 1;
    for x in 2..=n {
        arr[x] = arr[x - 1] + arr[x - 2];
    }
    return arr[n];
}
