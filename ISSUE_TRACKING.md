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
- #303 low_cpu forwarding           — pr/303 (merged)
- #335 hashbrown astar              — pr/335 (merged)
- #313 wgpu bump + nearest filter   — fix/wgpu-bump (merged, wgpu 0.19)
- (winit 0.30 cross-backend breakage) — fix/winit-030 (merged)
- (bevy 0.9 -> 0.13 migration)        — deps/bevy-011 (merged, mirrors #342)
- (bevy 0.13 -> 0.16 migration)      — deps/bevy-013, bevy-014, bevy-016 (merged)
- (bevy 0.16 -> 0.20 migration)      — deps/bevy-017, bevy-018, bevy-019, bevy-020 (merged)
- (rand 0.8 -> 0.10, rand_xorshift 0.3 -> 0.5) — deps/rand-010 (merged)
- (crossterm 0.25 -> 0.29)           — deps/crossterm-029 (merged)
- (image 0.24 -> 0.25)               — deps/image-025 (merged)
- (object-pool 0.5 -> 0.6, hashbrown 0.13 -> 0.14) — deps/minor-bumps (merged)
- (specs 0.18 -> 0.20)              — deps/specs-020 (merged)

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
- #143, #304, #305, #307, #313, #317, #321, #342, #351, #364 — PRs;
  #321/#364 overlapped by #343/#329 fixes here; #303 (low_cpu), #335
  (hashbrown) and #313 (wgpu bump, now 0.19) merged; the rest are
  feature/dependency/perf/styling changes left to upstream. #305 adds a
  required `Console::get_offset` trait method (breaking); #304 changes
  `DiceType::default()` and drops regex (behavior change); #342 (bevy
  0.9->0.11) was superseded by the full bevy 0.9->0.20 migration here.

## Notes

- Issue #329 (curses `fitscreen`) is already fixed on master (the field
  exists in wasm InitHints), so no branch was needed.
- Issue #363 kept with a straightforward optimality test; the g-cost bug
  mainly understates `f` for nodes more than one hop from the start.
- The winit 0.30 upgrade broke cross_term, curses, wasm and webgpu on
  master (removed `VirtualKeyCode` and several event types).
  fix/winit-030 (merged) restored cross_term, curses and wasm, and
  fix/wgpu-bump (merged) restored webgpu by bumping wgpu to 0.19. All
  four backends plus native now compile.
- bevy migrated 0.9 -> 0.20 across deps/bevy-011 .. deps/bevy-020
  (skipping the broken 0.15.x umbrella releases). This subsumes PR #342
  and issue #308 (bevy 0.9 tracking). Verified to compile and pass lib
  tests; no render-integration test exists to verify visuals.
