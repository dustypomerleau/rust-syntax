// SYNTAX TEST "source.rust" "Textmate grammar scope tests"

// This file is a placeholder stub that will gradually be expanded to unit test all scopes.
// Instructions for writing Textmate grammar tests can be found at:
// https://github.com/PanAeon/vscode-tmgrammar-test/blob/master/README.md

// example test
extern crate std;
// <------ meta.import.rust storage.type.rust
//     ^^^^^ meta.import.rust keyword.other.crate.rust
//           ^^^ meta.import.rust
//              ^ meta.import.rust punctuation.semi.rust

// decimal floats test
let x1 = 1.123e12;
//       ^^^^^^^^ constant.numeric.decimal.rust
//       ^^^^^ constant.numeric.decimal.exponent.mantissa.rust
//        ^ punctuation.separator.dot.decimal.rust
//            ^ keyword.operator.exponent.rust
let x2 = 1.123e+12;
//       ^^^^^^^^^ constant.numeric.decimal.rust
//       ^^^^^ constant.numeric.decimal.exponent.mantissa.rust
//        ^ punctuation.separator.dot.decimal.rust
//            ^ keyword.operator.exponent.rust
//             ^ keyword.operator.exponent.sign.rust
let x3 = 1.123e-12;
//       ^^^^^^^^^ constant.numeric.decimal.rust
//       ^^^^^ constant.numeric.decimal.exponent.mantissa.rust
//        ^ punctuation.separator.dot.decimal.rust
//            ^ keyword.operator.exponent.rust
//             ^ keyword.operator.exponent.sign.rust
let x4 = 1.123E12;
//       ^^^^^^^^ constant.numeric.decimal.rust
//       ^^^^^ constant.numeric.decimal.exponent.mantissa.rust
//        ^ punctuation.separator.dot.decimal.rust
//            ^ keyword.operator.exponent.rust
let x5 = 1.123E+12;
//       ^^^^^^^^^ constant.numeric.decimal.rust
//       ^^^^^ constant.numeric.decimal.exponent.mantissa.rust
//        ^ punctuation.separator.dot.decimal.rust
//            ^ keyword.operator.exponent.rust
//             ^ keyword.operator.exponent.sign.rust
let x6 = 1.123E-12;
//       ^^^^^^^^^ constant.numeric.decimal.rust
//       ^^^^^ constant.numeric.decimal.exponent.mantissa.rust
//        ^ punctuation.separator.dot.decimal.rust
//            ^ keyword.operator.exponent.rust
//             ^ keyword.operator.exponent.sign.rust

// decimal literals without exponents
let x7 = 42usize;
//       ^^^^^^^ constant.numeric.decimal.rust
//       ^^^^^^^ - constant.numeric.decimal.exponent.mantissa.rust
//       ^^^^^^^ - keyword.operator.exponent.rust
//         ^^^^^ storage.type.numeric.rust
let x8 = 1.25;
//       ^^^^ constant.numeric.decimal.rust
//       ^^^^ - constant.numeric.decimal.exponent.mantissa.rust
//        ^ punctuation.separator.dot.decimal.rust
let x9 = 42f32;
//       ^^^^^ constant.numeric.decimal.rust
//       ^^^^^ - constant.numeric.decimal.exponent.mantissa.rust
//         ^^^ storage.type.numeric.rust

// decimal literals with exponents
let x10 = 42e2;
//        ^^^^ constant.numeric.decimal.rust
//        ^^ constant.numeric.decimal.exponent.mantissa.rust
//          ^ keyword.operator.exponent.rust
let x11 = 1.25e-4f64;
//        ^^^^^^^^^^ constant.numeric.decimal.rust
//        ^^^^ constant.numeric.decimal.exponent.mantissa.rust
//         ^ punctuation.separator.dot.decimal.rust
//            ^ keyword.operator.exponent.rust
//             ^ keyword.operator.exponent.sign.rust
//               ^^^ storage.type.numeric.rust

// macro metavarables
macro_rules! metavariable_test {
    ($var:tt $Type:ty $var : tt $Type :ty) => {};
//   ^^^^^^^          ^^^^^^^^^             meta.macro.metavariable.rust
//           ^^^^^^^^           ^^^^^^^^^   meta.macro.metavariable.type.rust
//   ^       ^        ^         ^           keyword.operator.macro.dollar.rust
//    ^^^              ^^^                  variable.other.metavariable.name.rust
//            ^^^^               ^^^^       entity.name.type.metavariable.rust
//       ^        ^        ^          ^     keyword.operator.key-value.rust
//        ^^       ^^        ^^        ^^   variable.other.metavariable.specifier.rust
    ($var:pat_param $Var:pat_param) => {};
//   ^^^^^^^^^^^^^^                         meta.macro.metavariable.rust
//                  ^^^^^^^^^^^^^^          meta.macro.metavariable.type.rust
//   ^              ^                       keyword.operator.macro.dollar.rust
//    ^^^                                   variable.other.metavariable.name.rust
//                   ^^^                    entity.name.type.metavariable.rust
//       ^              ^                   keyword.operator.key-value.rust
//        ^^^^^^^^^      ^^^^^^^^^          variable.other.metavariable.specifier.rust
    ($var: expr_2021 $Var: expr_2021) => {};
//   ^^^^^^^^^^^^^^^                        meta.macro.metavariable.rust
//                   ^^^^^^^^^^^^^^^        meta.macro.metavariable.type.rust
//   ^               ^                      keyword.operator.macro.dollar.rust
//    ^^^                                   variable.other.metavariable.name.rust
//                    ^^^                   entity.name.type.metavariable.rust
//       ^               ^                  keyword.operator.key-value.rust
//         ^^^^^^^^^       ^^^^^^^^^        variable.other.metavariable.specifier.rust
    () => { $var $Type $crate };
//          ^^^^                            meta.macro.metavariable.rust
//               ^^^^^ ^^^^^^               meta.macro.metavariable.type.rust
//              ^     ^      ^              - meta.macro.metavariable.rust meta.macro.metavariable.type.rust
//          ^    ^     ^                    keyword.operator.macro.dollar.rust
//           ^^^                            variable.other.metavariable.name.rust
//                ^^^^                      entity.name.type.metavariable.rust
//                      ^^^^^               keyword.other.crate.rust
    () => { $var: not_a_specifier };
//          ^^^^                            meta.macro.metavariable.rust
//          ^                               keyword.operator.macro.dollar.rust
//           ^^^                            variable.other.metavariable.name.rust
//              ^^^^^^^^^^^^^^^^^           - meta.macro.metavariable.rust

    pub fn testPubModifier__01() {
//  ^^^ storage.modifier.visibility.rust
        let publish = 1;
//          ^^^^^^^ - storage.modifier.visibility.rust
        let some_str = "pub";
//                      ^^^ - storage.modifier.visibility.rust
    };

    pub(crate) fn testPubModifier__02() {}
//  ^^^ storage.modifier.visibility.rust
}
