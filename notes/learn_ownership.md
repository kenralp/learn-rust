# Ownership
## Copying
Any variable whose type implements the `Copy` trait can be **copied**:
```rust
let original: i64 = 1_000_000;

let copy = original;
// 'original' is still valid after
```
</br>

## Moving
Any variable whose type stores **heap-allocated** resources and does not implement the `Copy` trait are **moved** instead:
```rust
let original = String::from("Kenralp");

let move: String = original;
// 'original' is invalid from here on out
```
</br>

## Borrowing
Any function parameter declared with `&` is a **non-owning reference** and is **borrowed**:
```rust
fn borrow(param :&i32) {
    // ...
}
```
