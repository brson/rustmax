# Five Lists of Six Things About Rust

**Author:** Graydon Hoare

**Original URL:** <http://graydon2.dreamwidth.org/214016.html>

---

*I don't often give talks about Rust, but I was in Tokyo recently during the Rust 1.0 release and figured I'd drop in on the Mozilla Tokyo office release party. I figured they'd try to make me talk about something, so I prepared a lightning talk. As is typical of my Rust-project communication style, I presented in point form. This is the contents of the talk:*  

Five lists of six things about Rust:
----------

2. Six ways Rust is fundamentally different from how it started  

   2. Borrow checker subsumed most other safety mechanisms, lots discarded  

   3. Much more static: monomorphization of sizes, glue code, and static trait dispatch  

   4. LLVM: strengths (amazing optimization) and weaknesses (narrow semantics)  

   5. Between these two points, maybe 100x faster at runtime, 100x slower to compile  

   6. Grammar, resolution, dispatch, type system much more complex, expressive  

   7. Adopted standard C platform stack, threads, ABI, linkage, mangling, unwinding  

3. Six ways Rust is fundamentally the same as how it started  

   2. Safety through memory ownership and isolation, no global GC, no aliased mutable state  

   3. Default immutable, algebraic data types ("ML in C++ clothing")  

   4. Focus on dense memory layout, interior allocation, minimizing pointer chasing, vectors over lists  

   5. No global namespace, everything module scoped and crate relative  

   6. Support for standard tools: gdb, perf, dtrace, objdump  

   7. RAII, dtors, idempotent uncatchable unwinding ("crash only")  

4. Six things we lost along the way  

   2. Typestate system  

   3. Effect system  

   4. Function complexities: parameter modes, argument binding, stack iterators, tail calls  

   5. Language-integrated runtime for tasks, channels, logging  

   6. GC pointers, task local GC (yes, rustboot had a real mark/sweep)  

   7. Dynamic, structural object types  

5. Six things we gained along the way  

   2. Lambdas, with environment capture  

   3. First class borrowed pointers -- not just parameter modes -- with explicit lifetimes  

   4. Traits, with associated items  

   5. Hygienic macros beyond just raw syntax extensions  

   6. Multiple parallelism modes beyond just actors (ARC, fork/join, SIMD)  

   7. Cargo, crates.io, a huge vibrant community!  

6. Six things I'm irrationally, disproportionately pleased by  

   2. Rich patterns: slice patterns, range patterns, or patterns  

   3. Novel consequences of move semantics: static freeze/thaw, iterators that consume their containers  

   4. Rich syntax extensions: compile time regular expressions, SQL statements, docopt  

   5. Novel embeddings: postgres and python extensions, crypto libraries, kernels in rust  

   6. C++ ballpark on benchmarks  

   7. A 1.0 stable release after nearly a decade of work  

*(Ok, maybe my pleasure at some of these is rational and proportionate..)*