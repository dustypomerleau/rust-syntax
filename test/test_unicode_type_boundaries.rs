// SYNTAX TEST "source.rust" "A non-ASCII letter is not a word boundary before a built-in type"

    let äu32 = 1;
//      ^^^^ variable.other.rust - storage.type.numeric.rust

    let straße_f64 = 2;
//      ^^^^^^^^^^ variable.other.rust - storage.type.numeric.rust

    let x = 1u32;
//           ^^^ storage.type.numeric.rust
