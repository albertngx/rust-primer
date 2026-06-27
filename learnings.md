# Rust Learnings

## e01 — Euclid MCD

### Rust basics
- Function signatures require explicit parameter types and return types (`-> i64`)
- `mut` — variables are immutable by default, you opt into mutability
- Implicit returns — last expression without `;` is the return value
- Semicolons matter — they turn expressions into statements

### Ownership & borrowing (Rust's big idea)
- `&T` vs `&mut T` — read-only vs writable references
- Move semantics — passing a value to a function consumes it unless you pass a reference
- Why `read_line` needs `&mut buffer` — it writes into it

### Standard library
- `io::stdin()` returns an `io::Stdin` handle (an object you call methods on), not the input itself
- `stdin.read_line(&mut buffer)` appends a line of input (including the newline `\n`) into the buffer
- `String::new()` creates an empty growable string on the heap
- `.trim()` removes leading/trailing whitespace and the newline that `read_line` leaves behind
- `.parse::<i64>()` converts a string slice to a number — returns a `Result`, not the value directly
- `.clear()` empties the buffer so you can reuse it for a second read without appending to old content
- `.unwrap()` — extracts the value from a `Result`, panics with a cryptic message if it failed
- `.expect("message")` — same as `unwrap` but panics with your message, prefer this for clarity

### Idiomatic Rust style
- No `return` at end of functions
- Compound assignment (`%=`)
- Tuple swaps `(a, b) = (b, a)`

### Tooling & project structure
- `cargo run` / `cargo build`
- `target/` is build output, never commit it
- `.gitignore` — and that you need `git rm --cached` to untrack already-committed files
- Comments with `//`, no docstrings like Python
