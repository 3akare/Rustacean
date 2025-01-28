# 0x05-Ownership

## Ownership in Rust
- **Every value has exactly one owner**.
- **Only one owner at a time**: When a value is assigned to another variable or passed to a function, the original owner is invalidated ("moved").
- **Dropped on scope exit**: When the owner goes out of scope, Rust automatically deallocates the value (via `drop`).

### Owner Definition
An owner is the variable or data structure responsible for the value’s memory. It controls allocation (via initialization) and deallocation (via scope exit).

---

## Scope
- **Global Scope**: Variables declared with `static` (lives for the entire program).
- **Local Scope**: Variables within a function or block (lives until the block ends).

---

## Stack Memory
- Stores **fixed-size data** (e.g., integers, pointers, structs with known size).
- Uses **LIFO** (Last-In, First-Out) allocation.
- Allocation/deallocation is fast (pointer arithmetic only).

---

## Heap Memory
- Stores **dynamically sized or large data** (e.g., `String`, `Vec`).
- **Slower than stack**: Requires allocation at runtime and indirection via pointers.
- Heap allocation returns a pointer (stack-stored) pointing to the heap address.

---

## The `String` Type
- **Heap-allocated**, mutable, and dynamically sized.
- The `String` struct on the stack contains three `usize` fields:
  1. `ptr`: Pointer to the heap-allocated bytes.
  2. `len`: Current length of the string (bytes used).
  3. `capacity`: Total allocated memory (bytes reserved).

```rust
let s1 = String::from("Hello"); // Heap-allocated
```

---

## Copy vs Move
### Copy Semantics (Stack-Only Types)
- Applies to **fixed-size types** (e.g., `i32`, `bool`, tuples of `Copy` types).
- The value is **bitwise copied**; both variables are valid.
```rust
let x = 5;
let y = x; // Copy: x and y are independent
```

### Move Semantics (Heap-Allocated Types)
- Applies to **non-`Copy` types** (e.g., `String`, `Vec`).
- Ownership is **transferred**; the original variable is invalidated.
```rust
let s1 = String::from("hello");
let s2 = s1; // Move: s1 is now invalid
// println!("{}", s1); // ERROR: borrow of moved value
```

### Deep Copy with `clone()`
- Explicitly duplicates heap data (expensive for large data).
```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // Both s1 and s2 are valid
```

---

## Key Takeaways
1. **Ownership prevents dangling pointers**: Rust invalidates moved variables to enforce memory safety.
2. **The borrow checker enforces rules**: It ensures references do not outlive their data.
3. **`Copy` vs `Clone`**:
   - `Copy`: Implicit, low-cost bitwise copy (stack-only).
   - `Clone`: Explicit, potentially expensive deep copy (heap-aware).
