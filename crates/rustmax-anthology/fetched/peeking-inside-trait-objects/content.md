# Peeking Inside Trait Objects

**Author:** Huon Wilson

**Original URL:** <https://huonw.github.io/blog/2015/01/peeking-inside-trait-objects/>

---

 Peeking inside Trait Objects
==========

 By [Huon Wilson](/about) — 10 Jan 2015

One of the most powerful parts of[the Rust programming language](http://rust-lang.org)<sup id="fnref:version"><a class="footnote" href="#fn:version" rel="footnote" role="doc-noteref">0</a></sup> is the[trait system](http://doc.rust-lang.org/nightly/book/traits.html). They form the basis of the generic system and
polymorphic functions and types. There’s an interesting use of traits,
as so-called “trait objects”, that allows for dynamic polymorphism and
heterogeneous uses of types, which I’m going to look at in more detail
over a short series of posts.

*Update 2015-02-19*: A lot of this document has been copied into[the main book](http://doc.rust-lang.org/nightly/book/trait-objects.html), with improvements and updates.

This post will set the scene, with an introduction to the internals of
a trait object; the remaining posts will look at the `Sized` trait and
“object safety” in detail (a lot of people have encountered trouble
with somewhat abstruse compiler errors about this recently).

<details class="post-series"> <summary class="post-series-title">Other posts in this series on trait objects</summary>

1. [Peeking inside Trait Objects](/blog/2015/01/peeking-inside-trait-objects/)
2. [The Sized Trait](/blog/2015/01/the-sized-trait/)
3. [Object Safety](/blog/2015/01/object-safety/)
4. [Where Self Meets Sized: Revisting Object Safety](/blog/2015/05/where-self-meets-sized-revisiting-object-safety/)

</details>

Traits
----------

A simple example of a `trait` is this `Foo`. It has one method that is
expected to return a `String`, and, in the real world, there would be
some expectation about what the string would mean, but this is just a
blog, so you’re free to make up your own favourite meaning.

```
1
2
3
trait Foo {
    fn method(&self) -> String;
}

```

This can then be implemented for certain types, stating that these
types satisfy whatever behaviours the trait is trying to summarise and
allow polymorphism over. For example, bytes and strings are `Foo`,
apparently:

```
1
2
3
4
5
6
impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}
impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

```

There’s two basic ways to use traits to be polymorphic:

The first and most common are generic functions like `fn func<T:
Foo>(x: &T)`. These are implemented via monomorphisation, the compiler
creates a specialised version of the generic function for every type
used with it. This has some upsides—static dispatching of any
method calls<sup id="fnref:upsides"><a class="footnote" href="#fn:upsides" rel="footnote" role="doc-noteref">1</a></sup>, allowing for inlining and hence usually higher
performance—and some downsides—causing code bloat due to
many copies of the same function existing in the binary, one for each
type<sup id="fnref:one-per-type"><a class="footnote" href="#fn:one-per-type" rel="footnote" role="doc-noteref">2</a></sup>.

Fortunately, there’s second option if that trade-off is inappropriate,
or if being required to know every type everywhere is impossible or
undesirable.

Trait objects
----------

Trait objects, like `&Foo` or `Box<Foo>`, are normal values that store
a value of *any* type that implements the given trait, where the
precise type can only be known at runtime. The methods of the trait
can be called on a trait object via a special record of function
pointers (created and managed by the compiler).

A function that takes a trait object—say `fn func(x: &Foo)`—is not
specialised to each of the types that implements `Foo`: only one copy
is generated, often (but not always) resulting in less code
bloat. However, this comes at the cost of requiring slower virtual
function calls, and effectively inhibiting any chance of inlining and
related optimisations from occurring.

Trait objects are both simple and complicated: their core
representation and layout is quite straight-forward, but there are
some curly error messages and surprising behaviours to discover.

Obtaining a trait object
----------

There’s two similar ways to get a trait object value: casts and
coercions. If `T` is a type that implements a trait `Foo` (e.g. `u8`for the `Foo` above), then the two ways to get a `Foo` trait object
out of a pointer to `T` look like:

```
1
2
3
4
5
6
7
8
9
10
let ref_to_t: &T = ...;

// `as` keyword for casting
let cast = ref_to_t as &Foo;

// using a `&T` in a place that has a known type of `&Foo` will implicitly coerce:
let coerce: &Foo = ref_to_t;

fn also_coerce(_unused: &Foo) {}
also_coerce(ref_to_t);

```

These trait object coercions and casts also work for pointers like`&mut T` to `&mut Foo` and `Box<T>` to `Box<Foo>`, but that’s all at
the moment. Other than some bugs, coercions and casts are identical.

This operation can be seen as “erasing” the compiler’s knowledge about
the specific type of the pointer, and hence trait objects are
sometimes referred to “type erasure”.

Representation
----------

Let’s start simple, with the runtime representation of a trait
object. The `std::raw` module contains structs with layouts that are
the same as the complicated build-in types,[including trait objects](http://doc.rust-lang.org/nightly/std/raw/struct.TraitObject.html):

```
1
2
3
4
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}

```

That is, a trait object like `&Foo` consists of a “data” pointer and a
“vtable” pointer.

The data pointer addresses the data (of some unknown type `T`) that
the trait object is storing, and the vtable pointer points to the[vtable](https://en.wikipedia.org/wiki/Virtual_method_table) (“virtual method table”) corresponding to the implementation
of `Foo` for `T`.

A vtable is essentially a struct of function pointers, pointing to the
concrete piece of machine code for each method in the
implementation. A method call like `trait_object.method()` will
retrieve the correct pointer out of the vtable and then do a dynamic
call of it. For example:

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30
31
32
33
34
35
36
37
38
39
40
41
42
43
44
45
46
struct FooVtable {
    destructor: fn(*mut ()),
    size: usize,
    align: usize,
    method: fn(*const ()) -> String,
}

// u8:

fn call_method_on_u8(x: *const ()) -> String {
    // the compiler guarantees that this function is only called
    // with `x` pointing to a u8
    let byte: &u8 = unsafe { &*(x as *const u8) };

    byte.method()
}

static Foo_for_u8_vtable: FooVtable = FooVtable {
    destructor: /* compiler magic */,
    size: 1,
    align: 1,

    // cast to a function pointer
    method: call_method_on_u8 as fn(*const ()) -> String,
};

// String:

fn call_method_on_String(x: *const ()) -> String {
    // the compiler guarantees that this function is only called
    // with `x` pointing to a String
    let string: &String = unsafe { &*(x as *const String) };

    string.method()
}

static Foo_for_String_vtable: FooVtable = FooVtable {
    destructor: /* compiler magic */,
    // values for a 64-bit computer, halve them for 32-bit ones
    size: 24,
    align: 8,

    method: call_method_on_String as fn(*const ()) -> String,
};

```

(The `call_method_on_...` functions could also be UFCS: `<... as
Foo>::method`, but that’s somewhat less clear.)

The `destructor` field in each vtable points to a function that will
clean up any resources of the vtable’s type, for `u8` it is trivial,
but for `String` it will free the memory. This is necessary for owning
trait objects like `Box<Foo>`, which need to clean-up both the `Box`allocation and as well as the internal type when they go out of
scope. The `size` and `align` fields store the size of the erased
type, and its alignment requirements; these are essentially unused at
the moment since the information is embedded in the destructor, but
will be used in future, as trait objects are progressively made more
flexible.

Suppose we’ve got some values that implement `Foo`, the explicit form
of construction and use of `Foo` trait objects might look a bit like
(ignoring the type mismatches: they’re all just pointers anyway):

```
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
let a: String = "foo".to_string();
let x: u8 = 1;

// let b: &Foo = &a;
let b = TraitObject {
    // store the data
    data: &a,
    // store the methods
    vtable: &Foo_for_String_vtable
};

// let y: &Foo = x;
let y = TraitObject {
    // store the data
    data: &x,
    // store the methods
    vtable: &Foo_for_u8_vtable
};

// b.method();
(b.vtable.method)(b.data);

// y.method();
(y.vtable.method)(y.data);

```

If `b` or `y` were owning trait objects (`Box<Foo>`), there would be a`(b.vtable.destructor)(b.data)` (respectively `y`) call when they went
out of scope.

### Why pointers? ###

The use of language like “fat pointer” implies that a trait object is
always a pointer of some form, but why? I wrote above that

>
>
> [Trait objects] are normal values and can store a value of *any* type
> that implements the given trait, where the precise type can only
> be known at runtime.
>
>

Rust does not put things behind a pointer by default, unlike many
managed languages, so types can have different sizes. Knowing the size
of the value at compile time is important for things like passing it
as an argument to a function, moving it about on the stack and
allocating (and deallocating) space on the heap to store it.

For `Foo`, we would need to have a value that could be at least either
a `String` (24 bytes) or a `u8` (1 byte), as well as any other type
for which dependent crates may implement `Foo` (any number of bytes at
all). There’s no way to guarantee that this last point can work if the
values are stored without a pointer, because those other types can be
arbitrarily large.

Putting the value behind a pointer means the size of the value is not
relevant when we are tossing a trait object around, only the size of
the pointer itself.

Comments:

* [/r/rust](http://www.reddit.com/r/rust/comments/2rutqb/peeking_inside_trait_objects/)

Share this on

* [Bluesky](https://bsky.app/intent/compose?text=Peeking%20inside%20Trait%20Objects%20by%20@huonw.bsky.social%20https://huonw.github.io/blog/2015/01/peeking-inside-trait-objects/)
* [Twitter](https://twitter.com/intent/tweet?text=Peeking%20inside%20Trait%20Objects%20%23rustlang&url=https://huonw.github.io/blog/2015/01/peeking-inside-trait-objects/&via=huon_w)
* [Facebook](https://facebook.com/sharer.php?u=https://huonw.github.io/blog/2015/01/peeking-inside-trait-objects/)
* [Reddit](https://www.reddit.com/submit?url=https://huonw.github.io/blog/2015/01/peeking-inside-trait-objects/&title=Peeking%20inside%20Trait%20Objects)

1. It’s generally good practice for Rust posts to mention their version due to language instability, but this post and the series won’t have much real runnable code and the concepts described are pretty stable… but habits die hard: `rustc 1.0.0-nightly (44a287e6e 2015-01-08 17:03:40 -0800)`.

   This should also be the 1.0.0-alpha release (speaking of which,
   the language instability should be starting to settle down now). [↩](#fnref:version)

2. Static dispatching isn’t *guaranteed* to be an upside: compilers aren’t perfect and may “optimise” code to become slower. For example, functions inlined too eagerly will bloating the instruction cache (cache rules everything around us). This is part of the reason that `#[inline]` and `#[inline(always)]` that should be used carefully, and one reason why using a trait object—with its dynamic dispatch—is sometimes more efficient.

   However, the common case is that it is more efficient to use
   static dispatch, and one can always have a thin
   statically-dispatched wrapper function that does a dynamic, but
   not vice versa, meaning static calls are more flexible. The
   standard library tries to be statically dispatched where possible
   for this reason. [↩](#fnref:upsides)

3. There’s no guarantee that there will actually be a copy of the function for each type that implements the trait, or even for each type that is used with the function, since the compiler is free to combine copies if it can tell that sharing the code would not change semantics. But, in general, this optimisation doesn’t trigger. [↩](#fnref:one-per-type)