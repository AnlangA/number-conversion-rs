# AI_README

A concise, high-signal guide for AI agents working on this repository. Focuses on structure, conventions, and safe extension points. Prefer small, verifiable changes with tests.

## Project overview
- Language/GUI: Rust + egui (eframe)
- Purpose: A desktop utility with multiple pages (number conversion, text conversion, bit viewer, calculator). The calculator currently contains a "Radix" sub‑tool for evaluating expressions typed in a selected base (2/8/10/16), calculating in decimal via mathcore, then rendering results in multiple bases.
- Key external crate: `mathcore` for expression evaluation (functions like `sin`, `cos`, `pow`, etc.).

## Build and test
- Build/check: `cargo check`
- Run app: `cargo run`
- Run tests: `cargo test -q`
- Test policy: Keep test scope minimal and fast. Prefer targeted tests over entire suite when iterating.

## Repository layout (relevant parts)
- src/app/application.rs
  - App wiring: fonts, navigation bar, central panel routing.
- src/ui/components/navigation.rs
  - Top bar navigation, `AppPage` enum.
- src/ui/pages/mod.rs
  - Pages registry (number_conversion, text_conversion, bit_viewer, calculator).
- src/ui/pages/calculator/mod.rs
  - Calculator page (Radix sub‑project) implementation. Contains UI, expression conversion, formatting, highlighting, and history.

## Calculator: Radix sub‑project
The Radix calculator lets users type expressions using digits of the selected base. It converts numbers to decimal, evaluates via `mathcore`, then renders results in bases 2/8/10/16. Calculation is automatic on input/base change.

### State (struct RadixCalculator)
- `radix: u32` chosen base (2/8/10/16)
- `input: String` user expression (in chosen base)
- `output: String` last formatted output in the current base (kept for history)
- `last_error: Option<String>` most recent error message (not stored in history)
- `last_value: Option<f64>` last successful decimal value (enables multi‑base display)
- `history: VecDeque<HistoryEntry>` ring buffer; see History

### Compute flow
1) User input changes or base changes → `compute()` called.
2) Convert input from `radix` to a decimal expression string.
   - Parses numbers (supports `_` separators) and operators `+ - * / % ^ ,` and parentheses.
   - Identifiers (letters/underscore followed by alphanumerics/underscore) are passed through unchanged.
   - Supports unary minus for numbers in appropriate positions.
   - Supports implicit multiplication (see below).
3) Evaluate decimal expression with `mathcore::MathCore::calculate`.
4) If result is finite:
   - Save `last_value`.
   - Format an output string in the currently selected base using `format_auto`.
   - Push a history entry with: selected radix, original input, decimal expression, and current‑base output.
5) On error/non‑finite: set `last_error` and do not write history, clear `last_value`.

### Implicit multiplication rules
- Insert `*` automatically between adjacent tokens when mathematically implied:
  - Number/`)`/Identifier followed by Number/Identifier → insert `*`.
  - Number/`)` followed by `(` → insert `*`.
  - Identifier followed by `(`:
    - If identifier is a known function (e.g., `sin`, `cos`, `sqrt`, `pow`, `log`, `ln`, `min`, `max`, etc.), treat as a function call (no `*`).
    - Otherwise insert `*` (e.g., `pi(2)` becomes `pi*2`).
- Note: In non‑decimal bases (e.g., base 16), `A..F` are digits, not identifiers. So `A(B+1)` means `10*(11+1)` in base‑16 semantics.

### Formatting of results
- `format_auto(f64, radix, frac_digits)`:
  - If the result is within a small tolerance of an integer (≈1e‑12 relative/absolute), format as an integer in the target base.
  - Otherwise format as a floating value.
- For decimal (radix=10): prints a decimal float (12 digits), trimmed trailing zeros and trailing dot.
- For non‑decimal: prints integer part in that base plus a fractional part approximated with repeated multiply‑and‑floor for up to `frac_digits` digits (default 16). Negative sign applied consistently.
- The UI shows the computed value simultaneously in bases 2, 8, 10, 16. The history stores only the output string in the currently selected base, plus the decimal expression.

### History
- Type: `VecDeque<HistoryEntry>`; capacity bound via `MAX_HISTORY` (200). When over capacity, pop from front.
- Contains: `radix`, `input` (original), `decimal_expr` (converted), `output` (formatted string), `error` (unused now; errors aren’t recorded).
- UI offers a “清空历史” (clear) button and a “重用” button to restore a past input and radix.

### Error highlighting (TextEdit)
- A lightweight layouter marks invalid characters in red according to the chosen base.
- Valid: whitespace, digits legal in the base (including `_`), operators `+ - * / % ^ ,`, parentheses, and ASCII alphabetic identifiers/underscore.
- This is a visual aid; actual parsing and conversion enforce validity again.

## Conventions and guardrails for agents
- Prefer minimal, reversible edits. Keep changes scoped and run `cargo check` and relevant tests after changes.
- Do not install or upgrade dependencies without explicit instruction.
- When adding features to the calculator, be careful about base semantics:
  - Digits vs identifiers in bases 2/8/16 (A–F are digits in base‑16).
  - Implicit multiplication insertion can have side effects; update the rule set and tests together.
- If changing formatting (precision, tolerance), keep constants centralized and document the user‑visible impact.
- History must not record failures. Ensure `last_value` is `Some` only on success.

## Typical extension tasks
- Add more function names as “function‑like”:
  - Extend the `is_function_like` helper with additional names if mathcore supports them.
  - Keep it case‑insensitive.
- Make fractional precision configurable:
  - Add a small UI control to set `FRACTION_DIGITS` at runtime.
- Add angle unit toggle (degree/radian):
  - If implemented, apply a transform to trig function inputs in conversion or pre‑evaluation.
- Improve highlighting to token‑aware styles (operators, numbers, identifiers in different colors).
- Persist history:
  - Serialize to a file on exit and reload on startup (opt‑in setting).

## Safe refactoring plan (if needed)
- Split `src/ui/pages/calculator/mod.rs` into submodules:
  - format.rs: number/float formatting and helpers
  - convert.rs: base‑aware tokenization and decimal conversion (incl. implicit multiplication)
  - highlight.rs: layouter and input validation
  - history.rs: HistoryEntry and bounded deque utils
- Introduce `fn eval_expr(expr:&str, radix:u32)->Result<(f64,String),String>` encapsulating conversion and evaluation; `compute()` only orchestrates state and history.
- After splitting, add focused unit tests for each module.

## Troubleshooting
- `sin(10)` looked like `0`? Ensure no integer‑only formatting; current code supports floats. If regressions occur, re‑check `format_auto` and `format_float_in_radix`.
- `sin(pi/2)` not equal to `1`? Confirm the “near‑integer” rounding is applied before non‑decimal formatting. Check tolerance constants.
- Unexpected identifier behavior in base‑16: remember `A..F` are digits; adjust test expectations accordingly.
- NaN/Inf results: these are treated as errors; they do not enter history.

## Code style and testing guidance
- Keep functions short and single‑purpose; prefer extracting helpers over long, nested blocks.
- Document non‑obvious rules (e.g., implicit multiplication) with examples in comments and tests.
- When adding behavior, add or update tests under the smallest scope that covers the change.

## Glossary
- Decimal expression: The ASCII expression string after converting base‑specific number tokens to decimal integers, with identifiers/operators/parentheses otherwise preserved.
- Function‑like identifier: An identifier followed by `(` that should be considered a function call rather than implicit multiplication; managed by `is_function_like`.

