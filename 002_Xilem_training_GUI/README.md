- https://github.com/linebender/xilem

<hr />

# toml Fmt

- 설치 https://taplo.tamasfe.dev/cli/installation/binary.html
- Taplo예시 https://github.com/linebender/xilem
  - `.taplo.toml`

```bash
taplo format Cargo.toml
```

- `.taplo.toml`

```toml
# See https://taplo.tamasfe.dev/configuration/file.html
# and https://taplo.tamasfe.dev/configuration/formatter-options.html

[formatting]
# Aligning comments with the largest line creates
# diff noise when neighboring lines are changed.
align_comments = false

# Matches how rustfmt formats Rust code
column_width = 100
indent_string = "    "
```

