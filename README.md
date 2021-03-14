# Rust Syntax

This extension provides a TextMate grammar for Rust, with the following goals:

- Provide at least one scope target for each token.
- Where relevant, allow themes to distinguish declarations from invocations (function names, types).
- Make punctuation themeable. For symbols with multiple roles, distinguish punctuation use from operator use.
- Include appropriate child scopes in `meta` scopes (attributes, interpolation, `format!`).

This grammar has been upstreamed as the default Rust grammar for VS Code (issues and PRs should be submitted here).
It takes approximately 4 weeks for new changes to be scraped, so you'll need the extension if you want to see changes immediately.

## Compatibility

Not all themes are specifically optimized for Rust.
We have tried to provide sensible default scopes that will work with most themes.
If you want to modify the colors in a particular theme, you can do so in your `settings.json`:

```json
"editor.tokenColorCustomizations": {
    "[Theme Name]": {
        "textMateRules": [
            {
                "scope": "variable.other.rust",
                "settings": {
                    "foreground": "#ffff00"
                }
            }
        ]
    }
}
```

The VS Code command `Developer: Inspect Editor Tokens and Scopes` will show you the scope stack at the current cursor position.

Rust Syntax is compatible with [Rust Analyzer][], but the scopes provided by Rust Syntax will only be visible if you choose to disable semantic highlighting in your `settings.json`:

```json
"[rust]": {
    "editor.semanticHighlighting.enabled": false
}
```

## Contributing

The grammar is maintained as YAML, using tasks to generate JSON on save (please don't edit the JSON grammar directly).
You can regenerate the JSON manually from the command palette using `Tasks: Run Build Task`.

Tests have not yet been implemented for the grammar, but the framework is in place to do this when time allows.
If you submit new changes, please add (or modify) a [unit test][] with a brief code snippet that demonstrates the scopes affected by your PR.
Instructions for writing tests are available in @PanAeon's [vscode-tmgrammar-test][] repo.
You can run the tests from the command palette using `Tasks: Run Test Task`.

A PR with a failing test is the preferred way of filing an issue, as it ensures there is a code snippet to work on.

If you need a theme that works out of the box with this grammar, please check out [Tol][] or [Yarra Valley][].

[Rust Analyzer]: https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer
[Tol]: https://marketplace.visualstudio.com/items?itemName=dustypomerleau.tol
[unit test]: https://github.com/dustypomerleau/rust-syntax/blob/master/test/test.rs
[vscode-tmgrammar-test]: https://github.com/PanAeon/vscode-tmgrammar-test/blob/master/README.md
[Yarra Valley]: https://marketplace.visualstudio.com/items?itemName=dustypomerleau.yarra-valley
