use std::{cmp::PartialOrd, fmt::Display};

fn main() {
    no_generics();
    generics();
}

fn no_generics() {
    let numbers = vec![20, 30, 240, 245, 242452];
    let largest = no_generics_largest_i32(&numbers);
    println!("Largest number is {largest}");

    let numbers = vec!['a', 'y', 'x', 't', 'e', 'v', 'q', 'l'];
    let largest = no_generics_largest_char(&numbers);
    println!("Largest letter is {largest}");
}

fn generics() {
    let numbers = vec![20, 30, 240, 245, 242452];
    let largest_num = largest(&numbers);
    println!("Largest number is {largest_num}");

    let chars = vec!['a', 'y', 'x', 't', 'e', 'v', 'q', 'l'];
    let largest_char = largest(&chars);
    println!("Largest letter is {largest_char}"); 
    }


fn no_generics_largest_i32(numbers: &[i32]) -> &i32 {
    let mut largest = &numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    };

    largest
}

fn no_generics_largest_char(numbers: &[char]) -> &char {
    let mut largest = &numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    };

    largest
}

fn largest<T: PartialOrd + Display>(vec: &[T]) -> &T {
    let mut largest = &vec[0];
    for elem in vec {
        if elem > largest {
            largest = elem;
        }
    };

    largest
}
