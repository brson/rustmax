Types in which the bits are individually addressable.

- Crate [`::bitflags`].
- [docs.rs](https://docs.rs/bitflags)
- [cargo.io](https://crates.io/crates/bitflags)
- [GitHub](todo)

---

Rust does not natively support bitwise field access as some languages do.
This crate provides the `bitflags!` macro which defines typesafe bitmasks,
types with named values that are efficiently packed together as bits
to express sets of options.

```rust
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    struct Flag: u32 {
        const A       = 0b00000001;
        const B       = 0b00000010;
        const C       = 0b00000100;
        const ABC     = Flag::A.bits()
                        | Flag::B.bits()
                        | Flag::C.bits();
    }
}

fn main() {
    let e1 = Flag::A | Flag::C;
    let e2 = Flag::B | Flag::C;
    assert_eq!((e1 | e2), Flag::ABC);   // union
    assert_eq!((e1 & e2), Flag::C);     // intersection
    assert_eq!((e1 - e2), Flag::A);     // set difference
    assert_eq!(!e2, Flag::A);           // set complement
}
```