# LIFT OPS · Trainer

Phone-first industrial lift operations study bench  
(JLG boom lifts · rough-terrain telehandlers · forklifts)

**Not a certification.**  
OSHA requires formal instruction, practical evaluation on the specific machine, and a qualified evaluator  
(29 CFR 1910.178 for powered industrial trucks · 1926.453 for aerial lifts).  
This tool is a study aid only. The data plate, the operator manual, and your evaluator govern.

---

## What it is right now

- Single-file web trainer (`web/index.html`) that runs offline on your phone in the yard.
- Pre-use inspection checklists (boom + forklift) with CRITICAL items that must be checked before “READY”.
- Load-center capacity derate calculator + live stability-triangle visualization.
- Power-line minimum approach distance reference.
- Knowledge-check quiz with explanations.

## Design direction (SAGCO / BRICKS style)

```
content/          → pure data (YAML)
schema/           → formal shape of the knowledge
src/ast/          → KnowledgeNode, ChecklistNode, FlashCard, …
src/lexer/        → tokenizes training content
src/parser/       → builds the AST
src/enumerator/   → walks the tree into sequences
src/render/       → HTML · Flash (tachistoscope) · Terminal · future Agent
```

The current HTML is the first renderer.  
Later renderers (high-speed flash drills, Termux CLI, SAGCO agent prompts) will consume the same AST.

## Quick start (phone / Termux)

```bash
# just open the file
termux-open web/index.html
# or serve it
python -m http.server 8080 --directory web
```

## Rust path (optional)

```bash
cargo build
cargo run --bin trainer
```

The Rust side is currently a skeleton that compiles and is ready to grow the lexer → parser → AST pipeline.

## Packing

```bash
./scripts/pack.sh
# produces lift-ops-trainer-YYYYMMDD.tar.gz
```

## Safety notice

Regulatory numbers (MAD, capacity assumptions, fall-protection rules) change and vary by jurisdiction and site.  
Always verify against current OSHA text, the machine’s operator manual, and the data plate before operating.

---

Strategickhaos · 2026
