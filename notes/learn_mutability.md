# Mutability
## Variables
Variables are **immutable** by default. **Type annotation** is optional.
```rust
let immutable_var = String::from("Kenralp");

let immutable_var = String::from("Snowman");
```
</br>

The `mut` keyword makes it **mutable**.
```rust
let mut mutable_var = String::from("Kenralp");

mutable_var = String::from("Snowman");
```
</br>

## Constants
Constants are **immutable** by default. **Type annotation** is also required.
```rust
const CONST: i32 = 1600;

const CONST_EXPRESSION: i32 = ((12 + 12) * 30) + 100;
```

</br>
