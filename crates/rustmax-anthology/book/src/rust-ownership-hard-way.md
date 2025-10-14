# Rust Ownership the Hard Way

**Author:** Chris Morgan

**Original:** [https://chrismorgan.info/blog/rust-ownership-the-hard-way.html](https://chrismorgan.info/blog/rust-ownership-the-hard-way.html)

---

# Rust Ownership the Hard Way

**Author:** Chris Morgan

**Original URL:** <https://chrismorgan.info/blog/rust-ownership-the-hard-way.html>

---

**[Chris Morgan](https://chrismorgan.info/)** › [Blog](https://chrismorgan.info/blog/)
==========

**Rust ownership, the hard way**
==========

In order to truly understand Rust, one needs to understand its crucial differentiating feature: ownership. Let’s go through it in detail the hard way.

**Tagged:** [Rust](https://chrismorgan.info/blog/tags/rust/)  
**Published:** May 12, 2015  
**Last updated:** August 4, 2021

This article is intended to teach beginners about the Rust programming language. Displayed compiler output is from Rust 1.54.0.

This article is an introduction to Rust’s concept of ownership. It’s designed for someone who is already a programmer but who is not especially familiar (maybe even not at all familiar) with Rust. It doesn’t attempt to explain all the concepts it deals with, but for the most part it should be clear enough. If you’re a beginner you may also find that you don’t understand various parts of it; in such a case, you might like to hold the article while you go and learn more about them elsewhere; you’ll find more value in coming back to this article after.

This concept of ownership is the pivotal part of Rust; it’s the part that makes its combination of efficiency and safety possible. While the rest of Rust is pretty similar to what you’ll find in mainstream languages [OK, so pattern matching and algebraic data types aren’t widespread in C‐style languages, but they’re conceptually simple.], these concepts of ownership and lifetimes are different from anything in any mainstream language, so it’s likely to be the part that you’ll spend the longest trying to grok; if you don’t give up but persist, then when it all clicks, working in Rust will be a joy, and you’ll have unlocked a marvellous ability to reason about code. You’ll also probably wish you could transplant a lot of the aspects of Rust’s ownership model to other languages you deal with. [Seriously. The ownership model makes reasoning about many things *so* much easier, and I really *do* miss it when working in other languages.]

This article deals with the concepts and theory of ownership and lifetimes; it doesn’t provide much in the way of practical usage examples; those you can find elsewhere. But it does explain all the rules that go to make Rust’s ownership model what it is. (It *is* called “the hard way” for a reason.)

Well, on with the first of our four rules:

1. Each object can be used exactly once. When you use an object it is moved to the new location and is no longer usable in the old.

----------

[Later on](#exceptions-to-single-use) we’ll deal with a couple of features that make this model more palatable, but for now we’ll skip them, considering it at the most basic level.

```
struct A {}

fn main() {
    let a = A {};
    let b = a;
    let c = a;
}

```

```
error[E0382]: use of moved value: `a`
 --> <anon>:6:13
  |
4 |     let a = A {};
  |         - move occurs because `a` has type `A`, which does not implement the `Copy` trait
5 |     let b = a;
  |         - value moved here
6 |     let c = a;
  |         ^ value used here after move

error: aborting due to previous error
```

The `A` instance placed in the slot `a` is moved to `b`, rendering `a` unusable.

1. When an object passes out of scope, it is destroyed and is no longer usable.

----------

*Blocks*, delimited by curly braces [But not to be confused with struct declarations like `**struct** A {}` and struct literals like `A {}`, as seen in this next example.], introduce a new level of scope, and anything declared inside a block (e.g. the binding `x` in `**let** x = y;`) will live until the end of that block. [This was strictly true when I first wrote this article, but *non‐lexical lifetimes* landed in Rust 1.31 in 2018: in certain situations objects can now be destroyed before the end of their containing lexical scope; but I don’t think this is the right article to delve into detail about it. Suffice it to say that although it makes Rust conceptutally more complex, it makes it do what you want (rather than producing a compiler error) more of the time.]

```
struct A {}

fn main() {
    {
        let a = A {};
    }
    let b = a;
}

```

This example won’t work, because `a` has fallen out of scope and been destroyed by the time we try to assign it to `b`:

```
error[E0425]: cannot find value `a` in this scope
 --> <anon>:7:13
  |
7 |     let b = a;
  |             ^ not found in this scope

error: aborting due to previous error
```

There are seven ways of introducing variable bindings; all use patterns, and all but `**let**` introduce a new level of scope. They are: `**let**`; match branches; `**for**`; `**if let**`; `**while let**`; function arguments; and closure arguments.

When an object passes out of scope and is destroyed, if the type implements [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html), that destructor will be run. I won’t get into the details of destructors here. Types like `**&**T` and `**&mut** T` (immutable and mutable references) do not have destructors.

1. Blocks can produce a value which goes up one level of scope.

----------

As a language feature, this is typically known as *expression orientation*, as distinct from *statement orientation*. It’s a fairly simple concept and will seem perfectly natural to users of languages like Ruby, though to users of languages like C++ and Python it may seem a little odd. (I came originally from a Python background; at first I thought it a gimmick useful only for skipping the word `**return**` on the last statement of a function, but I rapidly discovered it isn’t a gimmick at all; it’s a very useful feature, for all that it wouldn’t suit a language like Python.)

The main rule is simple: the last expression in a block is the value that the block produces. (A block is thus an expression too.) There are a couple of other rules to deal with the corner cases:

People with some Rust experience may be familiar with the explanation of an implied `()` after a semicolon at the end of a block; this is not accurate, [as this type checking error shows](https://play.rust-lang.org/?code=fn foo() -> i32 { return 0; () }). The reachability check is important as it allows things like a `**return**` statement at the end of a function that returns something other than unit (even though that *specific* thing is considered bad style).

* An empty block produces `()` (*unit*).

* If a block’s contents ends with a semicolon and the end of the block is reachable, the block produces `()`.

Here’s a simple example:

```
let a = {
    // … do anything we like …
    1
};

```

Here, `a` ends up storing the value `1`. Given these rules, you can see that putting braces around an expression changes nothing, for at each added level the block contains only a single expression, which is then its own value. Hence, `**let** a = { { { { 1 } } } };` and `**let** a = 1;` are equivalent. [They’re actually not *quite* equivalent in general, for a block expression is an *rvalue*; thus `string[..].is_empty()` works, while `{ string[..] }.is_empty()` doesn’t.]

In languages without this feature, the only type of block that produces a value (if you’ll allow my sloppy terminology here) is a function; there, they have the `**return**` keyword to fill in this blank. (Rust also has the `**return**` keyword to make early return more convenient, but the language would work fine without it—​it’d just be harder to write some sorts of code.)

```
fn foo_the_c_way() -> i32 {
    return 1;
}

fn foo_the_rust_way() -> i32 {
    1
}
```

Because of this paradigm of expression orientation, the concept of special *ternary expressions* [`condition ? a : b` in most languages; `a **if** condition **else** b` in Python.] is not necessary in Rust, for an `**if**` expression can do the job just fine:

```
let x = if a { b } else { c };

let x = if a {
    // … do things …
    b
} else {
    // … do things …
    c
};

```

1. All objects have a lifetime which constrains which scopes they may be moved out of.

----------

Now we’re getting into the harder stuff, the biggest thing that’s unusual about Rust: lifetimes. Syntactically, a lifetime in Rust is any identifier with the prefix of a single quotation mark, e.g. `*'a*`. Any name will do for a lifetime, but there is one special lifetime, `***'static***`, which means that an object contains no non‐static references. Most of the primitive types (`**i32**`, `**bool**`, `**str**`, `[T]`, *&c.*) are static; those that are not are:

* Arrays (`[T; n]`) and slices (`[T]`) of non‐static types;
* Tuples with non‐static members;
* Non‐static references, *viz.* any `**&**T` or `**&mut** T` except `**&*'static*** T` and `**&*'static* mut** T`.

### Conventions ###

Just as there is a widespread convention for generic type parameters with no special significance to have names following the pattern `T`, `U`, `V`, *&c.*, there is a convention for generic lifetime parameters in Rust to follow the pattern `*'a*`, `*'b*`, `*'c*`, *&c.*

It’s also common to give them more meaningful letters or snake\_case names; for example, where a key‐value pair might be `Pair<K, V>`, if you needed lifetime parameters corresponding to each of them individually (this is occasionally useful, though in a case like this they’ll typically have the same lifetime) you might go for `*'k*` and `*'v*`.

### Lifetime positions in types ###

There are four places where a lifetime can appear in a *type*:

* `**&***'a* Type`: the lifetime of an immutable reference;
* `**&***'a* **mut** Type`: the lifetime of a mutable reference;
* `Type<*'a*>`: a generic lifetime parameter on a type;
* `**dyn** Trait + *'a*` [Before Rust 1.27, this was spelled without the `**dyn**` keyword, called a *[bare trait object](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#bare-trait-objects)*; that syntax was subsequently deprecated and is [removed altogether in the 2021 edition](https://github.com/rust-lang/rust/pull/83213).]: a trait object’s lifetime, as with generic bounds (dealt with below).

The first two are, I believe, fairly obvious; it’s clearly unsafe to have a reference to an object that has been freed. (By baking this into the language, we avoid the problem of *dangling pointers* that languages like C have.)

The third is much the same, as all generic lifetime parameters will, somewhere down the way, be of one of the other types, and so it’s just a way of passing that constraint through the types, like this:

```
struct Ref<'a, T: 'a>(&'a T);
```

As you can see, this `Ref` type is basically just a wrapper around an immutable reference; as the contained field is of that lifetime, clearly the containing structure may not live any longer than it and so it must have that lifetime also. (The `T: *'a*` part is a type bound, saying that the generic type `T` must live for at least `*'a*`; without this it would not work, for as already discussed it clearly wouldn’t make sense to have a reference living longer than the object it refers to.)

The fourth and final of these (`**dyn** Trait + *'a*`) is, I think, the most interesting, and it’s worth spending a short time on trait objects, because the way in which they fit into this arrangement is *slightly* different from elsewhere.

Trait objects are Rust’s form of safe and convenient *dynamic dispatch* (that’s what the `**dyn**` stands for); they allow you to store arbitrary types that satisfy a trait in the one type. Because of the potential difference in size of the types implementing a trait, trait objects are only usable through a reference of some form; the owned form is thus `**Box**<**dyn** Trait>`, and for references you can have `**&dyn** Trait` and `**&mut dyn** Trait`. This is a very brief and wholly lacking explanation of trait objects; you can find documentation of them elsewhere; a detailed explanation is out of scope for this article.

As stated, a trait object may be of a variety of different types; what, then, is the lifetime of a trait object? The answer is that we must specify it, like `**Box**<**dyn** Trait + ***'static***>`, indicating that the contained object must be `***'static***`, and `**&***'a* (**dyn** Trait + *'a*)`, indicating that the contained object’s lifetime must be at least `*'a*`. Now as it happens, some of these common cases like `+ ***'static***` on a boxed trait object and the duplication of the `*'a*` in the reference are taken care of as default trait bounds—​`**Box**<**dyn** Trait>` is normally equivalent to `**Box**<**dyn** Trait + ***'static***>` and `**&***'a* **dyn** Trait` to `**&***'a* (**dyn** Trait + *'a*)`. [RFC 599, on default object bounds](https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md), treats the current rules on this subject more precisely. [RFC 599 was written before the invention of the `**dyn**` keyword, so you’ll see the old‐style [bare trait object](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#bare-trait-objects) syntax in that document.]

(Note that just as the lifetime on a trait object can be omitted due to *default object bounds*, lifetimes on the other three cases can also often be omitted; the current rules of lifetime elision are covered in [RFC 141](https://github.com/rust-lang/rfcs/blob/master/text/0141-lifetime-elision.md).)

But still you may wonder: why do trait object need a lifetime? I think it’s easiest to just give an example of something that would be bad if it were allowed to compile ([which it isn’t](https://play.rust-lang.org/?code=fn main() %7B%0A    let trait_object: %26dyn AsRef<str> %3D %7B%0A        let string %3D %22I am a String%22.to_owned();%0A        %26string%0A    };%0A    println!(%22The string is {:?}%22%2C trait_object.as_ref());%0A})):

```
fn main() {
    let trait_object: &dyn AsRef<str> = {
        let string = "I am a String".to_owned();
        &string
    };
    println!("The string is {:?}", trait_object.as_ref());
}

```

By the time execution gets to the `*println!*` statement, `string` has been freed, so if this were allowed to compile, `trait_object`, which points to the contents of `string`, would also thus be pointing to freed memory, which is emphatically a Bad Thing™. To maintain memory safety, the boxed trait object cannot live longer than the string it contains a reference to.

### Lifetime positions in generic bounds ###

As shown very basically above, a lifetime can also appear in *type parameter bounds* and *lifetime parameter bounds*. Here are some more examples:

```
struct Ref<'a, T: SomeTrait + AnotherTrait + 'a>(&'a T);

// Suppose we have something that is reading values from a buffer and storing
// the decoded values, which still contain references to parts of the original
// buffer, into a vector. For maximal generality, we have *two* lifetimes,
// though normally one would suffice (this is a poor example since it doesn’t
// benefit from taking two lifetimes). The reference to the vector must not
// outlive the vector and all its items, which must not outlive the buffer.
struct DecodeResult<'buf: 'decoded, 'decoded, T: 'buf> {
	buffer: &'buf [u8],
	decoded: &'decoded mut Vec<T>,
}

```

By type parameter bounds we mean the `T: *'buf*`, stating that the lifetime of the type `T` must be at least `*'buf*` (it must live at least as long as the buffer).

By lifetime parameter bounds we mean the `*'buf*: *'decoded*`, stating that the lifetime `*'buf*` must be at least as long as (must satisfy the constraint) `*'decoded*`. This is extremely rare and is typically only of any use when [subtyping and variance gets involved](https://doc.rust-lang.org/nomicon/subtyping.html#variance).

Really, these sorts of bounds are very similar to the trait object bounds: because generics can be of arbitrary types, you will often need to stipulate a minimal lifetime in order to work with a type. In the `DecodeResult` example above, `T`’s lifetime must be constrained to be at least `*'decoded*`, or else the reference in the `decoded` field would not be valid. Rust could infer this sort of thing, but it would lead to surprises in various areas, so it refrains from the attempt.

Exceptions to “each object can be used exactly once”
----------

There; we’ve covered the four basic rules. There remains still one point to explain. Earlier on I posited the simple rule that each object can be used exactly once; as I also said, this isn’t actually universally true. There are two exceptions to this rule.

###

1. Copy and move semantics

 ###

Objects of types with *move semantics* can only be used once, but this is not true of types with *copy semantics*.

An object has *copy semantics* if it implements [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html). This means that a value of that type can be duplicated simply by copying the bytes of memory. `**i32**`, for example, is `Copy` because it’s just an arbitrary four bytes of data, entirely self‐contained, while `**Box**<T>` is not because it involves a heap allocation that it must own and so a simple copy of the bytes would cause two objects to own the data at the same time, breaking all sorts of invariants and probably eating your laundry.

In the [first code example](#rule-one)’s error, we saw the explanatory note “move occurs because `a` has type `A`, which does not implement the `Copy` trait”. `A` has move semantics; suppose we now implement the `Copy` trait, like this:

```
#[derive(Clone, Copy)]
struct A {}

fn main() {
    let a = A {};
    let b = a;
    let c = a;
}

```

`*#[derive(X)]*` means “hey compiler, I want to implement `X` for this type, I just want the obvious implementation, so can you please do it for me?”

The Rust documentation [explains deriving `Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html#derivable) and [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html#how-can-i-implement-copy).

With the addition of the `Copy` implementation for `A`, it’s changed from having move semantics to having copy semantics, and so the code now compiles.

There is one final matter in this exception to the rule worth mentioning: generics. If you work with a generic type, it will have move semantics unless you specify `Copy` in its bounds. In cases where you wish to copy the value, you might wish to use [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) instead, as it is more general (and all types that implement `Copy` implement `Clone`). Don’t worry about efficiency of cloning versus copying, either; unless the writer of the type explicitly went out of their way to do so (which they should *never* do), cloning and copying will perform identically when optimised.

For more information on this topic, see the [`Copy` docs](https://doc.rust-lang.org/std/marker/trait.Copy.html).

###

1. Reference reborrowing

 ###

A particularly notable example of a type with copy semantics is `**&**T`; you can have as many immutable references to an object alive as you like, so long as there are no mutable references to it at the same time. `**&mut** T`, however, has move semantics, for you’re not allowed to have more than one mutable reference in scope at a time.

So then, why does this example [work](https://play.rust-lang.org/?code=fn take_ref<T>(_: %26T) { }%0Afn take_mut<T>(_: %26mut T) { }%0Afn main() %7B%0A    let mut a %3D 6;%0A    %7B%0A        let a_ref %3D %26a;%0A        take_ref(a_ref);%0A        // Naturally this will work, for &T is Copy.%0A        take_ref(a_ref);%0A    }%0A    %7B%0A        let a_mut %3D %26mut a;%0A        take_mut(a_mut);%0A        // Surely a_mut should have been consumed by the line above?%0A        take_mut(a_mut);%0A    }%0A})?

```
fn take_ref<T>(_: &T) { }
fn take_mut<T>(_: &mut T) { }
fn main() {
    let mut a = 6;
    {
        let a_ref = &a;
        take_ref(a_ref);
        // Naturally this will work, for &T is Copy.
        take_ref(a_ref);
    }
    {
        let a_mut = &mut a;
        take_mut(a_mut);
        // Surely a_mut should have been consumed by the line above?
        take_mut(a_mut);
    }
}

```

There is another exception in play here: reference reborrowing. Where we pass `a_mut` to a function, it effectively becomes `**&mut ***a_mut`. This does mean that for a time two mutable references to the same thing *exist* (one in the caller and one in the function called), but at no point are both *accessible*, so it’s OK.

Note that this exception only applies in situations where the parameter is of the form `**&mut** T`; if it is a generic taken by value (e.g. `**fn**<T>(x: T)`) then a mutable reference you pass in will *not* be reborrowed automatically. You can still use the `**&mut ***x` incantation to manually reborrow it, however.

Summary
----------

Let’s list the main points once again:

1. Each object can be used exactly once. When you use an object it is moved to the new location and is no longer usable in the old. (`Copy` and automatic reference reborrowing are the two exceptions that make this more palatable.)

2. When an object passes out of scope, it is destroyed and is no longer usable.

3. Blocks can produce a value which goes up one level of scope. (As a language feature, this is known as *expression orientation*.)

4. All objects have a lifetime which constrains which scopes they may be moved out of.

These points are all that you need to understand all of Rust’s ownership model. The rules are surprisingly simple, though you can quickly get to fairly complex interactions which make things harder to reason about. Don’t worry if you haven’t understood all of this; if you are trying to learn Rust, go and do some other things for a while and come back to the article again later and you’ll probably find it makes more sense and you can understand how ownership and lifetimes work. When you finally get to that point, you’ll love Rust’s unusual ownership model.

Comments? Questions? Corrections? If you want to contact me about anything in this post, email me at [me@chrismorgan.info](mailto:me@chrismorgan.info) or [discuss it on /r/rust](https://old.reddit.com/r/rust/http://chrismorgan.info/blog/rust-ownership-the-hard-way.html).

Looking for more? My [article about FizzBuzz](/blog/rust-fizzbuzz/) is particularly relevant, dealing with these questions of ownership in a (slightly) practical context. If you’re a beginner, I suggest you also go through the [official documentation](https://doc.rust-lang.org/) with things like its [book](https://doc.rust-lang.org/book/README.html). I have [a few more posts about Rust](/blog/tags/rust/) too.