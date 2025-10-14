# Things Rust Shipped Without

**Author:** Graydon Hoare

**Original URL:** <http://graydon2.dreamwidth.org/218040.html>

---

Well-known things I'm very proud that rust shipped 1.0 without:  

* null pointers  

* array overruns  

* data races  

* wild pointers  

* uninitialized, yet addressable memory  

* unions that allow access to the wrong field  

Less-well-known things I'm very proud that rust shipped 1.0 without:  

* a shared root namespace  

* variables with runtime "before main" static initialization (the .ctors section)  

* a compilation model that relies on textual inclusion (#include) or textual elision (#ifdef)  

* a compilation model that relies on the order of declarations (possible caveat: macros)  

* accidental identifier capture in macros  

* random-access strings  

* UTF-16 or UCS-2 support anywhere outside windows API compatibility routines  

* signed character types  

* (hah! [vertical tab escapes](http://doc.rust-lang.org/reference.html#literals) (as [recently discussed](http://prog21.dadgum.com/76.html)) along with the escapes for bell and form-feed)  

* "accidental octal" from leading zeroes  

* goto (not even as a reserved word)  

* dangling else (or misgrouped control structure bodies of any sort)  

* case fallthrough  

* a == operator you can easily typo as = and still compile  

* a === operator, or any set of easily-confused equality operators  

* silent coercions between boolean and anything else  

* silent coercions between enums and integers  

* silent arithmetic coercions, promotions  

* implementation-dependent sign for the result of % with negative dividend  

* bitwise operators with lower precedence than comparison operators  

* auto-increment operators  

* a poor-quality default hash function  

* pointer-heavy default containers  

Next time you're in a conversation about language design and someone sighs, shakes their head and tells you that sad legacy design choices are just the burden of the past and we're helpless to avoid repeating them, try to remember that this is not so.