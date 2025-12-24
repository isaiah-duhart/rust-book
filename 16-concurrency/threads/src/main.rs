use std::thread;
use std::time::Duration;

fn main() {
    let vec = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            println!("printing vec {:?}", vec);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
