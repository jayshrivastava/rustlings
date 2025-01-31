// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

// str is immutable and lives on the stack. String lives on the heap, like Vec.
fn current_favorite_color() -> String {
    String::from("blue")
}
