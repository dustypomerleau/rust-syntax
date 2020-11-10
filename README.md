# Rust Syntax

This extension provides a TextMate grammar for Rust, with the following goals:

- Provide at least one scope target for each token.
- Where relevant, allow themes to distinguish declarations from invocations (function names, types).
- Make punctuation themeable. For symbols with multiple roles, distinguish punctuation use from operator use.
- Include appropriate child scopes in `meta` scopes (attributes, interpolation, `format!`).

This grammar is being evaluated in VS Code Nightly, and may become the default Rust grammar for VS Code in the future.
However, for the moment this extension is the most current version.
Feedback is ongoing, so the current scopes may change.

The grammar in [Rust Analyzer][] is temporarily being kept in sync with this one while we wait on a merge into VS Code Stable.
If you have an issue or PR, please submit it here.

## Contributing

The grammar is maintained as YAML, using tasks to generate JSON on save (please don't edit the JSON grammar directly).
You can regenerate the JSON manually from the command palette using `Tasks: Run Build Task`.

Along with your changes, please add (or modify) a [unit test][] with a brief code snippet that demonstrates the scopes affected by your PR.
Instructions for writing tests are available in @PanAeon's [vscode-tmgrammar-test][] repo.
You can run the tests from the command palette using `Tasks: Run Test Task`.

A PR with a failing test is the preferred way of filing an issue, as it ensures there is a code snippet to work on.

## Compatibility

Not all themes are specifically optimized for Rust.
We have tried to provide sensible default scopes that will work with most themes.
If you want to modify the colors in a particular theme, you can do so in your `settings.json`:

```json
"editor.tokenColorCustomizations": {
  "textMateRules": [
    {
      "scope": "variable.other.rust",
      "settings": {
        "foreground": "#c651e5"
      }
    }
  ]
}
```

The VS Code command `Developer: Inspect Editor Tokens and Scopes` will show you the scope stack at the current cursor position.

If you need a theme that works out of the box with this grammar, please check out [Yarra Valley][]:

![Yarra Valley](./images/rust.png)

[Rust Analyzer]: https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer
[unit test]: https://github.com/dustypomerleau/rust-syntax/tree/master/test/test.rs
[vscode-tmgrammar-test]: https://github.com/PanAeon/vscode-tmgrammar-test/blob/master/README.md
[Yarra Valley]: https://marketplace.visualstudio.com/items?itemName=dustypomerleau.yarra-valley
