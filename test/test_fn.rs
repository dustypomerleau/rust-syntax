// SYNTAX TEST "source.rust" "Textmate grammar scope tests for functions"

fn my_function() {}
// <--                      keyword.other.fn.rust
// ^^^^^^^^^^^              entity.name.function.rust
//            ^^            punctuation.brackets.round.rust
//               ^^         punctuation.brackets.curly.rust
// <---------------         meta.function.definition.rust

unsafe fn my_function() {}
// <------                  keyword.other.safety.rust
//     ^^                   keyword.other.fn.rust
//        ^^^^^^^^^^^       entity.name.function.rust
//                   ^^     punctuation.brackets.round.rust
//                      ^^  punctuation.brackets.curly.rust
// <----------------------  meta.function.definition.rust

safe fn my_function();
// <----                    keyword.other.safety.rust
//   ^^                     keyword.other.fn.rust
//      ^^^^^^^^^^^         entity.name.function.rust
//                 ^^       punctuation.brackets.round.rust
//                   ^      punctuation.semi.rust
// <--------------------    meta.function.definition.rust
