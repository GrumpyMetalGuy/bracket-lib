# Upstream issue/PR triage

Fork of https://github.com/amethyst/bracket-lib. Worked oldest-first through
open upstream issues and PRs; fixed what this fork can fix.

Legend: [X] fixed (branch named), [-] skipped (feature/env/question/no repro/non-code).

## Fixed (branches off master)

- #174 draw_box not filling       — fix/174  (interior now uses bg color)
- #181 line2d_vector negatives     — already fixed upstream (test present)
- #203 left_click on release       — fix/203
- #260 cellular distance dead code — fix/260
- #306 colored text drops plain    — fix/306
- #310 font not found message      — fix/310
- #316 Default+Display for Point   — fix/316
- #340 cellular distance underflow — fix/340
- #341 truncated doc comment       — fix/341
- #343 seeded() zero noise         — fix/343
- #359 unused incremental key      — fix/359
- #363 A* g-cost tracking          — fix/363
- #365/#366 crossterm flush        — fix/365

## Skipped (not code bugs this fork should change)

- #18, #34, #62, #74, #101, #134, #144, #150, #153, #159, #160, #167,
  #169, #176, #197, #200, #249, #254, #256, #266, #274, #293, #319,
  #322(?), #328, #353, #354 — feature requests / questions
- #44, #45, #175, #177, #180(?), #184, #207, #209, #222, #227, #232,
  #236, #253, #261, #264, #294, #301, #302, #318, #323, #324, #331,
  #332, #339, #346, #347, #352, #355, #358 — platform/driver/env/wasm specific
- #152, #183, #255, #259, #262, #279, #281, #290, #308, #320, #330,
  #338, #361, #362 — usage/perf/meta/unreproducible (no code change)
- #257 — duplicate of #346-style crossterm sizing; needs repro
- #143, #303, #304, #305, #307, #313, #317, #321, #335, #342, #351,
  #364 — PRs; #321/#364 overlapped by #343/#329 fixes here; others are
  feature/dependency/perf changes left to upstream

## Notes

- Issue #329 (curses `fitscreen`) is already fixed on master (the field
  exists in wasm InitHints), so no branch was needed.
- Issue #363 kept with a straightforward optimality test; the g-cost bug
  mainly understates `f` for nodes more than one hop from the start.
- The cross_term feature does not compile on master (winit 0.30 removed
  `VirtualKeyCode`); that is separate from the #365 flush fix and left
  as-is.
