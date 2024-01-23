# Rust Syntax

This extension provides a TextMate grammar for Rust. This grammar is
used for VS Code's built-in Rust syntax highlighting
([source](https://github.com/microsoft/vscode/blob/main/extensions/rust/syntaxes/rust.tmLanguage.json)).

Issues and PRs should be submitted here, and [VS Code syncs this repo regularly](https://github.com/microsoft/vscode/pull/194758#issuecomment-1748526176).

The semantic highlighting provided by [Rust
Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
is more precise than this grammar. For example, semantic highlighting can easily
distinguish enums, structs, and traits.

For best results, install Rust Analyzer to benefit from both.

Rust Syntax is compatible with Rust Analyzer, but the scopes provided
by this extension will not be visible while semantic highlighting is
enabled. If for some reason you would like to disable semantic
highlighting, you can do this in your `settings.json`:

```json
"[rust]": {
    "editor.semanticHighlighting.enabled": false
}
```

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

## Contributing

The grammar is maintained as YAML, using tasks to generate JSON on save (please don't edit the JSON grammar directly).

```sh
npm install

# Watch for changes of YAML files and regenerate JSON
npm start

# Run tests
npm test
```

If you are using VS Code, you can use the `Tasks: Run Build Task` command from the command palette to run the gulp task. And you can use the `Tasks: Run Test Task` command to run the tests.
