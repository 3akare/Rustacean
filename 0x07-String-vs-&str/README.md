# String vs &str
- String is a `heap-allocated` string type that `owns` its contents and is `mutable`.
- &str is an `immutable` sequence of UTF-8 bytes in memory, it does `not own` the underlying data and is `immutable`
- Think of &str as a `view` on a sequence of characters (stored as UTF-8 bytes) in memory
