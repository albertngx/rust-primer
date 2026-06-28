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

### References & Vec iteration
- `&numbers[1..]` — borrows a slice, does not consume the Vec; elements are `&T` in the loop
- `*m` — dereference operator: follows the reference to get the value; works for `Copy` types like `u64`
- `u64` is `Copy` — dereferencing copies the value out, original untouched
- `String` is not `Copy` — `*s` to move out of a borrow is a compile error; use `.clone()` or keep it as `&str`
- `&mut` reference — same address as the original, lets you mutate what's there; only one allowed at a time
- Iterating without `&` consumes the Vec, moving each element into the loop body
- A reference is an address, not a copy — there is only one value in memory

### Idiomatic Rust style
- No `return` at end of functions
- Compound assignment (`%=`)
- Tuple swaps `(a, b) = (b, a)`

### Tooling & project structure
- `cargo run` / `cargo build`
- `target/` is build output, never commit it
- `.gitignore` — and that you need `git rm --cached` to untrack already-committed files
- Comments with `//`, no docstrings like Python
