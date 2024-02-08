mod core;
mod ext;
mod models;
mod utils;

pub use self::core::*;
pub use self::ext::*;
pub use self::models::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
