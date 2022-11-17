# Enums and Pattern Matching



catch-all pattern and the _ placeholder
```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```