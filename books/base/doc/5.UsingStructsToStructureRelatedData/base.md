dgb!() take the ownership so when you dbg a stuce you will borrow it;

```rust
let x =5;
dbg!(x);
```

add trait to a struct 
```rust
#[derive(Debug)]
```
and then 
```rust
println!("{:?}",struct)
```

methods are similar to functions

defined within context of a stuct a enum or a trait object. their first parameter is always self, which represents the instance of the struct the method is being callled on.

## usage
start an 