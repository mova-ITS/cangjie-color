# cangjie-color — LEARN (Rust teaching spine)

Companion to `AGENTS.md`. **This file locks how we build the Rust tool.**  
Do not invent a parallel “mock-only” track unless the user reopens it.

---

## Concept (one sentence)

Teaching aids (as SVG) that show curated Cangjie letter→stroke colouring on a character — not inventing the code.

---

## Real product flow (locked)

```text
2 files (recipes + graphics)
  → 1 read
  → 2 parse (JSON → Rust)
  → 3 into our models
  → 4 consume (paint → teaching-aid SVG string)
  → (later) write .svg file / CLI
```

**Data flow ≠ TDD.** TDD is how we implement each step:

1. Write ONE failing test ← red  
2. Write minimal code ← green  
3. Refactor if needed  
4. Next test  

Teaching order in chat: **purpose → diagram → toolkit → workflow → code** (one screen at a time; no disk writes without user OK).

---

## Phases

| Phase | What | Done when |
|------:|---|---|
| **A** | MVP: hardcoded SVG proves concept under tests | **DONE** — colours/paths asserted |
| **B** | Read + parse **recipes** JSONL → recipe model (子) | test: 子 is ND with N/D signs + colours/indexes |
| **C** | Read + parse **graphics** JSONL → path list (子) | test: ≥3 paths; indexes in range |
| **D** | Consume: models → **computed** SVG (replace hardcoded body) | same SVG asserts, fills from data |
| **E** | Glue: both files + `子` → teaching-aid SVG | one end-to-end test |
| **F** | Later: errors, modes, modules, CLI, ship | after E |

**Inputs for B–E:** `testdata/recipes/sample.jsonl`, `testdata/graphics/sample.jsonl`.

---

## NOW / NEXT

```text
NOW:  commit browser SVG slice (path_flip + examples/zi-full)
DONE: A–D · F1–F4 · CLI --mode · MMAH y'=900−y + xmlns/viewBox
NEXT: typed errors · font source tag (later)
```

---

## Agent rules for this spine

- Stick to the phase ladder; don’t jump to CLI or full module trees early.  
- User types implementation bodies; agent coaches with small chunks.  
- No “semi-real mock JSONL” detour unless user asks.  
- When lost: reopen **this file**, not old C1–C9 digressions.
