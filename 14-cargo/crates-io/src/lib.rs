
//! Crates.io
//! 
//! `crates_io` is a collection of utilities that makes calculations easier
//! 

/// Adds one to the number given.
/// 
/// # Examples
/// ```
/// let arg = 5;
/// let answer = crates_io::add_one(arg);
/// 
/// assert_eq!(answer, 6);
/// ```
/// 
/// # Panics
/// 
/// # Errors
/// 
/// # Safety
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Subtracts one to the number given.
/// 
/// # Examples
/// ```
/// let arg = 5;
/// let answer = crates_io::subract_one(arg);
/// 
/// assert_eq!(answer, 4);
/// ```
/// 
/// # Panics
/// 
/// # Errors
/// 
/// # Safety
pub fn subtract_one(x: i32) -> i32 {
    x - 1
}
