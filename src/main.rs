fn main() {
    println!("Hello, world!");
    let x: i32 = 1;
    let y: i32 = 1;
    println!("x = {x}");
    println!("y = {y}");
    println!("x + y = {}", add(x, y));
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}