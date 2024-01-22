// SYNTAX TEST "source.rust" "Textmate grammar scope tests for line comment"

// This file is a placeholder stub that will gradually be expanded to unit test all scopes.
// Instructions for writing Textmate grammar tests can be found at:
// https://github.com/PanAeon/vscode-tmgrammar-test/blob/master/README.md

    // some comment content
//  ^^                          punctuation.definition.comment.rust
//  ^^^^^^^^^^^^^^^^^^^^^^^     comment.line.double-slash.rust
//  ^^^^^^^^^^^^^^^^^^^^^^^     source.rust
// <----                        source.rust

    /// some comment content
//  ^^^                         punctuation.definition.comment.rust
//  ^^^^^^^^^^^^^^^^^^^^^^^^    comment.line.documentation.rust
//  ^^^^^^^^^^^^^^^^^^^^^^^^    source.rust
// <----                        source.rust


