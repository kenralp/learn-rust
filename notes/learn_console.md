# Console
The `println!` macro prints `"Hello from learn-rust!"` to the console with a **newline** at the end:
```rust
fn main() {
    println!("Hello from learn-rust!");
}
```
</br>

The `stdin()` function from module `std::io` initializes console input. `read_line(&mut input_buffer)` reads input from console until the user clicks `Enter` or `Return` (in macOS) on their keyboard and stores it in `input_buffer`:
```rust
use std::io;

fn main() {
    println!("Enter your input:");

    let mut input_buffer = String::new();

    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read line!");

    println!("You entered: {}, input_buffer");
}
```
</br>
