# rustoleum 🦀

`rustoleum` is a collection of experimental Rust macros and utilities designed to bypass standard safety checks and provide low-level memory manipulation shortcuts. It is intended for educational purposes and "forbidden" techniques.

## Core Features

- **Safety Bypasses**: Macros like `safe!` and `uprintln!` for wrapping unsafe operations in a more concise syntax.
- **Pointer & Reference Utilities**:
  - `raw!($x)`: Get a constant pointer to a variable.
  - `maw!($x)`: Get a mutable pointer to a variable.
  - `unref!($x)`: Dereference a pointer unsafely.
- **Lifetime Transmutation**: Functions and macros (`stat!`, `mstat!`, `leak`) to transmute references into `'static` lifetimes or leak boxed values.
- **Global State Management**: Macros for defining and updating static and static mutable variables (`set!`, `sut!`, `put!`).
- **Borrow Checker "Techniques"**: Includes integration with `you_can::turn_off_the_borrow_checker` for unchecked reference handling.

## Available Macros

- `safe!`: Wraps an expression in an `unsafe` block.
- `uprintln!`: Unsafe version of `println!`.
- `met! / mef!`: Macros for `let mut` and `ref mut` bindings.
- `at!`: Unsafe array/slice indexing.
- `raw! / maw!`: Pointer creation shortcuts.
- `stat! / mstat!`: Lifetime transmutation to `'static`.

## Warning

This library makes extensive use of `unsafe` and is designed to deliberately ignore Rust's safety guarantees. Use with extreme caution and only for experimental purposes.

## Usage Example

```rust
use rustoleum::prelude::*;

fn main() {
  let x = 10;
  let ptr = raw!(x);
  
  // Unsafely dereference
  let val = unref!(ptr);
  uprintln!("Value from pointer: {}", val);
  
  // Transmute to static
  let s: &'static i32 = stat!(&x);
}
```

`rustoleum` is a collection of experimental Rust macros and utilities designed to bypass standard safety checks and provide low-level memory manipulation shortcuts. It is intended for educational purposes and "forbidden" techniques.

## Core Features

- **Safety Bypasses**: Macros like `safe!` and `uprintln!` for wrapping unsafe operations in a more concise syntax.
- **Pointer & Reference Utilities**:
  - `raw!($x)`: Get a constant pointer to a variable.
  - `maw!($x)`: Get a mutable pointer to a variable.
  - `unref!($x)`: Dereference a pointer unsafely.
- **Lifetime Transmutation**: Functions and macros (`stat!`, `mstat!`, `leak`) to transmute references into `'static` lifetimes or leak boxed values.
- **Global State Management**: Macros for defining and updating static and static mutable variables (`set!`, `sut!`, `put!`).
- **Borrow Checker "Techniques"**: Includes integration with `you_can::turn_off_the_borrow_checker` for unchecked reference handling.

## Available Macros

- `safe!`: Wraps an expression in an `unsafe` block.
- `uprintln!`: Unsafe version of `println!`.
- `met! / mef!`: Macros for `let mut` and `ref mut` bindings.
- `at!`: Unsafe array/slice indexing.
- `raw! / maw!`: Pointer creation shortcuts.
- `stat! / mstat!`: Lifetime transmutation to `'static`.

## Warning

This library makes extensive use of `unsafe` and is designed to deliberately ignore Rust's safety guarantees. Use with extreme caution and only for experimental purposes.

## Usage Example

```rust
use rustoleum::prelude::*;

fn main() {
  let x = 10;
  let ptr = raw!(x);
  
  // Unsafely dereference
  let val = unref!(ptr);
  uprintln!("Value from pointer: {}", val);
  
  // Transmute to static
  let s: &'static i32 = stat!(&x);
}
```


- **src/**: Contains the core library implementation.
- **Cargo.toml**: Dependency and project configuration.

## Usage

Add `rustoleum` to your `Cargo.toml`:

```toml
[dependencies]
rustoleum = { git = "https://github.com/Patrick-ring-motive/rustoleum" }
```

Then use it in your code:

```rust
use rustoleum::prelude::*;

fn main() {
    // Example usage of rustoleum utilities
    let result = rustoleum::do_something_fast();
    println!("Result: {:?}", result);
}
```
