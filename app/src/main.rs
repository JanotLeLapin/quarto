use quarto_core::Stack;

pub fn main() {
    println!("hello, world!");

    let stack = Stack::new();

    for piece in stack.0 {
        let piece = piece.unwrap();
        println!(
            "{}: bright = {}, square = {}, tall = {}, hollow = {}",
            piece,
            piece.is_bright(),
            piece.is_square(),
            piece.is_tall(),
            piece.is_hollow()
        );
    }
}
