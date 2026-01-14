Splitting strings on grapheme cluster, word, and sentence boundaries.

- Crate [`::unicode_segmentation`].
- [docs.rs](https://docs.rs/unicode-segmentation)
- [crates.io](https://crates.io/crates/unicode-segmentation)
- [GitHub](https://github.com/unicode-rs/unicode-segmentation)

---

`unicode-segmentation` provides iterators for splitting text according to [Unicode Standard Annex #29] rules.
It handles the complexities of measuring Unicode text for display,
where a single user-perceived character may be composed of multiple codepoints.

The crate's primary trait is [`UnicodeSegmentation`],
which provides methods for segmenting strings by grapheme clusters, words, and sentences.
Grapheme clusters represent what users think of as single characters,
which is essential for correctly counting characters,
truncating strings, or implementing text editors.

Note that while Unicode segmentation is a crucial algorithm,
it is rarely the right tool for most software &mdash;
it is mostly used by GUI toolkits for laying out text,
or by software that needs to understand the human concepts of
"words" and "sentences".

For modern background on Unicode units see [Let's Stop Ascribing Meaning to Code Points]
by Manish Goregaokar.

## Examples

Count user-perceived characters correctly:

```
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text = "Hello 👨‍👩‍👧‍👦!";

    // Wrong: counting bytes
    assert_eq!(text.len(), 32);

    // Wrong: counting codepoints
    assert_eq!(text.chars().count(), 14);

    // Correct: counting grapheme clusters
    assert_eq!(text.graphemes(true).count(), 8);
}
```

Split text into words:

```
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text = "Hello, world! How are you?";

    let words: Vec<&str> = text.unicode_words().collect();
    assert_eq!(words, vec!["Hello", "world", "How", "are", "you"]);
}
```

[Unicode Standard Annex #29]: https://www.unicode.org/reports/tr29/
[Let's Stop Ascribing Meaning to Code Points]: https://manishearth.github.io/blog/2017/01/14/stop-ascribing-meaning-to-unicode-code-points/
[`UnicodeSegmentation`]: https://docs.rs/unicode-segmentation/latest/unicode_segmentation/trait.UnicodeSegmentation.html
