use std::collections::HashMap;
use std::io;
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut directory: HashMap<String, Vec<String>> = HashMap::new();
    help();

    loop {
        let mut request = String::new();
        let Ok(_) = io::stdin().read_line(&mut request) else {
            println!("Error reading line");
            continue;
        };

        let mut request = request.split_ascii_whitespace();
        let Some(command) = request.next() else {
            println!("Request is not valid");
            continue
        };

        match command.to_ascii_lowercase().as_str() {
            "add" => add(&mut request, &mut directory),
            "print" => println!("{:?}", directory),
            "quit" => break,
            "help" => help(),
            _ => println!("{} is an invalid command", command)
        }
    }
}

fn add(request: &mut SplitAsciiWhitespace, directory: &mut HashMap<String, Vec<String>>) {
    let Some(employee) = request.next() else {
        println!("Employee not found");
        return;
    };

    println!("Processing employee {}...", employee);

    let Some(to) = request.next() else {
        println!("'to' not found");
        return;
    };

    if !to.eq_ignore_ascii_case("to") {
        println!("'to' not found");
        return;
    }

    let Some(department) = request.next() else {
        println!("Department not found");
        return;
    };

    println!("Adding employee {} to department {}", employee, department);

    let vec = directory.entry(String::from(department)).or_insert(Vec::new());
    vec.push(String::from(employee));
}

fn help() {
    println!("Enter add request in format: add <employee> to <department>");
    println!("To print directory enter: print");
    println!("To quit enter: quit");
    println!("To display this message: help");
}
