# Bitx: A Crate for Bit-level Manipulation

`bitx` is a lightweight and efficient Rust crate designed for handling and manipulating bit-level data. 
It provides an intuitive API that allows direct access and modification of individual bits using bit indexing.

# Features
- Bit-level indexing: Access and modify individual bits with ease.
- Efficient storage: Compact storage for large amounts of bit data.


# Example

```rust
use bitx::Bitx;
fn main() {
    let mut byte: Bitx<8> = 0b111u8.into();
    byte[1] = false;
    assert_eq!(byte.value(), 0b101)
}
```


# License
This project is licensed under the MIT License.