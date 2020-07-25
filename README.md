# `bstringify!`

> **Like `stringify!`, but yielding byte string literals instead.**

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](https://github.com/danielhenrymantilla/rust-bstringify)
[![Latest version](https://img.shields.io/crates/v/bstringify.svg)](https://crates.io/crates/bstringify)
[![Documentation](https://docs.rs/bstringify/badge.svg)](https://docs.rs/bstringify)
[![License](https://img.shields.io/crates/l/bstringify.svg)](https://github.com/danielhenrymantilla/rust-bstringify/blob/master/LICENSE-ZLIB)
![CI](https://github.com/danielhenrymantilla/rust-bstringify/workflows/CI/badge.svg)


Since `.as_bytes()` on `str`ings is a `const fn`, this macro should only be
useful to create a byte string literal to `match` against:

```rust
use ::bstringify::bstringify;

/// like FromStr but with [u8] instead
trait FromBytes : Sized {
    fn from_bytes (input: &'_ [u8])
      -> Option<Self>
    ;
}

macro_rules! derive_FromBytes {(
    $(#[$attr:meta])*
    $pub:vis
    enum $EnumName:ident {
        $(
            $Variant:ident
        ),* $(,)?
    }
) => (
    $(#[$attr])*
    $pub
    enum $EnumName {
        $(
            $Variant,
        )*
    }
    
    impl $crate::FromBytes
        for $EnumName
    {
        fn from_bytes (input: &'_ [u8])
          -> Option<Self>
        {
            match input {
            $(
                | bstringify!($Variant) => Some(Self::$Variant),
            )*
                | _ => None,
            }
        }
    }
)}

derive_FromBytes! {
    enum Example {
        Foo,
        Bar,
    }
}

fn main ()
{
    assert!(matches!(
        Example::from_bytes(b"Foo"), Some(Example::Foo)
    ));
    assert!(matches!(
        Example::from_bytes(b"Bar"), Some(Example::Bar)
    ));
    assert!(matches!(
        Example::from_bytes(b"Bad"), None
    ));
}
```
