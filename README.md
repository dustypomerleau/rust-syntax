# Rust Syntax

This extension provides a TextMate-style grammar for Rust, with the following goals:

- Every item on the page should have at least one scope for theme designers to target.
- It should be possible to visually distinguish declarations and invocations (function names, types).
- All punctuation should be themeable.
- Punctuation should be separated from its contents (example: quotation marks can be themed separately from strings).
- Some punctuation can be themed differently depending on context (example: angled brackets in types versus comparison operators).
- When appropriate, meta scopes should allow other scopes to shine through (examples: including strings in attributes, string interpolation, `format!`).

Rust Syntax may be used in conjunction with [The Rust Programming Language][] extension.

This extension is under active development, and there may be changes to the existing scopes. Relevant PRs include:

- [VS Code](https://github.com/microsoft/vscode/pull/108254)
- [Rust Analyzer](https://github.com/rust-analyzer/rust-analyzer/pull/6137)

If you need a theme that works out of the box with this grammar, please check out [Yarra Valley][]:

![Yarra Valley](./images/rust.png)

[The Rust Programming Language]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust
[Yarra Valley]: https://marketplace.visualstudio.com/items?itemName=dustypomerleau.yarra-valley
