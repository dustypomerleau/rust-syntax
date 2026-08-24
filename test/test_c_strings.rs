// SYNTAX TEST "source.rust" "C string highlighting"

    let regular = c"hello\xff";
//                ^^^^^^^^^^^^ string.quoted.double.c.rust
//                       ^^^^ constant.character.escape.rust
    let raw = cr#"hello"#;
//            ^^^^^^^^^^^ string.quoted.double.c.raw.rust
