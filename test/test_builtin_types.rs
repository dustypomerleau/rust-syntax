// SYNTAX TEST "source.rust" "Built-in type highlighting"

    let numeric: u32 = 42u32;
//               ^^^ storage.type.numeric.rust
//                       ^^^ storage.type.numeric.rust
//               ^^^ - entity.name.type.numeric.rust

    let floating: f64 = 1.0f64;
//                ^^^ storage.type.numeric.rust
//                         ^^^ storage.type.numeric.rust

    let boolean: bool = true;
//               ^^^^ storage.type.primitive.rust
//               ^^^^ - entity.name.type.primitive.rust

    let character: char = 'x';
//                 ^^^^ storage.type.primitive.rust

    let borrowed: &str = "text";
//                 ^^^ storage.type.primitive.rust

    let hexadecimal = 0xffu8;
//                        ^^ storage.type.numeric.rust
    let octal = 0o77usize;
//                  ^^^^^ storage.type.numeric.rust
    let binary = 0b1010i16;
//                     ^^^ storage.type.numeric.rust

    struct UserType;
//         ^^^^^^^^ entity.name.type.struct.rust
