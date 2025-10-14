# Rust for Functional Programmers

**Author:** Raphael Poss

**Original:** [http://science.raphael.poss.name/rust-for-functional-programmers.html](http://science.raphael.poss.name/rust-for-functional-programmers.html)

---

# Rust for Functional Programmers

**Author:** Raphael Poss

**Original URL:** <http://science.raphael.poss.name/rust-for-functional-programmers.html>

---

[ Rust for functional programmers ](https://dr-knz.net/rust-for-functional-programmers.html)
==========

#### Contents ####

Contents

* [Prologue](#prologue)
* [Why you should learn Rust](#why-you-should-learn-rust)
* [Straightforward equivalences](#straightforward-equivalences)
* [Traits: Rust’s type classes](#traits-rust-s-type-classes)
* [Ad-hoc objects and methods](#ad-hoc-objects-and-methods)
* [Safe references](#safe-references)
* [Lifetime and storage, and managed objects](#lifetime-and-storage-and-managed-objects)
* [Shared objects: Rc and Arc](#shared-objects-rc-and-arc)
* [Macros and meta-programming](#macros-and-meta-programming)
* [Literals](#literals)
* [Acknowledgements](#acknowledgements)
* [References](#references)

This post follows up on [OCaml for Haskellers](http://blog.ezyang.com/2010/10/ocaml-for-haskellers/) from
Edward Z. Yang (2010) and my own [Haskell for OCaml programmers](https://dr-knz.net/haskell-for-ocaml-programmers.html)from earlier this year.

---

Note

The latest version of this document can be found online at[https://dr-knz.net/rust-for-functional-programmers.html](https://dr-knz.net/rust-for-functional-programmers.html).
Alternate formats:[Source](https://dr-knz.net/rust-for-functional-programmers.txt),[PDF](https://dr-knz.net/rust-for-functional-programmers.pdf).

[Prologue](#toc-entry-1)
----------

Rust for C programmers != Rust for functional programmers.

For experienced C programmers with little experience in anything else,
Rust presents many new features and strange constructs that look and feel
relatively arcane. Even to C++, Java or C# programmers, Rust looks
foreign: its has objects but not classes, it has “structured enums”, a
strange “match” statement with no equivalent in the C family, it
prevents from assigning twice to the same variable, and all manners of
strange rules that were unheard of in the C family.

Why is that?

Although no current Rust manual dares putting it so bluntly, **Rust is
a functional language**, inspired from recent advances in
programming language design.

The problem faced by Rust advocates is that “functional programming,”
both as a term and as a discipline, has developed many stigmas over
the last 30 years: functional programs are “unreadable”, they are
relatively slow compared to C, they require heavyweight machinery
during execution (at least a garbage collector, often many functional
wrappers around system I/O), they are difficult to learn, they use a strange syntax, etc.

Trying to describe Rust as a functional language to C programmers, its
primary audience, would be a tough sell indeed. This is why the
official Rust manuals and tutorials duly and properly explain how to
use Rust for low-level tasks, and care to explain step-by-step the Rust
equivalents to many C programming patterns, without refering to other more modern languages.

*This is all and well, but what if you already know about functional programming?*

For experienced users of Haskell, OCaml, etc., yet another
detailed manual that presents the basics of programming with
a functional flavor are a bore to read. Tailored to this audience, the
text below constitutes **a fast-track introduction to Rust**, from the
functional perspective.

[Why you should learn Rust](#toc-entry-2)
----------

For better or for worse, most hardware processors will continue to be
based on program counters, registers and addressable memory for the
foreseeable future. This is where we were in the 1970’s, when C was
designed, and this is where we are still today, and this is precisely
why C is still in prevalent use.
Specifically, the C *abstract machine model* is the snuggest fit for
most hardware platforms, and it is therefore a good level of abstraction to
build low-level system software like interrupt service
routines, garbage collectors and virtual memory managers.

However, the “user interface” of the C language, in particular *its
preprocessor and its type system*, have aged tremendously. For anyone
who learned anything newer, *they frankly suck*.

This is where Rust has been designed: **Rust keeps the C abstract
machine model but innovates on the language interface**. Rust is
expressive, its type system makes system code safer, and its powerful
meta-programming facilities enable new ways to generate code automatically.

Note however that Rust is not yet fully stable at the time of this writing: it
is still subject to change with little notice, and the official documentation
is not fully synchronized with the implementation.

---

[Straightforward equivalences](#toc-entry-3)
----------

| Syntax |  Type   | Ocaml |            Haskell             |
|--------|---------|-------|--------------------------------|
|  `()`  |   ()    | `()`  |              `()`              |
| `true` |  bool   |`true` |             `True`             |
|`false` |  bool   |`false`|            `False`             |
| `123`  |(integer)|       |    `123 {- :: Num a => a }`    |
|`0x123` |(integer)|       |   `0x123 {- :: Num a => a }`   |
| `12.3` | (float) |       |`12.3 {- :: Fractional a => a }`|
| `'a'`  |  char   |       |             `'a'`              |
|`"abc"` |   str   |`"abc"`|            `"abc"`             |
| `b'a'` |   u8    | `'a'` |  `toEnum$fromEnum$'a'::Word8`  |
| `123i` |   int   | `123` |          `123 :: Int`          |
|`123i32`|   i32   |`123l` |         `123 :: Int32`         |
|`123i64`|   i64   |`123L` |         `123 :: Int64`         |
| `123u` |  uint   |       |         `123 :: Word`          |

Like in Haskell, Rust number literals without a suffix do not have a
predefined type. Their actual type is inferred from the context. Rust
character literals can represent any Unicode scalar value, in contrast
to OCaml’s character which can only encode Latin-1 characters. Rust’s
other literal forms are presented in [Literals](#literals) below.

Primitive types:

|Rust|Haskell|OCaml|                  Description                   |
|----|-------|-----|------------------------------------------------|
| () |  ()   |unit |                   Unit type                    |
|bool| Bool  |bool |                  Boolean type                  |
|int |  Int  | int |    Signed integer, machine-dependent width     |
|uint| Word  |     |   Unsigned integer, machine-dependent width    |
| i8 | Int8  |     |  8 bit wide signed integer, two’s complement   |
|i16 | Int16 |     |  16 bit wide signed integer, two’s complement  |
|i32 | Int32 |int32|  32 bit wide signed integer, two’s complement  |
|i64 | Int64 |int64|  64 bit wide signed integer, two’s complement  |
| u8 | Word8 |char |          8 bit wide unsigned integer           |
|u16 |Word16 |     |          16 bit wide unsigned integer          |
|u32 |Word32 |     |          32 bit wide unsigned integer          |
|u64 |Word64 |     |          64 bit wide unsigned integer          |
|f32 | Float |     |     32-bit IEEE 754 binary floating-point      |
|f64 |Double |float|     64-bit IEEE 754 binary floating-point      |
|char| Char  |     |Unicode scalar value (non-surrogate code points)|
|str |       |     |        UTF-8 encoded character string.         |

The type `str` in Rust is special: it is *primitive*, so that the
compiler can optimize certain string operations; but it is not *first
class*, so it is not possible to define variables of type `str` or
pass `str` values directly to functions. To use Rust strings in
programs, one should use string references, as described
later below.

Operators equivalences:

```
==  !=  <  >  <=  >=  && ||   // Rust
=   <>  <  >  <=  >=  && ||   (* OCaml *)
==  /=  <  >  <=  >=  && ||   {- Haskell -}

+  +  +  +  -  -  *  *  /     /  %     !    // Rust
+  +. @  ^  -  -. *  *. /     /. mod   not  (* OCaml *)
+  +  ++ ++ -  -  *  *  `div` /  `mod` not  {- Haskell -}

&    |   ^    <<     >>     !          // Rust
land lor lxor [la]sl [la]sr lnot       (* OCaml *)
.&.  .|. xor  shiftL shiftR complement {- Haskell -}

```

Note that Rust uses `!` both for boolean negation and for the unary
bitwise NOT operator on integers. The unary `~` has a different
meaning in Rust than in C, detailed later below.

Compound expressions:

|     Rust      |      OCaml       |    Haskell    |                     Name                     |
|---------------|------------------|---------------|----------------------------------------------|
|`R{a:10, b:20}`|  `{a=10; b=20}`  |`R{a=10, b=20}`|              Record expression               |
|`R{a:30, ..z}` | `{z with a=30}`  |   `z{a=30}`   |        Record with functional update         |
|   `(x,y,z)`   |    `(x,y,z)`     |   `(x,y,z)`   |               Tuple expression               |
|     `x.f`     |      `x.f`       |     `f x`     |               Field expression               |
|   `[x,y,z]`   |   `[|x;y;z|]`    |               |         Array expression, fixed size         |
|  `[x, ..10]`  |                  |               |Array expression, fixed repeats of first value|
|    `x[10]`    |     `x.(10)`     |    `x!10`     |      Index expression (vectors/arrays)       |
|    `x[10]`    |     `x.[10]`     |    `x!!10`    |          Index expression (strings)          |
|   `{x;y;z}`   |`begin x;y;z end` |               |               Block expression               |
|   `{x;y;}`    |`begin x;y;() end`|               |      Block expression (ends with unit)       |

Note that the value of a block expression is
the value of the last expression in the block, except when the block
ends with a semicolon, in which case its value is `()`.

Functions types and definitions:

|```<br/>// Rust<br/>// f : |int,int| -> int<br/>fn f (x:int, y:int) -> int { x + y };<br/><br/>// fact : |int| -> int<br/>fn fact (n:int) -> int {<br/>   if n == 1 { 1 }<br/>   else { n * fact(n-1) }<br/>}<br/><br/>```|```<br/>(* OCaml *)<br/>(* val f : int * int -> int *)<br/>let f (x, y) = x + y<br/><br/>(* val fact : int -> int *)<br/>let rec fact n =<br/>    if n = 1 then 1<br/>    else n * fact (n-1)<br/><br/>```|```<br/>{- Haskell -}<br/>f :: (Int, Int) -> Int<br/>f (x, y) = x + y<br/><br/>fact :: Int -> Int<br/>fact n =<br/>    if n = 1 then 1<br/>    else n * fact(n-1)<br/><br/>```|
|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

Pattern match and guards:

|```<br/>// Rust<br/>match e {<br/>  0           => 1,<br/>  t @ 2       => t + 1,<br/>  n if n < 10 => 3,<br/>  _           => 4<br/>}<br/><br/>```|```<br/>(* OCaml *)<br/>match e with<br/> | 0             -> 1<br/> | 2 as t        -> t + 1<br/> | n when n < 10 -> 3<br/> | _             -> 4<br/><br/>```|```<br/>{- Haskell -}<br/>case e of<br/>  0          -> 1<br/>  t @ 2      -> t + 1<br/>  n | n < 10 -> 3<br/>  _          -> 4<br/><br/>```|
|---------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------|

Recursion with side effects:

|```<br/>// Rust<br/>fn collatz(n:uint) {<br/>  let v = match n % 2 {<br/>     0 => n / 2,<br/>     _ => 3 * n + 1<br/>  }<br/>  println!("{}", v);<br/>  if v != 1 { collatz(v) }<br/>}<br/>fn main() { collatz(25) }<br/><br/>```|```<br/>(* OCaml *)<br/>let rec collatz n =<br/>    let v = match n % 2 with<br/>       | 0 -> n / 2<br/>       | _ -> 3 * n + 1<br/>    in<br/>    Printf.printf "%d\n" v;<br/>    if v <> 1 then collatz v<br/><br/>let _ = collatz 25<br/><br/>```|```<br/>{- Haskell -}<br/>collatz n = do<br/>    let v = case n `mod` 2 of<br/>            0 -> n `div` 2<br/>            _ -> 3 * n + 1<br/><br/>    putStrLn $ show v<br/>    when (v /= 1) $ collatz v<br/><br/>main = collatz 25<br/><br/>```|
|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

Obviously, Rust uses strict (eager) evaluation and functions can
contain side effecting expressions, just like in OCaml.

Note that Rust does not (yet) guarantee tail call elimination,
although the underlying LLVM code generator is smart enough that it
should work for the function above. When in doubt, the following is equivalent:

```
let mut n = 25u;
while n != 1 {
   n = if n % 2 == 0 { n / 2 }
       else { 3 * n + 1 }
   println("{}", n);
}

```

Record types, expressions and field access:

|```<br/>// Rust<br/>struct Point {<br/>  x : int,<br/>  y : int<br/>}<br/><br/>let v = Point {x:1, y:2};<br/>let s = v.x + v.y<br/><br/>```|```<br/>(* OCaml *)<br/>type Point = {<br/>  x : int;<br/>  y : int<br/>}<br/><br/>let v = { x = 1; y = 2 };<br/>let s = v.x + v.y<br/><br/>```|```<br/>{- Haskell -}<br/>data Point = Point {<br/>   x :: Int,<br/>   y :: Int<br/>}<br/><br/>v = Point {x = 1, y = 2}<br/>s = (x v) + (y v)<br/><br/>```|
|-------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|

Free type parameters (generic data and function types):

|```<br/>// Rust<br/>type Pair<a,b> = (a,b)<br/><br/>// id<t> : |t| -> t<br/>fn id<t>(x : t) -> t { x }<br/><br/>```|```<br/>(* OCaml *)<br/>type ('a, 'b) pair = 'a * 'b<br/><br/>(* val id : 't -> 't *)<br/>let id x = x<br/><br/>```|```<br/>{- Haskell -}<br/>type Pair a b = (a, b)<br/><br/>id :: t -> t<br/>id x = x<br/><br/>```|
|-------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------|

Algebraic data types:

|```<br/>// Rust<br/>enum Option<T> {<br/>   None,<br/>   Some(T)<br/>}<br/>// x : Option<t><br/>match x {<br/>  None    => false,<br/>  Some(_) => true<br/>}<br/><br/>```|```<br/>(* OCaml *)<br/>type 't option =<br/>    None<br/>  | Some of 't<br/><br/>(* x : t option *)<br/>match x with<br/>  | None -> false<br/>  | Some _ -> true<br/><br/>```|```<br/>{- Haskell -}<br/>data Maybe a =<br/>    Nothing<br/>  | Just a<br/><br/>{- x : Maybe t -}<br/>case x of<br/>   Nothing -> False<br/>   Just _ -> True<br/><br/>```|
|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

Lambda expressions and higher-order functions:

|```<br/>// Rust<br/>// ||int,int| -> int, int| -> int<br/>fn ff(f:|int,int|->int, x:int) -> int<br/>{ f (x, x) }<br/><br/>// m2 : |int| -> int<br/>fn m2(n : int) -> int<br/>{ ff ((|x,y| { x + y }), n) }<br/><br/>```|```<br/>(* OCaml *)<br/>(* (int*int->b)*int -> int *)<br/>let ff (f, x) =<br/>    f (x, x)<br/><br/>(* m2 : int -> int *)<br/>let m2 n =<br/>    ff ((fun(x,y) -> x + y), n)<br/><br/>```|```<br/>{- Haskell -}<br/>ff :: ((int,int)->int, int) -> int<br/>ff (f, x) =<br/>   f (x, x)<br/><br/>m2 :: Int -> Int<br/>m2 n =<br/>   ff ((\(x,y) -> x + y), n)<br/><br/>```|
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

[Traits: Rust’s type classes](#toc-entry-4)
----------

Rust’s “traits” are analogous to Haskell’s type classes.

The main difference with Haskell is that traits only intervene for
expressions with dot notation, ie. of the form `a.foo(b)`.

For C++/Java/C#/OCaml programmers, however, traits should
not be confused with traditional object classes. They are
really type classes: it is possible to add traits to arbitrary
data types, including the primitive types!

An example:

|```<br/>// Rust<br/>trait Testable {<br/>    fn test(&self) -> bool<br/>}<br/><br/>impl Testable for int {<br/>    fn test(&self) -> bool {<br/>       if *self == 0 { false }<br/>       else { true }<br/>    }<br/>}<br/><br/>fn hello(x:int) -> bool {<br/>   x.test()<br/>}<br/><br/>```|```<br/>{- Haskell -}<br/>class Testable a where<br/>    test :: a -> Bool<br/><br/>instance Testable Int where<br/>    test n =<br/>       if n == 0 then False<br/>       else True<br/><br/>hello :: Int -> Bool<br/>hello x =<br/>    test x<br/><br/>```|
|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

In a trait method declaration, the identifier “`self`” denotes
the actual object on which the method is applied.

Like in Haskell, Rust traits can be used for operator overloading. For
example, if one defines a new sum type for Peano integers:

|```<br/>// Rust<br/>enum Peano {<br/>   Zero,<br/>   Succ(Box<Peano>)<br/>}<br/><br/>```|```<br/>{- Haskell -}<br/>data Peano =<br/>    Zero<br/>  | Succ Peano<br/><br/>```|
|----------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------|

Then one can overload the comparison operator `==` between Peano
integers by instantiating the `PartialEq` class:

|```<br/>// Rust<br/>impl PartialEq for Peano {<br/>    fn eq(&self, other:&Peano) -> bool {<br/>      match (self, other) {<br/>        (&Zero, &Zero)               => true,<br/>        (&Succ(ref a), &Succ(ref b)) => (a == b),<br/>        (_, _)                       => false<br/>      }<br/>    }<br/>}<br/><br/>```|```<br/>{- Haskell -}<br/>instance Eq Peano where<br/>    (==) self other =<br/>        case (self, other) of<br/>          | (Zero, Zero)     -> True<br/>          | (Succ a, Succ b) -> (a == b)<br/>          | (_, _)           -> False<br/><br/>```|
|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

Also, like in Haskell, a trait can provide a default implementation
for a method, to be used when instances omit the specialization:

|```<br/>// Rust<br/>trait PartialEq {<br/>   fn eq(&self, other:&Self) -> bool;<br/>   fn ne(&self, other:&Self) -> bool<br/>   { !self.eq(other) }<br/> }<br/><br/>```|```<br/>{- Haskell -}<br/>class Eq a where<br/>   (==) : a -> a -> Bool<br/>   (!=) : a -> a -> Bool<br/>   (!=) x y = not (x == y)<br/><br/>```|
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|

In the method declarations inside a trait declaration, the identifier
“`Self`” refers to the actual type on which the trait applies.

Each overloadable operator in Rust has a corresponding
trait in the standard library:

|Expression| Expands to  |         Trait         |            Equivalent Haskell class/method            |
|----------|-------------|-----------------------|-------------------------------------------------------|
| `a == b` |  `a.eq(b)`  |  std::cmp::PartialEq  |    `class PartialEq a where (==) : a -> a -> bool`    |
| `a != b` |  `a.ne(b)`  |  std::cmp::PartialEq  |    `class PartialEq a where (/=) : a -> a -> bool`    |
| `a < b`  |  `a.lt(b)`  | std::cmp::PartialOrd  |    `class PartialOrd a where (<) : a -> a -> bool`    |
| `a > b`  |  `a.gt(b)`  | std::cmp::PartialOrd  |    `class PartialOrd a where (>) : a -> a -> bool`    |
| `a <= b` |  `a.le(b)`  | std::cmp::PartialOrd  |`class PartialOrd a : Eq a where (<=) : a -> a -> bool`|
| `a >= b` |  `a.ge(b)`  | std::cmp::PartialOrd  |`class PartialOrd a : Eq a where (>=) : a -> a -> bool`|
| `a + b`  | `a.add(b)`  | std::ops::Add\<b,c\>  |       `class Add a b c where (+) : a -> b -> c`       |
| `a - b`  | `a.sub(b)`  | std::ops::Sub\<b,c\>  |       `class Sub a b c where (-) : a -> b -> c`       |
| `a * b`  | `a.mul(b)`  | std::ops::Mul\<b,c\>  |       `class Mul a b c where (*) : a -> b -> c`       |
| `a / b`  | `a.div(b)`  | std::ops::Div\<b,c\>  |       `class Div a b c where (/) : a -> b -> c`       |
| `a % b`  | `a.rem(b)`  | std::ops::Rem\<b,c\>  |       `class Rem a b c where (%) : a -> b -> c`       |
|   `-a`   |  `a.neg()`  |  std::ops::Neg\<c\>   |          `class Neg a c where (-) : a -> c`           |
|   `!a`   |  `a.not()`  |  std::ops::Not\<c\>   |          `class Not a c where (!) : a -> c`           |
|   `*a`   | `a.deref()` | std::ops::Deref\<c\>  |         `class Deref a c where (*) : a -> c`          |
| `a & b`  |`a.bitand(b)`|std::ops::BitAnd\<b,c\>|     `class BitAnd a b c where (&) : a -> b -> c`      |
| `a | b`  |`a.bitor(b)` |std::ops::BitOr\<b,c\> |      `class BitOr a b c where (|) : a -> b -> c`      |
| `a ^ b`  |`a.bitxor(b)`|std::ops::BitXor\<b,c\>|     `class BitXor a b c where (^) : a -> b -> c`      |
| `a << b` | `a.shl(b)`  | std::ops::Shl\<b,c\>  |      `class Shl a b c where (<<) : a -> b -> c`       |
| `a >> b` | `a.shr(b)`  | std::ops::Shr\<b,c\>  |      `class Shr a b c where (>>) : a -> b -> c`       |

The `for` loop uses the special trait `std::iter::Iterator`, as follows:

```
// Rust

// the following expression:
[<label>] for <pat> in <iterator expression> {
   body...;
}

// ... expands to (internally):
match &mut <iterator expression> {
  _v => [label] loop {
    match _v.next() {
       None => break,
       Some(<pat>) => { body... }
    }
  }
}

```

The method `next` is implemented by `Iterator`. The return type
of `next` is `Option`, which can have values `None`, to mean
“nothing left to iterate”, or `Some(x)`, to mean the next iteration
value is `x`.

[Ad-hoc objects and methods](#toc-entry-5)
----------

In addition to the mechanism offered by traits, any `struct` or`enum` can be *decorated* with one or more method interface(s)
using “`impl`”, separately from its definition and/or in different modules:

|```<br/>// Rust<br/>struct R {x:int};<br/><br/>// in some module:<br/>impl R {<br/> fn hello(&self) {<br/>  println!("hello {}", self.x);<br/><br/> }<br/>}<br/>// possibly somewhere else:<br/>impl R {<br/> fn world(&self) {<br/>  println!("world");<br/> }<br/>}<br/>// Example use:<br/>fn main() {<br/>  let v = R {x:10};<br/>  v.hello();<br/>  v.world();<br/><br/>  (R{x:20}).hello();<br/>}<br/><br/>```|```<br/>// Rust<br/>enum E {A, B};<br/><br/>// in some module:<br/>impl E {<br/> fn hello(&self) {<br/>   match self {<br/>    &A => println!("hello A"),<br/>    &B => println!("hello B")<br/>   }<br/> }<br/>}<br/>// possibly somewhere else:<br/>impl E {<br/> fn world(&self) {<br/>  println!("world");<br/> }<br/>}<br/>// Example use:<br/>fn main() {<br/>  let v = A;<br/>  v.hello();<br/>  v.world();<br/><br/>  B.hello();<br/>}<br/><br/>```|
|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

[Safe references](#toc-entry-6)
----------

Like in C, by default function parameters in Rust are passed
by value. For large data types, copying the data may be
expensive so one may want to use *references* instead.

For any object `v` of type `T`, it is possible to create a*reference* to that object using the expression “`&v`”.
The reference itself then has type `&T`.

```
#[derive(Clone,Copy)]
struct Pt { x:f32, y:f32 }

// This works, but may be expensive:
fn dist1(p1 : Pt, p2: Pt) -> f32 {
   let xd = p1.x - p2.x;
   let yd = p1.y - p2.y;
   (xd * xd + yd * yd).sqrt()
}

// Same, using references:
fn dist2(p1 : &Pt, p2: &Pt) -> f32 {
   let xd = p1.x - p2.x;
   let yd = p1.y - p2.y;
   (xd * xd + yd * yd).sqrt()
}

// Usage:
fn main() {
   let a = Pt { x:1.0, y:2.0 };
   let b = Pt { x:0.0, y:3.0 };
   println!("{}", dist1(a, b));
   println!("{}", dist2(&a, &b));
}

```

As illustrated in this example, Rust provides some syntactic
sugar for `struct` references: it is possible to write `p1.x` if`p1` is of type `&Pt` and `Pt` has a field named `x`. This
sugar is also available for method calls (`x.foo()`).

However, in many other cases, either the referenced value must be “retrieved”
using the unary `*` operator, or patterns must be matched using `&`:

```
fn add3(x : &int) -> int {
   println!("x : {}", *x); // OK
   3 + x; // invalid! (+ cannot apply to &int)
   3 + *x; // OK
}

fn test<t>(x : &Option<t>) -> bool {
   match *x {
     None     => false,
     Some(_)  => true
   }
   // Also valid, equivalent:
   match x {
     &None    => false,
     &Some(_) => true
   }
}

```

Simple references not not allow modifying the underlying object;
if mutability by reference is desired, use “`&mut`” as follows:

```
fn incx(x : &int) {
   x = x + 1;   // invalid! (mismatched types in "int& = int")
   *x = *x + 1; // invalid! (*x is immutable)
}

fn inc(x : &mut int) {
   x = x + 1;   // invalid! (x is immutable)
   *x = *x + 1; // OK
}

fn main() {
   inc(&3); // invalid! (3 is immutable)
   inc(&mut 3); // OK, temp var forgotten after call
   let v = 3;
   inc(&v); // invalid! (v is immutable)
   inc(&mut v); // OK, temp var forgotten after call
   let mut w = 3;
   inc(&w); // OK
}

```

Rust’s type system *forbids mutable aliases* via references: it is not
possible to modify the same object using different names via
references, unlike in C. This is done via the concept of **borrowing**: while
ownership of an object is borrowed by a reference, the original variable
cannot be used any more. For example:

```
 fn main() {
    let mut a = 1;
    a = 2; // OK
    println!("{}", a); // OK, prints 2
    // {...} introduces a new scope:
    {
      let ra = &mut a;
      a = 3;   // invalid! (a is borrowed by ra)
      *ra = 3; // OK
      println!("{}", a); // invalid! (a is borrowed)
      println!("{}", *ra); // OK, prints 3
    };
    // end of scope, rb is dropped.
    println!("{}", a); // OK, a not borrowed any more (prints 3)
}

```

Reference types, together with “all values are immutable by default”
and the ownership/borrowing rules, are the core features of Rust’s
type system that make it fundamentally safer than C’s.

[Lifetime and storage, and managed objects](#toc-entry-7)
----------

Note

The concepts and syntax presented in this section are new in Rust
version 0.11. Rust 0.10 and previous versions used different
concepts, terminology and syntax.

Also, at the time of this writing (July 2014) the official Rust
manual and tutorials are not yet updated to the latest language
implementation. This section follows the implementation, not the manuals.

### Introduction ###

Rust is defined over the same *abstract machine model* as C. The
abstract machine has segments of memory, and the language’s run-time
machinery can allocate and deallocate segments of memory over time during
program execution.

In the abstract machine, the two following concepts are defined in
Rust just like in C:

* the *storage* of an object determines the type of memory where the object is stored;
* the *lifetime* or *storage duration* of an object is the segment of
  time from the point an object is allocated to the point where it is deallocated.

All memory-related problems in C come from the fact that C programs
can manipulate references to objects *outside of their lifetime* (ie. before
they are allocated or after they are deallocated), or *outside of their
storage* (ie. at lower or higher addresses in memory).

Rust goes to great lengths to prevent all these problems altogether,
by **ensuring that objects cannot be used outside of their lifetime or
their storage**.

### Storages in Rust vs. C ###

There are four kinds of storage in the C/Rust abstract machine:
static, thread, automatic and allocated.

| Storage |    Type of memory used     |         Example in C          |
|---------|----------------------------|-------------------------------|
| static  |process-wide special segment|        `static int i;`        |
|automatic|           stack            | `int i;` (in function scope)  |
|allocated|        global heap         |`int *i = malloc(sizeof(int));`|
| thread  |      per-thread heap       |    `_Thread_local int i;`     |

In C, the lifetime of an object is solely determined by its storage:

| Storage |                        Lifetime in C                        |
|---------|-------------------------------------------------------------|
| static  |              From program start to termination              |
|automatic|From start of scope to end of scope, one per activation frame|
|allocated|    From `malloc` to `free` (or `mmap` to `munmap`, etc)     |
| thread  |        From object allocation to thread termination         |

In Rust, the lifetime of static and automatic objects
is the same as in C; however:

* Rust introduces a new “box” type with dedicated syntax for heap
  allocated objects, which are called *managed objects*.

  Rust supports multiple management strategies for boxes, *associated
  to different typing rules*.

* The default management strategy for boxes ensures that **boxes are
  uniquely owned**, so that the compiler knows precisely when the
  lifetime of a box ends and where it can be safely deallocated
  without the need for extra machinery like reference counting or
  garbage collection.

* Another strategy is GC, that uses deferred garbage collection: the
  storage for GC boxes is reclaimed and made available for reuse at
  some point when the Rust run-time system can determine that they are
  not needed any more. (This may delay reclamation for an unpredictable
  amount of time.) References to GC boxes need not be unique, so GC boxes
  are an appropriate type to build complex data structures with common
  nodes, like arbitrary graphs.

* Unlike C, **Rust forbids sharing of managed objects across
  threads**: like in Erlang, objects have a single owner for
  allocation, and most objects are entirely private to each
  thread. This eliminates most data races. Also the single thread ownership
  implies that garbage collection for GC objects does not require
  inter-thread synchronization, so it is easier to implement and can
  run faster.

* As with references to normal objects, **Rust forbids mutable
  aliases** with references to managed objects.

* **All vector accesses are bound checked, and references do not support
  pointer arithmetic.**

* Rust strongly discourages all uses of unmanaged pointers, by
  tainting them with the `unsafe` attribute which systematically
  elicits compilers warnings.

### Creating and using boxes ###

The Rust handling of managed objects is relatively simple: the `box`keyword in expressions “puts objects into boxes”, where their lifetime
is managed dynamically. The immutability and ownership of boxes is
handled like with references described earlier:

```
fn main() {

  let b = box 3i; // b has type Box<int>
  b = 4i; // invalid! can't assign int to box<int>
  *b = 4i; // invalid! b is immutable, so is the box
  let mut c = box 3i; // c has type mut Box<int>
  *c = 4i; // OK

  let v = box vec!(1i,2,3); // r1 has type Box<Vec<int>>
  *v.get_mut(0) = 42; // invalid! v is immutable, so is the box
  let mut w = box vec!(1i,2,3);
  *w.get_mut(0) = 42; // OK, rust 0.11.1+ also permits w[0] = 42

  let z = w; // z has type Box<Vec<int>>, captures box
  println!("{}", w); // invalid! box has moved to z

  w = box vec!(2i,4,5); // overwrites reference, not box contents
  println!("{} {}", w, z); // OK, prints [2,4,5] [42,2,3]
}

```

The `box` keyword in expressions is actually a shorthand form for `box(HEAP)`.
The general form “`box(A) E`” places the result of the evaluation of `E`in a memory object allocated by `A`, which is a trait. As of Rust 0.11, the only other
allocator in the standard library is `GC`, for garbage collected objects.

|Expression |  Type  |                            Meaning                            |
|-----------|--------|---------------------------------------------------------------|
|  `box v`  |Box\<T\>|Unique reference to a copy of `v`, shorthand for `box(HEAP) v` |
|`box(GC) v`|Gc\<T\> |       Garbage-collected smart pointer to a copy of `v`        |
|`box(A) v` |??\<T\> |Some kind of smart pointer; `A` must implement a special trait.|

### Recursive data structures ###

Managed objects are the “missing link” to implement proper recursive
algebraic data types:

|```<br/>// Rust<br/>enum Lst<t> {<br/>   Nil,<br/>   Cons(t, Box<Lst<t>>)<br/>}<br/>fn len<t>(l : &Lst<t>) -> uint {<br/>   match *l {<br/>     Nil => 0,<br/>     Cons(_, ref x) => 1 + len(*x)<br/>   }<br/>}<br/><br/>fn main() {<br/>   let l = box Cons('a',<br/>            box Cons('b',<br/>             box Nil));<br/>   println!("{}", len(l));<br/>}<br/><br/>```|```<br/>(* OCaml *)<br/>type 't lst =<br/>    Nil<br/>  | Cons of 't * 't lst<br/><br/>let len l =<br/>  match l with<br/>  | Nil -> 0<br/>  | Cons(_, x) -> 1 + len x<br/><br/>let l = Cons('a',<br/>          Cons('b',<br/>              Nil));<br/>print_int (len l)<br/><br/>```|```<br/>{- Haskell -}<br/>data Lst t =<br/>    Nil<br/>  | Cons t (Lst t)<br/><br/>let len l =<br/>  case l of<br/>    Nil -> 0<br/>    Cons _ x -> 1 + len x<br/><br/>main = do<br/>         let l = Cons 'a'<br/>                      (Cons 'b'<br/>                            Nil)<br/>         putStrLn $ show l<br/><br/>```|
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|

[Shared objects: Rc and Arc](#toc-entry-8)
----------

In complement to the facilities offered by `box`, the Rust standard
library also implements two implementations of **reference counted
wrappers to immutable objects** that can be referred to from multiple owners:

```
use std::rc::Rc;
fn main() {
    let b = Rc::new(3i);
    let c = &b;
    println("{} {}", b, c); // OK
}

```

The use of reference counts ensures that an object gets deallocated
exactly when its last reference is dropped. The trade-off with boxes
is that the reference counter must be updated every time a new
reference is created or a reference is dropped.

Two implementations are provided: `std::rc::Rc` and`std::arc::Arc`. Both offer the same interface. The reason for this
duplication is to offer a controllable trade-off over performance to
programmers: `Rc` does not use memory atomics, so it is more
lightweight and thus faster, however it cannot be shared across
threads. `Arc` does use atomics, is thus slightly less efficient
than `Rc`, but can be used to share data across threads.

[Macros and meta-programming](#toc-entry-9)
----------

The basic syntax of a macro definition is as follows:

```
macro_rules! MACRONAME (
    (PATTERN...) => (EXPANSION...);
    (PATTERN...) => (EXPANSION...);
    ...
)

```

For example, the following macro defines a Pascal-like `for` loop:

```
macro_rules! pfor (
   ($x:ident = $s:expr to $e:expr $body:expr)
       => (match $e { e => {
              let mut $x = $s;
              loop {
                   $body;
                   $x += 1;
                   if $x > e { break; }
              }
          }});
);

// Example use:
fn main() {
     pfor!(i = 0 to 10 {
         println!("{}", i);
     });
}

```

Note how this macro uses the `match` statement to assign a local
name to an expression, so that it does not get evaluated more than once.

Like in Scheme, macros can be recursive. For example, the following
macro uses recursion to implement `pfor` both with and without `step`:

```
macro_rules! pfor (
   ($x:ident = $s:expr to $e:expr step $st:expr $body:expr)
       => (match $e,$st { e, st => {
              let mut $x = $s;
              loop {
                   $body;
                   $x += st;
                   if $x > e { break; }
              }
          }});
   ($x:ident = $s:expr to $e:expr $body:expr)
      => (pfor!($x = $s to $e step 1 $body));
);

// Example use:
fn main() {
     pfor!(i = 0 to 10 step 2 {
         println!("{}", i);
     });
}

```

Macros can also be variadic, in that arbitrary repetitions of a syntax
form can be captured by one macro argument. For example, the following
macro invokes `println!` on each of its arguments, which can
be of arbitrary type:

```
macro_rules! printall (
     ( $( $arg:expr ),*  ) => (
          $( println!("{}", $arg) );*
     );
);

// example use:
fn main() {
    printall!("hello", 42, 3.14);
}

```

The syntax works as follows: on the left hand side (pattern) the form`$( PAT )DELIM*` matches zero or more occurrences of PAT delimited
by `DELIM`; on the right hand side (expansion), the form `$( TEXT)DELIM*` expands to one or more repetitions of `TEXT` separated by`DELIM`. The number of repetitions of the expansion is determined by
the number of matches of the enclosed macro argument(s). In the
example, each argument (separated by commas) is substituted by a
corresponding invocation of `println!`, separated by semicolons.

[Literals](#toc-entry-10)
----------

Rust provides various lexical forms for number literals:

|Rust syntax| Same as |  Type   |Haskell equivalent|OCaml equivalent|
|-----------|---------|---------|------------------|----------------|
|  `123i`   |         |   int   |   `123 :: Int`   |     `123`      |
|  `123u`   |         |  uint   |  `123 :: Word`   |                |
|  `123i8`  |         |   i8    |  `123 :: Int8`   |                |
|  `123u8`  |         |   u8    |  `123 :: Word8`  | `Char.chr 123` |
| `123i16`  |         |   i16   |  `123 :: Int16`  |                |
| `123u16`  |         |   u16   | `123 :: Word16`  |                |
| `123i32`  |         |   i32   |  `123 :: Int32`  |     `123l`     |
| `123u32`  |         |   u32   | `123 :: Word32`  |                |
| `123i64`  |         |   i64   |  `123 :: Int64`  |     `123L`     |
| `123u64`  |         |   u64   | `123 :: Word64`  |                |
| `1_2_3_4` | `1234`  |(integer)|                  |                |
| `1234_i`  | `1234i` |   int   |                  |                |
| `0x1234`  | `4660`  |(integer)|     `0x1234`     |    `0x1234`    |
|`0x1234u16`|`4660u16`|   u16   |`0x1234 :: Word16`|                |
| `0b1010`  |  `10`   |(integer)|                  |    `0b1010`    |
| `0o1234`  |  `668`  |(integer)|     `0o1234`     |    `0o1234`    |
|  `b'a'`   | `97u8`  |   u8    |                  |     `'a'`      |
|  `b"a"`   |`[97u8]` |  [u8]   |                  |                |
|  `12.34`  |         | (float) |     `12.34`      |                |
|`12.34f32` |         |   f32   | `12.34 :: Float` |                |
|`12.34f64` |         |   f64   |`12.34 :: Double` |    `12.34`     |
|  `12e34`  |`1.2e35` | (float) |     `12e34`      |                |
|  `12E34`  |`1.2e35` | (float) |     `12E34`      |                |
| `12E+34`  |`1.2e35` | (float) |     `12E+34`     |                |
| `12E-34`  |`1.2e-33`| (float) |     `12E-34`     |                |
| `1_2e34`  | `12e34` | (float) |                  |                |
| `1_2e3_4` | `12e34` | (float) |                  |                |

Escapes in character, byte and string literals:

|    Syntax    |Same as | Syntax  |Same as|    Syntax    |Same as |
|--------------|--------|---------|-------|--------------|--------|
|   `'\x61'`   | `'a'`  |`b'\x61'`| 97u8  |   `"\x61"`   | `"a"`  |
|    `'\\'`    |`'\x5c'`| `b'\\'` | 92u8  |    `"\\"`    |`"\x5c"`|
|    `'\''`    |`'\x27'`| `b'\''` | 39u8  |    `"\""`    |`"\x22"`|
|    `'\0'`    |`'\x00'`| `b'\0'` |  0u8  |    `"\0"`    |`"\x00"`|
|    `'\t'`    |`'\x09'`| `b'\t'` |  9u8  |    `"\t"`    |`"\x09"`|
|    `'\n'`    |`'\x0a'`| `b'\n'` | 10u8  |    `"\n"`    |`"\x0a"`|
|    `'\r'`    |`'\x0d'`| `b'\r'` | 13u8  |    `"\r"`    |`"\x0d"`|
|  `'\u0123'`  |        |         |       |  `"\u0123"`  |        |
|`'\U00012345'`|        |         |       |`"\U00012345"`|        |

Note that the other common escapes in the C family (`\a`, `\f`,
etc.) are not valid in Rust, nor are octal escapes (eg. `\0123`).

Finally, Rust like Python supports raw strings and multiple string delimiters, to
avoid quoting occurrences of the delimiters within the string:

|      Syntax      |String value|      Syntax       |               Value                |
|------------------|------------|-------------------|------------------------------------|
|     `"foo"`      |   `foo`    |     `b"foo"`      |         `[102u8,111,111]`          |
|    `"fo\"o"`     |   `fo"o`   |    `b"fo\"o"`     |        `[102u8,111,34,111]`        |
|    `r"fo\n"`     |   `fo\n`   |    `rb"fo\n"`     |        `[102u8,111,92,110]`        |
|   `r#"fo\"o"#`   |  `fo\"o`   |   `rb#"fo\"o"#`   |      `[102u8,111,92,34,111]`       |
|  `"foo#\"#bar"`  |`foo#"#bar` |  `b"foo#\"#bar"`  |`[102u8,111,111,35,34,35,98,97,114]`|
|`r##"foo#"#bar"##`|`foo#"#bar` |`rb##"foo#"#bar"##`|`[102u8,111,111,35,34,35,98,97,114]`|

[Acknowledgements](#toc-entry-11)
----------

Many thanks to the numerous commenters on Reddit and Hacker news who
provided high-quality comments and substantively contributed to
improving upon the first version of this article.

[References](#toc-entry-12)
----------

* [The Rust Language Tutorial](http://doc.rust-lang.org/0.10/tutorial.html), version 0.10, April 2014.
* [The Rust Language Tutorial](http://doc.rust-lang.org/0.11.0/tutorial.html), version 0.11.0, July 2014.
* [The Rust Guide](http://doc.rust-lang.org/0.11.0/guide.html), version 0.11.0, July 2014.
* Aaron Turon, [Rust Guidelines](https://aturon.github.io/), 2014.
* Will Yager. [Why Go is not good](http://yager.io/programming/go.html), 2014. Explains how Go lacks many
  features found in Rust and thereby fails to be a modern functional language.
* Edward Z. Yang, [OCaml for Haskellers](http://blog.ezyang.com/2010/10/ocaml-for-haskellers/), October 2010.
* Raphael ‘kena’ Poss, [Haskell for OCaml programmers](https://dr-knz.net/haskell-for-ocaml-programmers.html), March 2014.
* Xavier Leroy et al., [The OCaml system release 4.01](http://caml.inria.fr/pub/docs/manual-ocaml-4.01/), September 2013.

 Like this post? Share on: [BlueSky](https://bsky.app/intent/compose?text=Rust%20for%20functional%C2%A0programmers%20https%3A//dr-knz.net/rust-for-functional-programmers.html) ❄ [Twitter](https://twitter.com/intent/tweet?text=Rust%20for%20functional%C2%A0programmers&url=https%3A//dr-knz.net/rust-for-functional-programmers.html&via=kena42&hashtags=rust,haskell,ocaml,functional-programming,language-comparison,analysis,programming-languages) ❄ [HN](https://news.ycombinator.com/submitlink?t=Rust%20for%20functional%C2%A0programmers&u=https%3A//dr-knz.net/rust-for-functional-programmers.html) ❄ [Reddit](https://www.reddit.com/submit?url=https%3A//dr-knz.net/rust-for-functional-programmers.html&title=Rust%20for%20functional%C2%A0programmers) ❄ [LinkedIn](https://www.linkedin.com/shareArticle?mini=true&url=https%3A//dr-knz.net/rust-for-functional-programmers.html&title=Rust%20for%20functional%C2%A0programmers&summary=This%20post%20follows%20up%20on%20OCaml%20for%20Haskellers%20from%0AEdward%20Z.%20Yang%20%282010%29%20and%20my%20own%20Haskell%20for%20OCaml%20programmers%0Afrom%20earlier%20this%C2%A0year.%0A%0A%0ANote%0AThe%20latest%20version%20of%20this%20document%20can%20be%20found%20online%20at%0Ahttps%3A//dr-knz.net/rust-for-functional-programmers.html.%0AAlternate%20formats%3A%0ASource%2C%0APDF.%0A%0A%0APrologue%0ARust%20for%20C%20programmers%20%E2%80%A6&source=https%3A//dr-knz.net/rust-for-functional-programmers.html) ❄ [Email](mailto:?subject=Rust%20for%20functional%C2%A0programmers&body=https%3A//dr-knz.net/rust-for-functional-programmers.html)

###### Comments ######

Interested to discuss? Leave your comments below.

[ Comments ](https://dr-knz.net/rust-for-functional-programmers.html#comment_thread)

---

Keep Reading
----------

* [Finding the right tool for the job - FAIL](https://dr-knz.net/finding-the-right-language.html)
* [Haskell for OCaml programmers](https://dr-knz.net/haskell-for-ocaml-programmers.html)
* [Categories from scratch](https://dr-knz.net/categories-from-scratch.html)
* [Abstract Machine Models Also: what Rust got particularly right](https://dr-knz.net/abstract-machine-models.html)
* [Unusual primitives in programming languages](https://dr-knz.net/unusual-primitives-in-programming-languages.html)

---

* « [Categories from scratch](https://dr-knz.net/categories-from-scratch.html)
* [How good are you at programming? A CEFR-like approach to measure programming proficiency](https://dr-knz.net/programming-levels.html) »

#### Reading Time ####

\~19 min read

#### Published ####

Jul 2014

#### Last Updated ####

Jan 2020

#### Category ####

[Programming](https://dr-knz.net/categories#programming-ref)

#### Tags ####

* [analysis 26](https://dr-knz.net/tags#analysis-ref)
* [functional programming 4](https://dr-knz.net/tags#functional-programming-ref)
* [haskell 4](https://dr-knz.net/tags#haskell-ref)
* [language comparison 3](https://dr-knz.net/tags#language-comparison-ref)
* [ocaml 3](https://dr-knz.net/tags#ocaml-ref)
* [programming languages 16](https://dr-knz.net/tags#programming-languages-ref)
* [rust 3](https://dr-knz.net/tags#rust-ref)

#### Stay in Touch ####

[](https://dr-knz.net/feeds/all.rss.xml) [](https://bsky.app/profile/raphaelposs.com)