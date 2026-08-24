// SYNTAX TEST "source.rust" "f16 and f128 highlighting"

    let half = 1.5f16;
//             ^^^^^^ constant.numeric.decimal.rust
//                ^^^ storage.type.numeric.rust
    let quad: f128 = 1f128;
//            ^^^^ storage.type.numeric.rust
//                   ^^^^^ constant.numeric.decimal.rust
