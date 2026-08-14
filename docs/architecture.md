# Architecture

## Pipeline

```
content/*.yaml
    ↓
parser (serde_yaml)
    ↓
AST (Document enum)
    ↓
enumerator (sequences)
    ↓
renderers
    ├── web/index.html   (current phone deliverable)
    ├── flash            (tachistoscope / RSVP)
    ├── terminal         (future Termux)
    └── agent            (future SAGCO prompt emitter)
```

## Why this shape

Matches the SAGCO / BRICKS pattern:

- Knowledge is data, not hard-coded UI.
- One AST, many views.
- Lexer + parser extension points for a future training DSL or FlameLang surface.
- Packable as a clean `tar.gz` that can be uploaded straight to the GitHub repo.

## Current status

- HTML trainer is fully usable today (`web/index.html`).
- Rust side compiles and can load YAML → AST → print critical items.
- Flash renderer skeleton is present; high-speed presentation will be wired next.
