# Rust Concept Index

Short definitions of concepts related to Rust,
each with links to where they are explained in depth:
the [library] books, the [standard library][std] API docs,
and the [rustmax crates][rmx].

[library]: ../library.html
[std]: ../api/std/index.html
[rmx]: ../api/rustmax/index.html

**Ownership and borrowing:**
[Ownership](#ownership) ·
[Moves](#moves) ·
[Borrowing](#borrowing) ·
[Lifetimes](#lifetimes) ·
[Lifetime elision](#lifetime-elision) ·
[The static lifetime](#the-static-lifetime) ·
[Copy and Clone](#copy-and-clone) ·
[Drop and resource management](#drop-and-ownership-based-resource-management) ·
[Slices](#slices)

**Types:**
[Structs and enums](#structs-and-enums) ·
[Option](#option) ·
[Strings](#strings) ·
[Smart pointers](#smart-pointers) ·
[Interior mutability](#interior-mutability) ·
[Clone-on-write](#clone-on-write) ·
[Dynamically sized types](#dynamically-sized-types) ·
[Zero-sized types](#zero-sized-types) ·
[PhantomData](#phantomdata) ·
[The never type](#the-never-type) ·
[Type layout](#type-layout)

**Traits and generics:**
[Traits](#traits) ·
[Generics](#generics) ·
[Trait bounds](#trait-bounds) ·
[Trait objects](#trait-objects) ·
[impl Trait](#impl-trait) ·
[Associated types](#associated-types) ·
[Generic associated types](#generic-associated-types) ·
[Const generics](#const-generics) ·
[Higher-ranked trait bounds](#higher-ranked-trait-bounds) ·
[Marker traits](#marker-traits) ·
[Blanket implementations](#blanket-implementations) ·
[Coherence and the orphan rule](#coherence-and-the-orphan-rule) ·
[Conversion traits](#conversion-traits) ·
[Operator overloading](#operator-overloading) ·
[Deref coercion](#deref-coercion)

**Expressions and control flow:**
[Pattern matching](#pattern-matching) ·
[let else](#let-else) ·
[Closures](#closures) ·
[Iterators](#iterators) ·
[Type inference](#type-inference) ·
[Turbofish](#turbofish)

**Error handling:**
[Result](#result) ·
[The question mark operator](#the-question-mark-operator) ·
[The Error trait](#the-error-trait) ·
[Panics](#panics)

**Unsafe and FFI:**
[Unsafe Rust](#unsafe-rust) ·
[Undefined behavior](#undefined-behavior) ·
[Raw pointers](#raw-pointers) ·
[FFI](#ffi) ·
[no_std](#no_std)

**Concurrency and async:**
[Threads](#threads) ·
[Send and Sync](#send-and-sync) ·
[Shared state](#shared-state) ·
[Atomics](#atomics) ·
[Channels](#channels) ·
[Data parallelism](#data-parallelism) ·
[Async and await](#async-and-await) ·
[Async runtimes](#async-runtimes) ·
[Pinning](#pinning)

**Code organization:**
[Crates and packages](#crates-and-packages) ·
[Modules and visibility](#modules-and-visibility) ·
[Preludes](#preludes) ·
[Workspaces](#workspaces) ·
[Features](#features) ·
[Conditional compilation](#conditional-compilation) ·
[Attributes](#attributes) ·
[Declarative macros](#declarative-macros) ·
[Procedural macros](#procedural-macros) ·
[Build scripts](#build-scripts) ·
[Editions](#editions) ·
[Semantic versioning](#semantic-versioning)

**Idioms and patterns:**
[Newtype](#newtype) ·
[Builder](#builder) ·
[Typestate](#typestate) ·
[Extension traits](#extension-trait) ·
[Sealed traits](#sealed-traits) ·
[The entry API](#the-entry-api) ·
[Lazy initialization](#lazy-initialization)

**Internals:**
[Monomorphization](#monomorphization) ·
[Zero-cost abstractions](#zero-cost-abstractions) ·
[The borrow checker](#the-borrow-checker) ·
[Variance](#variance) ·
[Specialization](#specialization)




## Ownership and borrowing


### Ownership

Every value has exactly one owner, a variable or another value,
and is destroyed when its owner goes out of scope.
Programmers do not need to free memory manually,
and Rust does not need a [garbage collector](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)).

Books: [TRPL 4.1](../library/trpl/ch04-01-what-is-ownership.html),
[Nomicon: Ownership](../library/nomicon/ownership.html)


### Moves

Assigning a value or passing it to a function _moves_ it,
transferring ownership.
The old binding can no longer be used.
A move is a shallow `memcpy` at most,
and frequently optimized away.

Books: [TRPL 4.1](../library/trpl/ch04-01-what-is-ownership.html),
[Reference: Moved and copied types](../library/reference/expressions.html#moved-and-copied-types)


### Borrowing

A reference, `&T` or `&mut T`, uses a value without taking ownership of it.
At any moment a value may have any number of shared `&` borrows
or exactly one `&mut` borrow, never both.
One can think of `&mut` as "exclusive" rather than "mutable".
The mutability is a benefit of having exclusive access.

Books: [TRPL 4.2](../library/trpl/ch04-02-references-and-borrowing.html),
[Nomicon: References](../library/nomicon/references.html)


### Lifetimes

A lifetime is the region of code over which a reference is valid,
written `'a`.
Lifetime annotations never change how long anything lives.
They describe relationships between references
so the compiler can check them.

Books: [TRPL 10.3](../library/trpl/ch10-03-lifetime-syntax.html),
[Nomicon: Lifetimes](../library/nomicon/lifetimes.html)


### Lifetime elision

Rules that let most function signatures omit lifetimes.
With exactly one input reference, the output borrows from it.
In a method, the output borrows from `self`.
Elision rules reflect the most common lifetime patterns.
When elision produces incorrect restrictions it results in compilation failures.
Write the lifetimes out then.

Books: [Reference: Lifetime elision](../library/reference/lifetime-elision.html),
[Nomicon: Lifetime elision](../library/nomicon/lifetime-elision.html)


### The static lifetime

`'static` means "can live for the rest of the program",
not "lives for the rest of the program".
As a bound, `T: 'static` means the type holds no non-static borrows,
so owned types like `String` satisfy it.

Books: [TRPL 10.3](../library/trpl/ch10-03-lifetime-syntax.html)


### Copy and Clone

`Clone` is an explicit, possibly expensive, duplication: `x.clone()`.
`Copy` marks types where a bitwise copy is enough,
so they are copied instead of moved.
A type can be `Copy` only if all its fields are `Copy`
and it has no `Drop` implementation.

Books: [TRPL Appendix C](../library/trpl/appendix-03-derivable-traits.html)<br>
std: [`Copy`](../api/core/marker/trait.Copy.html),
[`Clone`](../api/core/clone/trait.Clone.html)


### Drop and ownership-based resource management

When a value goes out of scope its `Drop` implementation runs,
then its fields are dropped in declaration order.
Resources such as files, locks, and sockets are released this way.
This pervasive behavior is what C++ calls (confusingly)
["resource acquisition is initialization"](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization).

Books: [TRPL 15.3](../library/trpl/ch15-03-drop.html),
[Reference: Destructors](../library/reference/destructors.html),
[Nomicon: OBRM](../library/nomicon/obrm.html)<br>
std: [`Drop`](../api/core/ops/trait.Drop.html),
[`std::mem`](../api/core/mem/index.html)


### Slices

A slice, `[T]`, is a view into a contiguous run of elements.
It is [dynamically sized](#dynamically-sized-types),
and almost always used behind a reference as `&[T]` or `&mut [T]`,
which are also conventionally called slices.
Likewise `&str` is a string slice,
a reference to the unsized `str` slice.
Take slices as function parameters rather than `&Vec<T>` or `&String`,
each of which automatically dereferences to a slice.

Books: [TRPL 4.3](../library/trpl/ch04-03-slices.html),
[Reference: Slice types](../library/reference/types/slice.html)<br>
std: [`std::slice`](../api/core/slice/index.html)




## Types


### Structs and enums

Structs group fields together. Enums choose between variants,
each of which can carry its own data.
Enums are [sum types](https://en.wikipedia.org/wiki/Tagged_union),
and together with structs they make up Rust's [algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type).
They are the idiomatic way to model a value that is one of several shapes,
and `match` forces every shape to be handled.

Books: [TRPL 5](../library/trpl/ch05-00-structs.html),
[TRPL 6](../library/trpl/ch06-00-enums.html),
[Reference: Enumerations](../library/reference/items/enumerations.html)


### Option

The `Option<T>` enum is either `Some(T)` or `None`.
Rust uses it where other languages use [null](https://en.wikipedia.org/wiki/Null_pointer).
Its combinators, like `map`, `and_then`, `unwrap_or`, and `ok_or`,
are core Rust vocabulary.

Books: [TRPL 6.1](../library/trpl/ch06-01-defining-an-enum.html)<br>
std: [`std::option`](../api/core/option/index.html)


### Strings

`String` is an owned, growable, [UTF-8](https://en.wikipedia.org/wiki/UTF-8) buffer.
`&str` is a borrowed view of one.
Strings can't be indexed by integer because a Unicode character may be several bytes.
`OsString` and `PathBuf` hold platform strings that may not be UTF-8,
and `CString` holds the [NUL-terminated strings](https://en.wikipedia.org/wiki/Null-terminated_string) C expects.

Books: [TRPL 8.2](../library/trpl/ch08-02-strings.html)<br>
std: [`std::string`](../api/alloc/string/index.html),
[`std::str`](../api/core/str/index.html),
[`std::ffi`](../api/std/ffi/index.html)


### Smart pointers

Types that own a value through a pointer and add behavior.
`Box<T>` puts a value on the heap with a single owner.
`Rc<T>` and `Arc<T>` add shared ownership through [reference counting](https://en.wikipedia.org/wiki/Reference_counting),
`Arc` being the thread-safe one.
Cycles of `Rc` and `Arc` leak.
Break them with `Weak`.

Books: [TRPL 15](../library/trpl/ch15-00-smart-pointers.html)<br>
std: [`Box`](../api/alloc/boxed/struct.Box.html),
[`std::rc`](../api/alloc/rc/index.html),
[`Arc`](../api/alloc/sync/struct.Arc.html)


### Interior mutability

Mutating through a shared `&` reference,
with the exclusivity rule checked at runtime or made unnecessary statically.
`Cell` moves values in and out without ever handing out a reference,
so it needs no runtime checks.
`RefCell` checks borrows at runtime.
`Mutex`, `RwLock`, and [atomics](#atomics) do the same across threads.
`Rc<RefCell<T>>` is a common combination that produces
shared mutable data structures like in many other languages.
It is a pattern that works against the strengths of Rust's type system
and its presence can be a hint that the code could be restructured
to be more idiomatic and possibly more performant.

Books: [TRPL 15.5](../library/trpl/ch15-05-interior-mutability.html),
[Reference: Interior mutability](../library/reference/interior-mutability.html)<br>
std: [`std::cell`](../api/core/cell/index.html)


### Clone-on-write

`Cow<'a, T>` holds either a borrowed or an owned value,
cloning only when it has to be modified or kept.
It suits functions that usually return their input unchanged
but sometimes have to build something new.

std: [`Cow`](../api/alloc/borrow/enum.Cow.html)


### Dynamically sized types

Types whose size is only known at runtime: `str`, `[T]`, and `dyn Trait`.
They only exist behind a "fat" pointer that carries a length or [vtable](https://en.wikipedia.org/wiki/Virtual_method_table).
Generic parameters are implicitly `Sized`.
Write `T: ?Sized` ("maybe sized") to accept these types too.

Books: [Reference: Dynamically sized types](../library/reference/dynamically-sized-types.html),
[Nomicon: Exotic sizes](../library/nomicon/exotic-sizes.html)<br>
std: [`Sized`](../api/core/marker/trait.Sized.html)


### Zero-sized types

Types with no data, such as `()`, unit structs, and `PhantomData`,
take up no memory at all.
A `HashMap<K, ()>` is effectively a `HashSet<K>`, with no wasted space.

Books: [Nomicon: Exotic sizes](../library/nomicon/exotic-sizes.html)


### PhantomData

A zero-sized marker that makes a type act as if it owned a `T`
without storing one.
Use it when a type parameter appears only in the meaning of a type,
such as typed IDs or handles to foreign data,
so lifetime [variance](#variance) is analyzed correctly.
In safe code the compiler will tell you when you need this.

Books: [Nomicon: PhantomData](../library/nomicon/phantom-data.html)<br>
std: [`PhantomData`](../api/core/marker/struct.PhantomData.html)


### The never type

`!` is the type of expressions that never produce a value,
like `panic!()`, `loop {}`, `return`, `continue`.
It is also the return type of functions that never return,
e.g. by panicking or looping forever.
It coerces to any type, which is why `let x: u32 = todo!();` compiles.

Books: [Reference: Never type](../library/reference/types/never.html),
[TRPL 20.3](../library/trpl/ch20-03-advanced-types.html)


### Type layout

By default the compiler is free to reorder and pad struct fields.
`#[repr(C)]` fixes the layout to match C,
and `#[repr(transparent)]` guarantees a wrapper has its single field's layout.

Books: [Reference: Type layout](../library/reference/type-layout.html),
[Nomicon: repr(Rust)](../library/nomicon/repr-rust.html),
[Nomicon: Alternative representations](../library/nomicon/other-reprs.html)<br>
std: [`std::mem`](../api/core/mem/index.html)




## Traits and generics


### Traits

A trait is a set of methods a type can implement,
and is Rust's main tool for abstraction and code sharing.
A trait's methods can only be called where the trait is in scope.
Crates that are heavily generic often provide a [prelude](#preludes)
to import many traits at a time.

Books: [TRPL 10.2](../library/trpl/ch10-02-traits.html),
[Reference: Traits](../library/reference/items/traits.html)


### Generics

Functions and types parameterized over other types, `fn max<T: Ord>(...)`.
Generic code is compiled separately for each concrete type it is used with,
called [monomorphization](#monomorphization).

Books: [TRPL 10.1](../library/trpl/ch10-01-syntax.html),
[Reference: Generic parameters](../library/reference/items/generics.html)


### Trait bounds

Constraints on generic parameters, `T: Display + Clone`,
written inline or in a `where` clause.
A bound grants a capability.
Inside the function you can only use what the bounds promise.

Books: [TRPL 10.2](../library/trpl/ch10-02-traits.html),
[Reference: Trait and lifetime bounds](../library/reference/trait-bounds.html)


### Trait objects

`dyn Trait` is a value of some unknown type implementing `Trait`,
called through a [vtable](#dynamically-sized-types), usually as `Box<dyn Trait>` or `&dyn Trait`.
Use trait objects to hold collections of heterogeneous types,
or to reduce compile times and the syntactic overhead
of generics when ultimate performance is not necessary.
Not every trait can be made into an object,
and traits that can be are called "dyn compatible",
formerly "object safe".

Books: [TRPL 18.2](../library/trpl/ch18-02-trait-objects.html),
[Reference: Trait objects](../library/reference/types/trait-object.html)


### impl Trait

In argument position, `impl Trait` is shorthand for a generic parameter.
In return position, it returns one concrete type without naming it,
which is how functions return closures and iterator chains.

Books: [TRPL 10.2](../library/trpl/ch10-02-traits.html),
[Reference: Impl trait type](../library/reference/types/impl-trait.html)


### Associated types

A type chosen by each implementation of a trait,
like `Iterator::Item`.
A trait with a generic parameter, `Iterator<T>`,
could be implemented many times for one type, once per `T`.
With an associated type there is only one implementation,
so `Item` is determined by the implementing type
and callers never have to specify it.

Books: [TRPL 20.2](../library/trpl/ch20-02-advanced-traits.html),
[Reference: Associated items](../library/reference/items/associated-items.html)


### Generic associated types

Associated types with their own generic or lifetime parameters,
`type Item<'a> where Self: 'a;`.
They make "lending" iterators and similar borrowed-view APIs expressible.

Books: [Reference: Associated items](../library/reference/items/associated-items.html),
[RFC 1598](../library/rfcs/1598-generic_associated_types.html)


### Const generics

Generic parameters that are values rather than types,
most often array lengths: `struct Matrix<const N: usize>`.
This is what lets arrays of any length implement traits.

Books: [Reference: Generic parameters](../library/reference/items/generics.html),
[RFC 2000](../library/rfcs/2000-const-generics.html)


### Higher-ranked trait bounds

`for<'a> F: Fn(&'a T)` says a bound holds for every lifetime,
not just one chosen by the caller.
They are mostly written for you by elision in `Fn` bounds,
and show up by name in error messages about closures.

Books: [Nomicon: HRTB](../library/nomicon/hrtb.html),
[Reference: Trait and lifetime bounds](../library/reference/trait-bounds.html)


### Marker traits

Traits with no methods that describe a property of a type:
`Copy`, `Sized`, `Send`, `Sync`, `Unpin`.
Several are "auto traits", implemented automatically
when all of a type's fields implement them.

Books: [Reference: Special types and traits](../library/reference/special-types-and-traits.html)<br>
std: [`std::marker`](../api/core/marker/index.html)


### Blanket implementations

An `impl<T: Display> ToString for T` implements a trait
for every type meeting a bound at once.
They are powerful, but each one rules out many other implementations,
restricting other crates' ability to implement the trait.

Books: [TRPL 10.2](../library/trpl/ch10-02-traits.html),
[Reference: Implementations](../library/reference/items/implementations.html)


### Coherence and the orphan rule

There must be at most one implementation of a trait for a type.
To guarantee this across crates, you may only implement a trait
if either the trait or the type is defined in your crate.
Work around it with a [newtype](#newtype) or an [extension trait](#extension-trait).

Books: [Reference: Implementations](../library/reference/items/implementations.html),
[TRPL 20.2](../library/trpl/ch20-02-advanced-traits.html),
[RFC 1023](../library/rfcs/1023-rebalancing-coherence.html)


### Conversion traits

`From` and `Into` for infallible conversions,
`TryFrom` and `TryInto` for fallible ones,
`AsRef` for cheap reference-to-reference conversions,
and `Borrow` when the borrowed form must hash and compare identically.
Implement `From` and you get `Into` for free.

Books: [API Guidelines: Interoperability](../library/api-guidelines/interoperability.html)<br>
std: [`std::convert`](../api/core/convert/index.html),
[`Borrow`](../api/core/borrow/trait.Borrow.html)<br>
Crates: [`derive_more`](../api/rustmax/derive_more/index.html)


### Operator overloading

Operators are sugar for trait methods:
`a + b` calls `Add::add`, `a[i]` calls `Index::index`,
`a == b` calls `PartialEq::eq`.
Implement the traits to give your types operators.

Books: [TRPL 20.2](../library/trpl/ch20-02-advanced-traits.html),
[Reference: Operator expressions](../library/reference/expressions/operator-expr.html)<br>
std: [`std::ops`](../api/core/ops/index.html),
[`std::cmp`](../api/core/cmp/index.html)


### Deref coercion

When a `&T` is expected and a `&U` is given,
the compiler inserts `Deref` calls until the types match,
so a `&String` passes as `&str` and a `&Box<T>` as `&T`.
Method calls auto-deref the same way.
Implementing `Deref` on non-pointer types to imitate [inheritance](https://en.wikipedia.org/wiki/Inheritance_(object-oriented_programming)) is discouraged.

Books: [TRPL 15.2](../library/trpl/ch15-02-deref.html),
[Reference: Type coercions](../library/reference/type-coercions.html),
[Nomicon: The dot operator](../library/nomicon/dot-operator.html)<br>
std: [`Deref`](../api/core/ops/trait.Deref.html)




## Expressions and control flow


### Pattern matching

Patterns destructure values and test their shape,
in `match`, `if let`, `while let`, `let`, and function parameters.
`match` must be exhaustive, covering every possible value.

Books: [TRPL 6.2](../library/trpl/ch06-02-match.html),
[TRPL 19](../library/trpl/ch19-00-patterns.html),
[Reference: Patterns](../library/reference/patterns.html)


### let else

`let Some(x) = opt else { return; };` binds on a match
and diverges otherwise.
This exists entirely to avoid the unwieldy indentation
imposed by a `match` over a single case.

Books: [Reference: Statements](../library/reference/statements.html),
[RFC 3137](../library/rfcs/3137-let-else.html)


### Closures

Anonymous functions that capture variables from their environment,
`|x| x + offset`.
Each closure has its own unnameable type implementing
`Fn`, `FnMut`, or `FnOnce` depending on how it uses its captures.
A `move` before the arguments makes the closure take ownership of what it captures,
which `thread::spawn` usually requires.

Books: [TRPL 13.1](../library/trpl/ch13-01-closures.html),
[Reference: Closure expressions](../library/reference/expressions/closure-expr.html)<br>
std: [`Fn`](../api/core/ops/trait.Fn.html)


### Iterators

Lazy sequences driven by `next()`, transformed with adapters
like `map`, `filter`, and `zip`, and consumed by `collect` or `sum`.
Iterator chains usually compile to the same code as a hand-written loop.
`for` loops call `IntoIterator::into_iter`,
so `for x in &v`, `for x in &mut v`, and `for x in v`
are equivalent to iterating `v.iter()`, `v.iter_mut()`, and `v.into_iter()`.

Books: [TRPL 13.2](../library/trpl/ch13-02-iterators.html)<br>
std: [`std::iter`](../api/core/iter/index.html)<br>
Crates: [`itertools`](../api/rustmax/itertools/index.html),
[`rayon`](../api/rustmax/rayon/index.html)


### Type inference

The compiler infers types within a function body from how values are used,
so annotations are rarely needed except on function signatures,
which are never inferred.

Books: [TRPL 3.2](../library/trpl/ch03-02-data-types.html)


### Turbofish

The `::<>` syntax for supplying generic arguments in expression position:
`"5".parse::<u32>()`, `iter.collect::<Vec<_>>()`.
The `_` asks the compiler to infer the rest.
The name "turbofish" is an informal affectionate name that is evocative
of the awkward syntax.

Books: [Reference: Paths](../library/reference/paths.html)




## Error handling


### Result

Recoverable errors are values: `Result<T, E>` is `Ok(T)` or `Err(E)`.
A `Result` must be used, or the compiler warns.
Libraries tend to define specific error types.
Applications tend to use a single catch-all error type for convenience.

Books: [TRPL 9.2](../library/trpl/ch09-02-recoverable-errors-with-result.html)<br>
std: [`std::result`](../api/core/result/index.html)


### The question mark operator

`expr?` returns early if `expr` is `Err` or `None`,
and unwraps the value otherwise.
On the way out it calls `From::from` on the error,
which is how one function can propagate several error types into one.

Books: [TRPL 9.2](../library/trpl/ch09-02-recoverable-errors-with-result.html),
[Reference: Operator expressions](../library/reference/expressions/operator-expr.html)


### The Error trait

The common interface for error types,
with a `Display` message plus an optional underlying `source()`.
[`thiserror`] derives it for library errors.
[`anyhow`] wraps any error with context for applications.

Books: [Rust Cookbook: Error handling](../library/rust-cookbook/errors.html)<br>
std: [`std::error`](../api/core/error/index.html)<br>
Crates: [`anyhow`], [`thiserror`]

[`anyhow`]: ../api/rustmax/anyhow/index.html
[`thiserror`]: ../api/rustmax/thiserror/index.html


### Panics

Unrecoverable errors like `panic!`, a failed `unwrap`, or an out-of-bounds index.
By default a panic unwinds the stack, running destructors,
and terminates the current thread.
Panics are for bugs, not for expected failures.

Books: [TRPL 9.1](../library/trpl/ch09-01-unrecoverable-errors-with-panic.html),
[TRPL 9.3](../library/trpl/ch09-03-to-panic-or-not-to-panic.html),
[Nomicon: Unwinding](../library/nomicon/unwinding.html)<br>
std: [`std::panic`](../api/std/panic/index.html)




## Unsafe and FFI


### Unsafe Rust

An `unsafe` block lets you dereference raw pointers,
call `unsafe` functions, and access mutable statics,
promising the compiler that you uphold rules it can't check.
When writing unsafe Rust the goal is to wrap small `unsafe` cores in safe APIs
whose safety doesn't depend on their callers.

Books: [TRPL 20.1](../library/trpl/ch20-01-unsafe-rust.html),
[Nomicon](../library/nomicon/meet-safe-and-unsafe.html),
[Reference: Unsafety](../library/reference/unsafety.html)


### Undefined behavior

Breaking a rule the compiler relies on, such as creating a dangling
reference or two live `&mut` to the same value.
[UB](https://en.wikipedia.org/wiki/Undefined_behavior) may not produce a crash.
The compiler may assume UB never happens,
so UB can produce arbitrary behavior, including a crash.
Safe Rust can't cause it.
Unsafe Rust's primary responsibility is to not cause it.
`cargo miri test` finds much of it in unsafe code.

Books: [Reference: Behavior considered undefined](../library/reference/behavior-considered-undefined.html),
[Nomicon: What unsafe can do](../library/nomicon/what-unsafe-does.html),
[Unsafe Code Guidelines](../library/unsafe-code-guidelines/glossary.html)


### Raw pointers

`*const T` and `*mut T` are pointers without lifetimes or borrow rules.
Creating one is safe.
Dereferencing one is `unsafe`.

Books: [TRPL 20.1](../library/trpl/ch20-01-unsafe-rust.html)<br>
std: [`std::ptr`](../api/core/ptr/index.html)


### FFI

The [foreign function interface](https://en.wikipedia.org/wiki/Foreign_function_interface),
calling C from Rust through `extern "C"` blocks, and exposing Rust to C.
`bindgen` generates Rust declarations from C headers,
and `cc` compiles C and C++ sources in build scripts.

Books: [Nomicon: FFI](../library/nomicon/ffi.html),
[Reference: External blocks](../library/reference/items/external-blocks.html)<br>
std: [`std::ffi`](../api/std/ffi/index.html)<br>
Crates: [`libc`](../api/rustmax/libc/index.html),
[`bindgen`](../api/rustmax/bindgen/index.html),
[`cc`](../api/rustmax/cc/index.html)


### no_std

`#![no_std]` crates use only `core`, and optionally `alloc`,
not the OS-dependent parts of `std`,
so they run on [bare metal](https://en.wikipedia.org/wiki/Bare_machine) and in other restricted environments.

Books: [Embedded Book: no_std](../library/embedded-book/intro/no-std.html),
[Nomicon: Beneath std](../library/nomicon/beneath-std.html)<br>
std: [`core`](../api/core/index.html),
[`alloc`](../api/alloc/index.html)




## Concurrency and async


### Threads

OS threads are spawned with `thread::spawn`,
which requires `'static` data,
or with `thread::scope`, which may borrow from the enclosing function.
The type system prevents [data races](https://en.wikipedia.org/wiki/Race_condition#Data_race) but not [deadlocks](https://en.wikipedia.org/wiki/Deadlock_(computer_science)),
a guarantee often called "fearless concurrency" in Rust.

Books: [TRPL 16.1](../library/trpl/ch16-01-threads.html)<br>
std: [`std::thread`](../api/std/thread/index.html)


### Send and Sync

`Send` types can be moved to another thread.
`Sync` types can be shared between threads by reference.
They are auto traits, so most types get them for free.
`Rc` is neither, so it is tied to its thread.
`RefCell` is `Send` but not `Sync`.
Their thread-safe counterparts are `Arc` and `Mutex`.

Books: [TRPL 16.4](../library/trpl/ch16-04-extensible-concurrency-sync-and-send.html),
[Nomicon: Send and Sync](../library/nomicon/send-and-sync.html)<br>
std: [`Send`](../api/core/marker/trait.Send.html),
[`Sync`](../api/core/marker/trait.Sync.html)


### Shared state

Data shared between threads lives in an `Arc`
and is mutated through a `Mutex` or `RwLock`.
A mutex in Rust owns the data it protects,
so you can't read or write the data without holding the lock.

Books: [TRPL 16.3](../library/trpl/ch16-03-shared-state.html),
[Nomicon: Poisoning](../library/nomicon/poisoning.html)<br>
std: [`std::sync`](../api/std/sync/index.html)


### Atomics

Integer and pointer types updated with single hardware instructions,
the building blocks of [lock-free](https://en.wikipedia.org/wiki/Non-blocking_algorithm) code.
Each operation takes a [memory `Ordering`](https://en.cppreference.com/w/cpp/atomic/memory_order).
`SeqCst` is the safe default.
`Relaxed` is for counters.
Atomics are tricky to use soundly and hard to debug. Be careful!

Books: [Nomicon: Atomics](../library/nomicon/atomics.html),
[Reference: Memory model](../library/reference/memory-model.html)<br>
std: [`std::sync::atomic`](../api/core/sync/atomic/index.html)


### Channels

[Message passing](https://en.wikipedia.org/wiki/Channel_(programming)) between threads, senders on one side,
a receiver on the other.
`std::sync::mpsc` covers the basics.
`crossbeam` and `tokio` have more capable channels.

Books: [TRPL 16.2](../library/trpl/ch16-02-message-passing.html)<br>
std: [`std::sync::mpsc`](../api/std/sync/mpsc/index.html)<br>
Crates: [`crossbeam`](../api/rustmax/crossbeam/index.html),
[`tokio`](../api/rustmax/tokio/index.html)


### Data parallelism

Splitting one computation over many cores.
`rayon` turns `iter()` into `par_iter()`
and handles the threads and work splitting.

Books: [Rust Cookbook: Concurrency](../library/rust-cookbook/concurrency.html)<br>
Crates: [`rayon`](../api/rustmax/rayon/index.html)


### Async and await

`async fn` returns a [`Future`](https://en.wikipedia.org/wiki/Futures_and_promises), a value that makes progress when polled,
and `.await` suspends until another future is done.
Futures do nothing until awaited or spawned,
and std provides no executor to run them.
`tokio` provides the most common executor.

Books: [TRPL 17](../library/trpl/ch17-00-async-await.html),
[Reference: Await expressions](../library/reference/expressions/await-expr.html)<br>
std: [`std::future`](../api/core/future/index.html),
[`std::task`](../api/core/task/index.html)<br>
Crates: [`futures`](../api/rustmax/futures/index.html)


### Async runtimes

The library that polls futures, schedules tasks,
and provides async I/O and timers.
[`tokio`](../api/rustmax/tokio/index.html) is the de facto standard,
and most async crates assume it.

Books: [TRPL 17.6](../library/trpl/ch17-06-futures-tasks-threads.html)<br>
Crates: [`tokio`](../api/rustmax/tokio/index.html)


### Pinning

`Pin<P>` guarantees a value will not move in memory again,
which async code needs because futures can hold references into themselves.
Most code meets pinning only through `Box::pin` and the `pin!` macro.

Books: [TRPL 17.5](../library/trpl/ch17-05-traits-for-async.html)<br>
std: [`std::pin`](../api/core/pin/index.html),
[`Unpin`](../api/core/marker/trait.Unpin.html)




## Code organization


### Crates and packages

A crate is the unit of compilation, a library or a binary.
A package is defined by a `Cargo.toml` (manifest) that builds one or more crates.
Crates are published to [crates.io](https://crates.io) and documented on
[docs.rs](https://docs.rs).

Books: [TRPL 7.1](../library/trpl/ch07-01-packages-and-crates.html),
[Cargo Book: Manifest](../library/cargo-book/reference/manifest.html),
[Reference: Crates and source files](../library/reference/crates-and-source-files.html)


### Modules and visibility

Modules, declared with `mod`, organize code within a crate
and are the primary boundary of visibility and unsafety in Rust.
Items are private to their module unless marked `pub`,
or `pub(crate)` for crate-wide visibility.
`use` brings paths into scope.

Books: [TRPL 7.2](../library/trpl/ch07-02-defining-modules-to-control-scope-and-privacy.html),
[Reference: Visibility and privacy](../library/reference/visibility-and-privacy.html)


### Preludes

A set of names imported into every module automatically.
The standard prelude, chosen by edition, is why `Option`, `Vec`, `String`,
and traits like `Clone` and `Iterator` need no `use`.
Many crates offer their own prelude to glob-import with `use cratename::prelude::*`.

Books: [Reference: Preludes](../library/reference/names/preludes.html)<br>
std: [`std::prelude`](../api/std/prelude/index.html)<br>
Crates: [`rustmax::prelude`](../api/rustmax/prelude/index.html)


### Workspaces

Multiple packages sharing one `Cargo.lock` and target directory,
with dependency versions they can declare once and share.

Books: [TRPL 14.3](../library/trpl/ch14-03-cargo-workspaces.html),
[Cargo Book: Workspaces](../library/cargo-book/reference/workspaces.html)


### Features

Cargo features turn optional code and dependencies on and off.
Features are unified across the dependency graph, so they must be additive.
Enabling a feature should never remove functionality.

Books: [Cargo Book: Features](../library/cargo-book/reference/features.html)


### Conditional compilation

`#[cfg(...)]` includes code only for some targets, features, or test builds,
and `cfg!` tests the same conditions as a boolean.
The [`cfg_if`](../api/rustmax/cfg_if/index.html) macro chains them
like `if`/`else`.

Books: [Reference: Conditional compilation](../library/reference/conditional-compilation.html)<br>
Crates: [`cfg_if`](../api/rustmax/cfg_if/index.html)


### Attributes

Metadata on items, `#[derive(Debug)]`, `#[test]`, `#[inline]`,
or on a whole crate, `#![no_std]`.
`#[derive]` generates trait implementations,
and is the most common way to use procedural macros.

Books: [Reference: Attributes](../library/reference/attributes.html),
[Reference: Derive](../library/reference/attributes/derive.html),
[TRPL Appendix C](../library/trpl/appendix-03-derivable-traits.html)


### Declarative macros

Macros defined with `macro_rules!` that match token patterns
and expand to code.
They can do things functions can't: take a variable number of arguments,
or declare items.

Books: [TRPL 20.5](../library/trpl/ch20-05-macros.html),
[Reference: Macros by example](../library/reference/macros-by-example.html)


### Procedural macros

Macros written as Rust functions over token streams,
compiled into their own crate:
derive macros, attribute macros, and function-like macros.
Nearly all are built from `syn`, `quote`, and `proc-macro2`.

Books: [TRPL 20.5](../library/trpl/ch20-05-macros.html),
[Reference: Procedural macros](../library/reference/procedural-macros.html)<br>
Crates: [`syn`](../api/rustmax/syn/index.html),
[`quote`](../api/rustmax/quote/index.html),
[`proc_macro2`](../api/rustmax/proc_macro2/index.html)


### Build scripts

A `build.rs` compiled and run before the package itself,
to compile C code, generate Rust source, or probe the system.
Its output goes to `OUT_DIR`, and it talks to Cargo by printing
`cargo::` directives.

Books: [Cargo Book: Build scripts](../library/cargo-book/reference/build-scripts.html)<br>
Crates: [`cc`](../api/rustmax/cc/index.html),
[`bindgen`](../api/rustmax/bindgen/index.html)


### Editions

Opt-in language revisions, 2015, 2018, 2021, and 2024,
selected per crate in `Cargo.toml`.
Crates of different editions link together freely,
and `cargo fix --edition` does most of a migration.

Books: [Edition Guide](../library/edition-guide/editions/index.html)


### Semantic versioning

Cargo treats version numbers as [semver](https://semver.org).
Before 1.0, a minor bump such as 0.7 to 0.8 is a breaking change,
which is why so much of the ecosystem is versioned 0.x.

Books: [Cargo Book: SemVer compatibility](../library/cargo-book/reference/semver.html),
[Cargo Book: Specifying dependencies](../library/cargo-book/reference/specifying-dependencies.html)<br>
Crates: [`semver`](../api/rustmax/semver/index.html)




## Idioms and patterns


### Newtype

A single-field tuple struct around another type, `struct Meters(f64)`.
It gives a value a distinct type at no runtime cost,
hides an implementation, and gets around the [orphan rule](#coherence-and-the-orphan-rule).

Books: [TRPL 20.2](../library/trpl/ch20-02-advanced-traits.html),
[API Guidelines: Newtypes](../library/api-guidelines/type-safety.html#c-newtype)<br>
Crates: [`derive_more`](../api/rustmax/derive_more/index.html)


### Builder

Constructing a complex value step by step through chained method calls,
`Command::new("ls").arg("-l").spawn()`.
This prominent [pattern](https://en.wikipedia.org/wiki/Builder_pattern) mostly exists because Rust has no named or default arguments.

Books: [API Guidelines: Builders](../library/api-guidelines/type-safety.html#c-builder)<br>
std: [`std::process`](../api/std/process/index.html)


### Typestate

Encoding the state of an object in its type,
so operations invalid in the current state don't compile.
A method that consumes `self` and returns a different type
moves the object to its next state.
This is a lightweight form of [typestate analysis](https://en.wikipedia.org/wiki/Typestate_analysis).

Books: [Embedded Book: Typestate programming](../library/embedded-book/static-guarantees/typestate-programming.html)


### Extension trait

A trait defined only to add methods to a type from another crate,
working around the orphan rule.
Conventionally named with an `Ext` suffix, like `FutureExt`.
The [`extension_trait`](../api/rustmax/extension_trait/index.html) macro
removes the boilerplate.

Books: [RFC 445](../library/rfcs/0445-extension-trait-conventions.html)<br>
Crates: [`extension_trait`](../api/rustmax/extension_trait/index.html),
[`futures`](../api/rustmax/futures/index.html),
[`itertools`](../api/rustmax/itertools/index.html)


### Sealed traits

A public trait that downstream crates can't implement,
because it has a private supertrait.
This lets a library add methods to the trait later without breaking anyone.

Books: [API Guidelines: Sealed traits](../library/api-guidelines/future-proofing.html#c-sealed)


### The entry API

`map.entry(key)` looks a key up once and returns a handle to insert,
modify, or read the slot,
avoiding a second lookup and fights with the borrow checker:
`*counts.entry(word).or_insert(0) += 1`.

Books: [TRPL 8.3](../library/trpl/ch08-03-hash-maps.html)<br>
std: [`Entry`](../api/std/collections/hash_map/enum.Entry.html)


### Lazy initialization

Global values computed on first use](https://en.wikipedia.org/wiki/Lazy_initialization).
Used when statics can't be computed at compile time.
Typically created with `LazyLock` and `OnceLock`.

std: [`LazyLock`](../api/std/sync/struct.LazyLock.html),
[`OnceLock`](../api/std/sync/struct.OnceLock.html)




## Internals


### Monomorphization

Generic code is compiled into a separate copy for each set of type arguments,
so it runs as fast as hand-written code
at the cost of compile time and binary size.
Trait objects are the alternative.

Books: [rustc dev guide: Monomorphization](../library/rustc-dev-guide/backend/monomorph.html)


### Zero-cost abstractions

The principle, [inherited from C++](https://en.cppreference.com/w/cpp/language/Zero-overhead_principle),
that abstractions like iterators, closures, and generics
cost nothing at runtime compared to writing the code by hand.

Books: [TRPL 13.4](../library/trpl/ch13-04-performance.html)


### The borrow checker

The compiler pass that enforces ownership and borrowing rules.
Since "non-lexical lifetimes" it reasons about where references are
actually used, not just their lexical scope.
"Fighting the borrow checker" is a common experience for new Rust programmers.
For experienced Rust programmers it is an indication to restructure.

Books: [Nomicon: Limits of lifetimes](../library/nomicon/lifetime-mismatch.html),
[rustc dev guide: Borrow checking](../library/rustc-dev-guide/borrow-check.html),
[RFC 2094](../library/rfcs/2094-nll.html)


### Variance

Whether a generic type with a longer lifetime may be used
where a shorter one is expected.
`&'a T` is [covariant](https://en.wikipedia.org/wiki/Covariance_and_contravariance_(computer_science)) in `T`, so it may.
`&'a mut T` is invariant in `T`, so it may not.
It surfaces mostly as confusing lifetime errors in generic code.

Books: [Nomicon: Subtyping and variance](../library/nomicon/subtyping.html),
[Reference: Subtyping and variance](../library/reference/subtyping.html)


### Specialization

A long-unstable feature allowing overlapping trait implementations
where the more specific one wins.
It is not available on stable Rust,
which is why some generic code can't take fast paths for particular types.
The standard library gets to use it to optimize its own types.

Books: [RFC 1210](../library/rfcs/1210-impl-specialization.html)
