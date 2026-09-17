// SYNTAX TEST "source.rust" "Unicode escapes accept 1-6 hex digits"

    let one_digit = "\u{a}";
//                   ^^^^^ constant.character.escape.rust
//                     ^^^ constant.character.escape.unicode.rust - meta.interpolation.rust

    let two_digits = "\u{7f}";
//                    ^^^^^^ constant.character.escape.rust
//                      ^^^^ constant.character.escape.unicode.rust - meta.interpolation.rust

    let four_digits = "\u{FFFF}";
//                     ^^^^^^^^ constant.character.escape.rust
//                       ^^^^^^ constant.character.escape.unicode.rust

    let six_digits = "\u{10FFFF}";
//                    ^^^^^^^^^^ constant.character.escape.rust
//                      ^^^^^^^^ constant.character.escape.unicode.rust

    let underscores = "\u{1_F600}";
//                     ^^^^^^^^^^ constant.character.escape.rust
//                       ^^^^^^^^ constant.character.escape.unicode.rust - meta.interpolation.rust

    let in_a_char = '\u{0}';
//                   ^^^^^ constant.character.escape.rust
//                     ^^^ constant.character.escape.unicode.rust

    let braces = "\u{9}";
//                  ^ constant.character.escape.unicode.punctuation.rust
//                    ^ constant.character.escape.unicode.punctuation.rust
