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

<hr />

# 알아야할 크레이터 정리
- Font introspection, complex text shaping and glyph rendering.
  - 글꼴 성찰, 복잡한 텍스트 형태 및 글리프 렌더링.
  - https://github.com/dfrg/swash

- Rich text layout library(Parley provides an API for implementing rich text layout. It is backed by Swash.)
  - 리치 텍스트 레이아웃 라이브러리(Parley는 리치 텍스트 레이아웃을 구현하기 위한 API를 제공합니다. Swash가 지원합니다.)
  - https://github.com/linebender/parley
    - The Parley text stack
      - Currently, Parley directly depends on
      - four crates: Fontique, Swash, Skrifa, and Peniko.
        - These crates cover different pieces of the text-rendering process.

