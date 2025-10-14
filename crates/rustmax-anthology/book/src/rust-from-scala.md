# Rust: A Scala Engineer's Perspective

**Author:** Beachape

**Original:** [https://beachape.com/blog/2017/05/24/rust-from-scala/](https://beachape.com/blog/2017/05/24/rust-from-scala/)

---

# Rust: A Scala Engineer's Perspective

**Author:** Beachape

**Original URL:** <https://beachape.com/blog/2017/05/24/rust-from-scala/>

---

May 24th, 2017

 Rust: A Scala Engineer's Perspective
==========

The 1st year anniversary of my first line of Rust code is coming up, and it’s getting for 5 years since I wrote my first line of Scala code. I thought it would be a good idea to summarise my Scala-tinted perspective of The Rust Experience <sup>TM</sup>, one year on.

<img class="caption" height="450" src="/images/rusty-spiral-staircase.jpg" width="450">[Rusty spiral staircase](https://www.flickr.com/photos/janodecesare/2947948666/in/photolist-5uv1r9-56xXaX-4bDqR-SebcMQ-maN4i-7N23dr-7xSH4D-7rhtiD-pHDnby-62icy-pZNZN8-6cZ79B-5uv1BG-4cTa1X-Bwg7kq-7ahJE2-pb2Mcq-5DQf7p-o2NMu4-3VwpKy-nKqFJu-nJCpHS-aA3uj1-zi3AJf-9iUi3a-maMRE-maMUM-maMSb-5bpZDr-388hw8-maMSL-maN4Q-68jZPS-dWFLCF-aA3urd-4vjtb6-7B76ht-36fhwZ-maMYH-7jPJw9-avc8L2-4SQCD3-4C4njx-h46Ev-maN5y-DuqqVb-CpMJiF-maMY4-maN7f-Raj8Es) by Jano De Cesare

This is *not* an objective language vs language comparison. I’ve written this post as part experience dump, part waymark for other Scala devs who are exploring or thinking of exploring Rust.

A bit about me
----------

I’ve written [a few Rust libraries/tools](https://github.com/lloydmeta?utf8=%E2%9C%93&tab=repositories&q=&type=&language=rust) as well as [Scala ones](https://github.com/lloydmeta?utf8=%E2%9C%93&tab=repositories&q=&type=&language=scala). For all intents and purposes, I’m a Scala engineer: I get paid to do it and it’s by far my strongest language. I’ve used Rust in a few of my side projects (libraries and smaller utilities).

On the Scala side, I’m the author of [enumeratum](https://github.com/lloydmeta/enumeratum), which brings flexible enums and [value-enums](https://github.com/lloydmeta/enumeratum#valueenum) to Scala as a library. I’ve also dabbled in writing macro-based libraries to make things like [Free Monads](https://github.com/lloydmeta/freast) and [Tagless Final](https://github.com/lloydmeta/diesel) nicer to use.

On the Rust side, I’ve written [frunk](https://github.com/lloydmeta/frunk), a Rust functional programming toolbelt that is roughly a port of [Shapeless](https://github.com/milessabin/shapeless) with a bit of [cats](https://github.com/typelevel/cats)/[scalaz](https://github.com/scalaz/scalaz) mixed in, which does some pretty funky things with the type system that I’ve blogged about ([1](/blog/2017/03/04/labelledgeneric-in-rust-what-why-how/), [2](/blog/2017/03/12/gentle-intro-to-type-level-recursion-in-Rust-from-zero-to-frunk-hlist-sculpting/), [3](/blog/2017/04/12/boilerplate-free-struct-transforms-in-rust/), [4](/blog/2017/02/04/rust-generic-not-generics/)). I also wrote a Rust port of [requestb.in](https://requestb.in) called [rusqbin](https://github.com/lloydmeta/rusqbin) based on Hyper, and a small WIP async client for Microsoft Cognitive services called [cogs](https://github.com/lloydmeta/cogs).

### Forewarning ###

* I’m biased towards Scala and I’ve mostly gotten used to [Scala’s warts](http://www.lihaoyi.com/post/WartsoftheScalaProgrammingLanguage.html). That said, I make an effort to try to be as neutral as possible.
* When I talk about Rust, I mean Rust stable. This is because I only use Scala stable.
* Some of the stuff that I write about with regards to Rust might have changed by the time you read this. After all, there is an ongoing [ergonomics initiative](https://blog.rust-lang.org/2017/03/02/lang-ergonomics.html)

Overview
----------

* [Things I’m happy with](#things-im-happy-with)
  * [Batteries included](#batteries-included)
  * [Type System](#type-system)
  * [Macros](#macros)
  * [Compile-time optimisations](#compile-time-optimisations)
  * [Syntax](#syntax)
  * [Interoperability with C](#interoperability-with-c)
  * [The current Zeitgeist](#the-current-zeitgeist)

* [Things I’ve adjusted to](#things-ive-adjusted-to)
  * [Semicolons](#semicolons)
  * [Ownership model: Stack vs heap](#ownership-model-stack-vs-heap)

* [Things I wish were different](#things-i-wish-were-different)
  * [Async IO](#async-io)
  * [Strings](#strings)
  * [Cross compiling](#cross-compiling)
  * [Odd headscratchers](#odd-headscratchers)
  * [Gimme](#gimme)

* [Conclusion](#conclusion)

Things I’m happy with
----------

### Batteries included ###

The dev-environment-setup experience with Rust is amazing. The Rust community has striven to make it super easy to [get started with Rust](https://doc.rust-lang.org/book/getting-started.html) and it shows. Literally [one shell command](https://doc.rust-lang.org/book/getting-started.html#installing-rust) will set everything you need up.

* `rustup` for managing your Rust toolbelts (different versions/channels of Rust)
* `cargo` for managing your build and for publishing to crates.io, which includes, among other things:
  * A [`test` subcommand](https://doc.rust-lang.org/book/testing.html) for running tests
  * A [`bench` subcommand](https://doc.rust-lang.org/book/benchmark-tests.html) for running benchmarks

* `rustfmt` for formatting your code (runs on cargo projects via `cargo fmt`)
* `rustdoc` for generating beautiful [documentation websites](https://api.rocket.rs/rocket/).
  * This tool supports doc tests with zero additional configuration/setup (runs as part of `cargo test`)

Coming from Scala, having all of this set up with no fuss right out of the gate is a breath of fresh air and feels like a big win for productivity. I know there are reasons for Scala’s more *modular* approach, but I think it would be nice if *some* of this rubbed off on ~~Scala~~ other languages.

#### **Editor/IDE** ####

When I first started with Rust, I used IntelliJ and its Rust plugin, but later switched to [Microsoft Studio Code](https://code.visualstudio.com/) with the [Rust plugin](https://github.com/editor-rs/vscode-rust), which interfaces very well with Rust Language Server (installable as a [rustup toolchain component](https://github.com/rust-lang-nursery/rls#step-3-install-the-rls)). It feels very lightweight, and offers all the assistance I need.

### Type System ###

If you lean more towards the functional programming paradigm side of Scala then you’ll probably love the following about Rust’s type system:

* No inheritance for data types (there is a bottom type but it’s used much more sparingly)
* No universal equality
* No nulls
* Traits are basically Haskell typeclasses
* Many more primary types (instead of just `Int`, there are `i8`, `i16`, `i32`, `i64`, `isize`, as well as `u8`, `u16` … )

Essentially Rust has a *lot* of the good things about Scala’s type system. One thing currently missing from Rust is first class support for higher-kinded types (HKT), which, to be honest, I don’t miss too much because:

1. There are ways to emulate it to an extent
2. Rust’s ownership/memory model tends to push you towards thinking more granularly about your values/references, something which is perhaps in conflict with the kind of programming typically involving HKT-based abstractions.

If this still sounds unacceptable, just know that you can get quite far in building reuseable abstractions using Rust’s traits + associated types, and BurnSushi’s [port of quickcheck](https://github.com/BurntSushi/quickcheck) is available for writing and enforcing laws.

There are a few interesting things in the pipeline as well:

1. [Higher kinded polymorphism](https://github.com/rust-lang/rfcs/issues/324)
2. [Pi (value) types](https://github.com/rust-lang/rfcs/issues/1930)

Adding functionality by using Rust’s traits should be familiar territory if you’ve written typeclass-like stuff in Scala. In fact, Rust’s trait system feels a lot more similar to Haskell’s typeclass system than Scala’s, something which has its pros and cons (no scoping of implementations for a given type, for example). I’ve written an intro/guide to Rust’s trait system in [another post](/blog/2017/03/12/gentle-intro-to-type-level-recursion-in-Rust-from-zero-to-frunk-hlist-sculpting/).

#### **Type inference** ####

Both Rust and Scala have local type inference, and overall, they work in pretty much the same way. In both of them, you need to write the types for your function parameters. In Scala, you can leave the return type off and have the compiler infer it for you, in Rust you can’t (if you leave it off, it is assumed to be `()`, unit).

### Macros ###

The [Rust macro system](https://doc.rust-lang.org/book/macros.html), while less powerful than Scala’s, is quite useful for keeping your code DRY and importantly, integrates really well with the rest of the language. It is in fact enabled and available out of the box without any additional dependencies/flags.

Compared with Scala’s macros, Rust’s macros feel like a very natural part of the language, and you’ll run into them quite often when reading/using Rust libraries. In Rust code bases, you’ll often see macros declared and used immediately for the purpose of code generation (e.g. deriving trait implementations for a list of numeric types, or for tuples up to N elements), something that Scala users have generally done “out-of-band” by [hooking into SBT](http://www.scala-sbt.org/0.13/docs/Howto-Generating-Files.html) and using another templating or AST-based tool.

On the other hand, in Scala, the usual refrain is “don’t write macros if you don’t have to”. When I compare the approaches the two languages have taken, I feel that Scala may have been overambitious in terms of giving developers power, thus leading to deprecations of APIs that can’t be maintained due to complexity. Indeed, Scala’s metaprogramming toolkit is going through another reform with the migration to [Scalameta](http://scalameta.org/).

Because of its simplicity (the macros work based on a series of patterns), Rust’s macro API may feel limiting at first, but if you stick with it, you’ll likely find that you can accomplish more than what you initially thought. For example, the fact that you can build/restructure macro arguments recursively (!) and call the macro again (or even call another macro) is [a fairly powerful tool](https://stackoverflow.com/a/40070907/1814775).

Having said that, in addition to the legacy macro system, Rust will soon be getting [procedural macros](https://github.com/rust-lang/rfcs/blob/master/text/1566-proc-macros.md), which are more similar to what Scala devs are used to seeing. You can get a peek of what procedural macros are like by looking at [custom derives](https://doc.rust-lang.org/book/procedural-macros.html), which I’ve used to implement [`derive` for `LabelledGeneric` in Rust](/blog/2017/03/04/labelledgeneric-in-rust-what-why-how/).

### Compile-time optimisations ###

I think it’s not news to anyone that Rust is fast and efficient. The [home page of the official site](https://www.rust-lang.org) says it runs “blazingly fast” and features “zero-cost abstractions”, and the Rust-faithfuls loudly trumpted [Rust’s defeat of GCC-C in in k-nucleotide](https://www.reddit.com/r/rust/comments/5vcrvb/rust_is_now_the_fastest_language_on_knucleotide/) a few months ago. Even if you don’t completely buy into the “faster than C” part, it’s not a big jump to say that Rust performance is in the same ballpark as C, or at least, there is no reason for it not to be (yes, language and implementation are different, compilers make a difference, etc.).

I’m particularly impressed by the Rust compiler’s (though I’m not sure if it’s LLVM?) ability to compile abstractions away so that the operations they enable have zero overhead. As a personal anecdote, when I wrote [LabelledGeneric in frunk](https://github.com/lloydmeta/frunk#labelledgeneric), I expected there to be *some* performance difference between using that abstraction for conversions between structs versus writing the conversions by hand (using `From`). After all, there are non-negligible differences in the Shapeless version of it in Scala land ([benchmark code](https://github.com/lloydmeta/caseclass-conversion-benches#results)):

|```<br/>1<br/>2<br/>3<br/>4<br/>5<br/>6<br/>7<br/><br/>```|```<br/>// JMH benchmark results<br/><br/>[info] Benchmark                               Mode  Cnt     Score     Error  Units<br/>[info] Benchmarks.from24FieldsManual           avgt   30    33.626 ±   1.032  ns/op<br/>[info] Benchmarks.from24FieldsShapeless        avgt   30  4443.018 ± 101.612  ns/op<br/>[info] Benchmarks.from25FieldsManual           avgt   30    33.066 ±   0.650  ns/op<br/>[info] Benchmarks.from25FieldsShapeless        avgt   30  4859.432 ± 104.763  ns/op<br/>```|
|----------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

To my surprise, Rust manages to compile frunk’s LabelledGeneric-based, non-trivial, multi-step, unoptimised (other than using the stack, no effort was spent) transform between structs into a zero-cost abstraction. That is, using LabelledGeneric for conversion adds *zero* overhead over writing the transform by hand ([benchmark code](https://github.com/lloydmeta/frunk/blob/master/benches/labelled.rs)):

|```<br/>1<br/>2<br/>3<br/>4<br/>5<br/>6<br/><br/>```|```<br/>// Cargo benchmark results<br/><br/>test from_24fields_manual           ... bench:         109 ns/iter (+/- 49)<br/>test from_24fields_labelledgeneric  ... bench:         104 ns/iter (+/- 24)<br/>test from_25fields_manual           ... bench:         129 ns/iter (+/- 9)<br/>test from_25fields_labelledgeneric  ... bench:         131 ns/iter (+/- 13)<br/>```|
|----------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

![](/images/mind-blown.gif "'Mind Blown'")

*Note*: The Rust vs Scala `LabelledGeneric` benchmarks are not completely apples-to-apples (the Rust version needs to instantiate new source objects every run because of move semantics), but they illustrate the performance difference between LabelledGeneric-based vs handwritten conversion in the two languages.

### Syntax ###

Overall, the Rust’s syntax is very similar to Scala’s. Sure, there are small adjustments here and there (`let` and `let mut` vs `var` and `val`, you’ll be using angle brackets instead of square ones, etc), but overall the languages feel very similar because they’re both C-like languages that are heavily inspired by ML.

Scala people will probably rejoice at things like the [`enum`](https://doc.rust-lang.org/book/enums.html) being available (coming soon to Scala via Dotty) as well as partial destructuring (e.g. assuming `struct Point { x: i32, y: 32}`, you can do `let Point { x, .. } = p;`).

There are a handful of things that you’ll miss just from muscle memory in the beginning, but are either implemented as libraries or are done slightly differently, such as lazy values ([rust-lazy](https://github.com/reem/rust-lazy) or [lazy-static](https://github.com/rust-lang-nursery/lazy-static.rs)) and methods such as Option’s `foreach` (try `if let Some(x) = myOption { /* use x here */ }` instead). Others are just plain missing, such as by-name parameters (not too big of a deal for me), `for`/`do` comprehensions, and keyword arguments (these last two hurt).

Oh, in Rust, types and traits are named the same way as in Scala, in CamelCase, but identifiers (bindings and methods) use snake\_case, which I still find makes code look longer but isn’t a big problem. You’ll find [references](https://aturon.github.io/style/naming.html) that can help if you are unsure and you’ll likely pick it up from reading library code anyways.

As with Swift, I haven’t been able to find conclusive evidence nor credit given to suggest that there was any influence from Scala on Rust …

### Interoperability with C ###

Rust makes working with C as smooth as possible while sticking to its mantra of keeping things safe. For reference take a look at the section in the Rust book that deals with [FFI](https://doc.rust-lang.org/book/ffi.html).

|```<br/>1<br/>2<br/>3<br/>4<br/>5<br/>6<br/>7<br/><br/>```|```<br/>// Taken from the Rust book<br/>#[link(name = "snappy")]<br/>extern {<br/>    fn snappy_max_compressed_length(source_length: size_t) -> size_t;<br/>}<br/><br/>let x = unsafe { snappy_max_compressed_length(100) };<br/><br/>```|
|----------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

The syntax might look familiar to those who have played around with [Scala.Native](https://scala-native.readthedocs.io/en/latest/).

|```<br/>1<br/>2<br/>3<br/>4<br/>5<br/>6<br/><br/>```|```<br/>// Taken from Scala Native homepage<br/>@extern object stdlib {<br/>  def malloc(size: CSize): Ptr[Byte] = extern<br/>}<br/><br/>val ptr = stdlib.malloc(32)<br/><br/>```|
|----------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

Since calling C-code can be unsafe (wrt memory, thread-safety), Rust requires you to wrap your C-calls in unsafe. If you wish to hide this from your users, you can wrap these calls in another function.

|```<br/>1<br/>2<br/>3<br/>4<br/>5<br/>6<br/><br/>```|```<br/>// Taken from the Rust book<br/>pub fn validate_compressed_buffer(src: &[u8]) -> bool {<br/>    unsafe {<br/>        snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0<br/>    }<br/>}<br/><br/>```|
|----------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

Calling Rust code from C is also [very smooth](https://doc.rust-lang.org/book/ffi.html#callbacks-from-c-code-to-rust-functions), something that Scala Native has yet to implement.

### The current Zeitgeist ###

The current “feel” of Rust, and its community (or communities, since libraries/frameworks can have their own) is very welcoming and helpful. It’s also very difficult to quantify so I’ll just list some observations:

* Rust stable is only 2 years old and yet there is an official [ergonomics initiative](https://blog.rust-lang.org/2017/03/02/lang-ergonomics.html) to reduce friction
* I’ve asked a hand full of questions on StackOverflow and have gotten prompt and helpful answers each time.
* Rust is the #1 “most loved” language in [StackOverflow’s 2017 survey](https://insights.stackoverflow.com/survey/2016#technology-most-loved-dreaded-and-wanted)
* Rust feels very community driven: its got a very lively [RFC repo](https://github.com/rust-lang/rfcs/issues) and since I’ve started tinkering in it I’ve seen at least 3 RFCs make it into the language (type macros, custom derives, and `?` syntax for `Try`s).

Things I’ve adjusted to
----------

### Semicolons ###

In Scala, semicolons are optional and *almost* everything is an expression and therefore return values.

|```<br/>1<br/>2<br/>3<br/>4<br/>5<br/>6<br/>7<br/>8<br/>9<br/><br/>```|```<br/>3 // returns 3<br/><br/>val x = 3 // assignment, returns unit<br/><br/>// certain things don't return anything though, such as import<br/>// statements, and blocks<br/><br/>import com.beachape._ // returns nothing<br/>object Hello {} // returns nothing<br/><br/>```|
|----------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

In Rust, semicolons are non-optional and [are of significance](http://rustbyexample.com/expression.html). Statements that end with semicolons return `()` (unit) and those that do not get turned into expressions and thus return a value.

|```<br/>1<br/>2<br/>3<br/>4<br/>5<br/>6<br/>7<br/>8<br/>9<br/>10<br/>11<br/>12<br/>13<br/>14<br/>15<br/>16<br/><br/>```|```<br/>// taken from the Rust book<br/><br/>let x = 5u32; // this is a statement<br/><br/>let y = {<br/>    let x_squared = x * x;<br/>    let x_cube = x_squared * x;<br/><br/>    // This expression will be assigned to `y`<br/>    x_cube + x_squared + x<br/>};<br/><br/>let z = {<br/>    // The semicolon suppresses this expression and `()` is assigned to `z`<br/>    2 * x;<br/>};<br/><br/>```|
|-----------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

### Ownership model: Stack vs heap ###

Rust’s memory/ownership model is, to me, its main killer feature; it gives you tighter control over the way your program consumes memory while maintaining memory-safety, all without having to ship a garbage collector with the runtime. You get to decide whether to pass things by value or by reference as well as mutability of bindings (including when pattern matching).

There is also the matter of *where* things get allocated. In Scala (and perhaps with most JVM-based languages), there are a set of rules that decide whether or not something gets put on the stack or on the heap (and thus incur the future cost of garbage collection). In general, the only time something gets allocated on the stack are primitives that do not escape methods as fields of objects, and references to objects which themselves get allocated on the heap. There might be fun runtime tricks the runtime environment does, like escape analysis, but overall, you don’t get to choose.

In Rust, you can choose to allocate things on the heap by instantiating them inside (or transferring ownership of them to) data structures such as `Box`es or `Vec`s, etc. Or you can choose to work with plain values. You get to pick your abstraction based on the cost you want to pay for the features and guarantees they offer, such as safe multi-thread access ([this page](http://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/) is a great reference point). Either way, Rust’s ownership system will, at compile time, make sure that you won’t get data races caused by, for instance, modifying naked values in different threads with no access control.

Scala’s doesn’t give its users the same level of control, so naturally there is some adjustment to be made. However, contrary to the experiences of some others, I didn’t find the ownership stuff **too** hard to understand and get used to. Having experience with Scala’s rich type system meant that the lifetime annotation stuff was quite easy to come to grips with. Maybe doing C and C++ in Comsci courses in university helped too.

* **Note**: If you’re a glass-half-full kind of person, I guess you can say that Rust *forces* you to take control rather than *gives* you control. It’s all a matter of perspective …
* **Note 2**: If you find yourself doing lots of `.clone()`s to get the compiler off your back, maybe you’re doing something not quite right.

#### **Mutability** ####

Mutability deserves to be mentioned separately. If you’re coming from years of Scala (or pretty much any other language that stresses immutability and referential transparency as the road to enlightenment), writing your first `let mut` or `&mut self` can feel dirty.

It took me a while to get used to the idea, but hey, when in Rome, right? If it helps, remember that Rust is focused on speed and efficiency through (near, or actually) zero-cost abstractions and that, thanks to its strict ownership model, data races due to mutability are not a problem.

Things I wish were different
----------

### Async IO ###

In Scala, most frameworks that deal with any sort of IO have embraced non-blocking IO by utilising some kind of wrapper data type, such as `Future[A]`, `Task[A]`, or `IO[A]` (usually a Monad), that separates the description of your program from its execution, and identify, by type, the effect of talking with the scary and dirty outside world. This allows you to not block the executing thread when waiting for stuff to happen (such as data to come back) by choosing a suitable execution strategy.

In Rust land, most of the widely-used libraries that I’ve seen, such as the Redis client, and and Hyper (and all the various things built on it, such as Rusoto, Rocket, etc) are all blocking. While this works okay for stuff like single-user utilities, this is suboptimal for applications that are IO heavy and need to serve a large number of concurrent users because your application’s threads can get tied up just waiting for data, leaving it unable to serve other requests. Or, you end up with potentially huge thread pools (à la old school Java Servlet apps..), which seems to go against Rust’s spirit of efficiency.

Having said that I know that advances are being made in this area:

* [Tokio](https://tokio.rs/), “tokenised IO”, an async IO framework that exposes a Future-based API is making lots of progress. Looks production-ready.
* [Hyper](https://github.com/hyperium/hyper), the defacto HTTP client server framework, is going to hit 0.11 soon, which will bring with it a Futures-based API based on Tokio. This will likely (I hope) cascade down to any libs based on Hyper.

Also, as of now, it’s painful to transform and return Futures from functions because every transformation causes the concrete type of your object to get chained and tagged with an arbitrary closure type. Since writing the result type is non-optional in Rust, the current solution is to declare your return type as `Box<Future<A>>`, but it’s less efficient at runtime because boxed [trait objects](https://doc.rust-lang.org/book/trait-objects.html) necessitate dynamic dispatch and heap allocation. Hopefully soon “impl Trait” will be released to address this issue ([tracking RFC](https://github.com/rust-lang/rust/issues/34511))

### Strings ###

In Rust there are a number of ways to represent Strings. Here are a few:

* `String` runtime string value, with its contents allocated on the heap
* `&'a str` string with a lifetime
  * `&' static str` string with static lifetime (baked into your binary)

* `Vec<u8>`

While I’ve mostly gotten used to this by now and understand the purpose of having each one, I hope the ergonomics initiative can make this situation better to understand, since strings are so ubiquitous. How? I have no idea..maybe I’m just ranting.

### Cross compiling ###

Obviously, Scala devs are used to compiling once and running the same binaries everywhere thanks to the JVM (mostly :p). While I don’t expect the same for Rust because it compiles to native machine code, I do wish the cross-compilation tools were better out of the box (for example, like [it is in Golang](https://dave.cheney.net/2015/08/22/cross-compilation-with-go-1-5)).

At the moment, depending on the target platform, cross-compilation for Rust is a bit involved and there are several options:

1. Adding a target toolchain via Rustup and possibly installing some more packages specifically for your target platform (as in [this guide](https://hackernoon.com/compiling-rust-for-the-raspberry-pi-49fdcd7df658))
2. Using a pre-built Docker container that holds all the settings/environment variables/installations needed to compile to your target platform (see [rust-on-raspberry-docker](https://github.com/Ragnaroek/rust-on-raspberry-docker))
3. Using the [`cross`](https://github.com/japaric/cross), cargo tool that seems like it automates 2.

My use case is building for my Raspberry Pi and I’ve only tried the first 2, but that last one looks to be the winner here and it would be awesome to see something like that included by default as part of rustup or cargo.

### Odd headscratchers ###

Just a few things I still don’t quite get:

#### **Do we actually need `ref`?** ####

In my opinion, `ref` is unnecessarily confusing. From what I can tell, it’s mostly used for binding pointers during pattern matching

|```<br/>1<br/>2<br/>3<br/>4<br/>5<br/><br/>```|```<br/>match some_int {<br/>  // Why not Some(& s) => ... ???<br/>  Some(ref s) => println!("{}",s),<br/>  None => unreachable!()<br/>}<br/><br/>```|
|----------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|

#### **`&mut`** ####

When handing out references of something bound with let mut, why do i need to do `&mut` instead of just `&` ?

|```<br/>1<br/>2<br/>3<br/>4<br/>5<br/><br/>```|```<br/>// This uses mut for no reason other than to prove a point.<br/>fn non_empty(s: &mut String) -> bool { s.len() > 0 }<br/><br/>let mut string = "hello".to_string();<br/>hello(&mut string); // why can't this just be hello(& string) ??<br/><br/>```|
|----------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

#### **Scoping of lifetimes with braces** ####

I somehow managed to code my way into a deadlock when using `RWLock` because the lifetime-scoping behaviour of `{}` braces when used with pattern matching is, in my opinion, non-intuitive. If you’re interested, more about it in [this issue](https://github.com/rust-lang/rust/issues/37612).

### Gimme ###

I know these things are in the pipeline but I wish they were in Rust yesterday:

1. Higher kinded types
2. “Specialisation”, aka finding the most specific implementation of a traits according to the type of value at the call site. Right now, if you implement a Rust trait `for A`, then it clashes with every other implementation you write. Specialisation should remedy that ([tracking RFC](https://github.com/rust-lang/rust/issues/31844))
3. A REPL. There’s [Rusti](https://github.com/murarth/rusti) but I think Rust is missing a trick by not supplying one out-of-the-box, especially when it’s got such a strong dev-env-setup game.
4. Some kind of `do` or `for` comprehension for working with container types (there are libs out there but built-in would be nice)

Conclusion
----------

That concludes my take on what it’s like to use Rust, from a Scala dev’s perspective, one year on, in 2017. Overall I’m very happy that the me a year ago decided to look into Rust. It’s been a fun and exciting ride: for a while it felt like every few months I was getting new toys that I could immediately use: type macros and custom derives were game changers because they made it ergonomic to write [Hlist types](https://beachape.com/frunk/frunk/macro.Hlist.html) by hand, and made [Generic](https://beachape.com/frunk/frunk_core/generic/trait.Generic.html)/[LabelledGeneric](https://beachape.com/frunk/frunk_core/labelled/trait.LabelledGeneric.html) practical, respectively.

Overall, I believe there are a lot of things in Rust for Scala engineers to like. The community is friendly and diverse so you can easily find a library that interests you to get involved in (shameless plug: contributions to [frunk](https://github.com/lloydmeta/frunk) are always welcome). Or, you can do your own side project and write a small system utility or program a microcontroller; online resources are very easy to find. In any case, with Rust, you really can’t say it’s hard to get started !

Posted by Lloyd May 24th, 2017 [Rust](/blog/categories/rust/), [Scala](/blog/categories/scala/), [language comparison](/blog/categories/language-comparison/), [opinion](/blog/categories/opinion/)

[Tweet](//twitter.com/share)

* [« Boilerplate-free Struct transforms in Rust.](/blog/2017/04/12/boilerplate-free-struct-transforms-in-rust/)
* [Structural typing in Rust »](/blog/2021/05/25/structural-typing-in-rust/)