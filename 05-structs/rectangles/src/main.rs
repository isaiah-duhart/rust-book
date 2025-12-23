#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    name: String
}

impl Rectangle {
    fn method_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
 
    fn max(self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle { 
            width: w,
            height: h,
            name: String::from("max")
        }
    }

    fn set_to_max(self, other: Self) -> Self {
        let max = self.max(other);
        max
    }
}

fn main() {
    let r1 = Rectangle { 
        width: 9, 
        height: 9, 
        name: String::from("r1") 
    };
    let r2 = Rectangle {
        width: 16,
        height: 16,
        name: String::from("r2")
    };

    r1.set_to_max(r2);
}

// fn main() {
//     let width = 50;
//     let height = 60;

//     println!("The area of this rectangle is {}", area(width, height));

//     let dimension = (50, 60);

//     println!("The area of this tuple rectangle is {}", tuple_area(dimension));

//     let rect = Rectangle {
//         width: 50,
//         height: 60
//     };

//     println!("The area of this struct rectangle is {}", struct_area(&rect));
//     println!("The area of this struct method rectangle is {}", rect.method_area());

//     println!("{rect:?}");

//     let rect1 = Rectangle {
//         width: 40,
//         height: 50
//     };
//     let rect2 = Rectangle {
//         width: 50,
//         height: 60
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 70
//     };

//     println!("rect can hold rect1: {}", rect.can_hold(&rect1));
//     println!("rect can hold rect2: {}", rect.can_hold(&rect2));
//     println!("rect can hold rect3: {}", rect.can_hold(&rect3));

//     println!("{rect:?}");

//     let square = Rectangle::square(20);
//     println!("{square:?}");
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn tuple_area(dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }

// fn struct_area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
