// SYNTAX TEST "source.rust" "Bitwise operator scopes"

    let bits = a ^ b | c << 1 >> 2;
//               ^   ^   ^^   ^^ keyword.operator.bitwise.rust
    let boolean = a && b || !c;
//                  ^^   ^^ ^ keyword.operator.logical.rust
