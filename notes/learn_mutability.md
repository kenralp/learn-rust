# Mutability
### Variables
The `let` keyword defines an **immutable** variable; Its value cannot be **mutated** nor **moved** to a new variable, but it can be **redefined** (or **shadowed**) which creates a new immutable variable with the **same name**:
```rust
let username = String::from("Kenralp");

// username = String::from("THIS LINE WILL FAIL!");

let username = String::from("Snowman");
```
</br>

Appending `mut` makes the variable **mutable**:
```rust
let mut username = String::from("Kenralp");

username = String::from("Snowman");
```
</br>

**Type annotations** are **optional** for variables (with some _**exceptions**_). Do not use type annotations when defining variables that calls a **method** (like `new()` or `from()`) as it requires the **type name** to be called, making the type **obvious** already (for good practice):
```rust
let username = String::new();

let username: String = String::from("NO NEED FOR TYPE ANNOTATION!");
```
</br>

For **numeric types**, a **type suffix** can be appended:
```rust
let i32_variable = 1600i32;

let i64_variable = 1_000_000_000_000_000_000i64;

let f64_variable = 1888.23332f64;
```

</br>

### Constants and Constant Expressions
The `const` keyword defines a **constant** value; Its value is **immutable** and it requires **type annotation**. It can also be used for **constant expressions**:
```rust
const MY_CONSTANT: i32 = 7_899_999_120;

const MY_CONST_EXPR: i32 = ((12 + 24) * 2) + 100;
```
</br>
