// SYNTAX TEST "source.rust" "Non-ASCII identifiers"

    fn grüße() {}
//     ^^^^^ entity.name.function.rust

    grüßen();
//  ^^^^^^ entity.name.function.rust

    struct Über;
//         ^^^^ entity.name.type.struct.rust

    trait Größe {}
//        ^^^^^ entity.name.type.trait.rust

    enum Färbe {}
//       ^^^^^ entity.name.type.enum.rust

    type Straße = u32;
//       ^^^^^^ entity.name.type.declaration.rust

    mod schön;
//      ^^^^^ entity.name.module.rust

    let café = 1;
//      ^^^^ variable.other.rust

    let x = Übersicht::neu();
//          ^^^^^^^^^ entity.name.type.rust
//                     ^^^ entity.name.function.rust

    let b: Behälter<u32> = x;
//         ^^^^^^^^ entity.name.type.rust

    grüß!("x");
//  ^^^^^ entity.name.function.macro.rust

    macro_rules! begrüßung { () => {} }
//               ^^^^^^^^^ entity.name.function.macro.rust

    const ΜΈΓΕΘΟΣ: u32 = 1;
//        ^^^^^^^ constant.other.caps.rust

    let m = MAXIMALGRÖSSE;
//          ^^^^^^^^^^^^^ constant.other.caps.rust

    use crate::модуль::Тип;
//             ^^^^^^ entity.name.namespace.rust
//                     ^^^ entity.name.type.rust

    struct 日本語;
//         ^^^ entity.name.type.struct.rust

    fn 挨拶() {}
//     ^^ entity.name.function.rust

    let 変数 = 1;
//      ^^ variable.other.rust
