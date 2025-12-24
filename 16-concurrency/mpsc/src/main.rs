use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello from thread!"),
            String::from("Second message!"),
            String::from("Third message!"),
            String::from("Fourth message!"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello from thread 2!"),
            String::from("Second message 2!"),
            String::from("Third message 2!"),
            String::from("Fourth message 2!"),
        ];

        for val in vals {
            tx.send(val).unwrap();
        }
    });
    for val in rx {
        println!("{val}");
    } 
}
