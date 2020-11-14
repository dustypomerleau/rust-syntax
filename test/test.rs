// SYNTAX TEST "source.rust" "Textmate grammar scope tests"

// PLEASE NOTE: My apologies, but we are blocked on testing, due to an inability of the test platform to process some of the lookbehinds in the grammar. VS Code has no issue with them, but the test engine cannot create the Oniguruma scanner needed to run the tests.

// This file is a placeholder stub that will gradually be expanded to unit test all scopes.
// Instructions for writing Textmate grammar tests can be found at:
// https://github.com/PanAeon/vscode-tmgrammar-test/blob/master/README.md

// example test
extern crate std;
// <------ meta.import.rust storage.type.rust
//     ^^^^^ meta.import.rust keyword.other.crate.rust
//           ^^^ meta.import.rust
//              ^ meta.import.rust punctuation.semi.rust
