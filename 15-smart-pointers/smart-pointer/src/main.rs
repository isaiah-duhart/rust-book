
#[derive(Debug)]
struct MySmartPointer {
    data: String
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer with data {}", self.data)
    }
}
fn main() {
    let a = MySmartPointer {
        data: String::from("First string")
    };

    let b = MySmartPointer {
        data: String::from("Second string")
    };
    println!("{:?} {:?}", a, b);
}
