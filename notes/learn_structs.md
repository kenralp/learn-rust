# Structs
## Members

</br>

## Access Modifiers

</br>

## Implementations
Functions can be **implemented to operate** on a `struct`. `Self` refers to the **current** `struct` the `impl` block is adding implementations to:
```rust
struct UserDefined {
    data1: i32,
    data2: i32
}

impl UserDefined {
    pub fn new(d1: i32, d2: i32) -> Self {
        Self {
            data1: d1,
            data2, d2
        }
    }
}
```
The `UserDefined::new(...)` function serves as the **constructor** for `UserDefined` for its **object instantiation**. The `UserDefined::drop(&mut self)` function serve as the **destructor** whenever the object **goes out of scope** or is explicitly freed:
```rust
// ...
impl Drop for UserDefined {
    fn drop(&mut self) {
        // Deinitialization code...
    }
}
```
If `UserDefined` manages a **heap-allocated** resource, the `Drop` trait can be **implemented** on `UserDefined` to create **deinitialization** of the data when it goes out of scope.

</br>

## Traits


## Generics
