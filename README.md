# Patina

Watch Rust run, line by line. An interactive visualizer for core Rust concepts and DSA interview patterns, in a single HTML page.

**Live:** https://YOUR-USERNAME.github.io/patina/

## What's inside

**29 Rust concept lessons** drawn as stack, heap and static-memory diagrams, task lanes and tables: ownership and moves, borrowing, lifetimes (including elision, `'static` and variance), `Box`/`Rc`/`RefCell`, `Weak` cycles, `Arc` vs `Rc`, `Vec<Vec<i32>>` layout, struct layout and `PhantomData`, enums and niche optimisation, strings and UTF-8, `Cow`, zero-copy parsing, `Drop` and `mem::take`, traits and dispatch, advanced traits, closures, custom iterators, error handling with `?`, `unsafe`, `macro_rules!`, `?Sized` and FFI, threads with `Arc<Mutex>`, scoped threads and Rayon, `Send`/`Sync`, atomics and memory ordering, `RwLock`/`Condvar`/deadlock, async with Tokio channels, `select!`/`Semaphore`/`spawn_blocking`, and `Pin`.

Each lesson has a "Show failing version" toggle with the real compiler error (or the real runtime panic, leak or hang).

**25 DSA patterns** with step-through traces and your own inputs: two pointers, sliding window, binary search, monotonic stack and deque, prefix sums, Kadane, intervals, list reversal, fast/slow pointers, tree DFS/BFS, BST, trie, heaps, two heaps, grid BFS, Dijkstra, DFS cycle detection, topological sort, union-find, backtracking, 1D and 2D DP, and an LRU cache.

An "Analyze a question" tab suggests which pattern fits a pasted problem statement.

## Verified Rust

Every solution and program shown on the page is in [`rust/`](rust/). Run `rust/verify.sh` to compile them all with `-D warnings`, run the unit tests, and print each program's output. Tested with rustc 1.75, Tokio 1.38 and Rayon 1.10.

## Run locally

Open `index.html` in a browser. No build step and no dependencies; fonts load from Google Fonts with system fallbacks.
