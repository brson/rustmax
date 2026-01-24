# Topic Index Format Brainstorming

A mechanical format for the topic index, separate from concrete source mappings.


## Goals

1. **Machine-readable** - parseable, validatable, usable by search engine
2. **Symbolic** - topics link to topics, not to URLs or files
3. **Queryable** - support alias expansion, relationship traversal, faceting
4. **Maintainable** - easy to add/edit topics, catch errors


## Core data model

### Topic node

```
Topic {
    id: String,           // canonical identifier, kebab-case
    name: String,         // display name
    aliases: [String],    // alternate search terms
    category: CategoryId, // primary category
    tags: [String],       // secondary categorization
    brief: String,        // one-line description for search snippets
    relations: [Relation],
}
```

### Relation types

```
Relation {
    kind: RelationKind,
    target: TopicId,
}

RelationKind:
    - related      // loose association, "see also"
    - parent       // hierarchical: this topic is a subtopic of target
    - implements   // this crate/tool implements that concept
    - requires     // understanding this requires understanding target
    - supersedes   // this replaces/deprecates target
    - contrast     // often confused with, but different from
```

### Categories

Top-level facets for filtering:

```
Category {
    id: String,
    name: String,
    description: String,
}
```

Proposed categories:
- `lang` - language features
- `std` - standard library
- `crate` - ecosystem crates
- `tool` - toolchain and dev tools
- `domain` - programming task domains
- `pattern` - idioms and patterns
- `book` - documentation resources
- `error` - compiler errors/diagnostics
- `culture` - community, events, history
- `platform` - OS and target platforms


## Example topic entries

### Language feature

```toml
[topics.ownership]
name = "Ownership"
aliases = ["move semantics", "owned values", "ownership system"]
category = "lang"
tags = ["memory-safety", "core-concept"]
brief = "Rust's system for managing memory through ownership rules"

[[topics.ownership.relations]]
kind = "related"
target = "borrowing"

[[topics.ownership.relations]]
kind = "related"
target = "lifetimes"

[[topics.ownership.relations]]
kind = "contrast"
target = "garbage-collection"
```

### Crate

```toml
[topics.serde]
name = "serde"
aliases = ["serialization framework", "serialize deserialize"]
category = "crate"
tags = ["serialization", "derive"]
brief = "Framework for serializing and deserializing Rust data structures"

[[topics.serde.relations]]
kind = "implements"
target = "serialization"

[[topics.serde.relations]]
kind = "related"
target = "serde-json"

[[topics.serde.relations]]
kind = "related"
target = "toml"
```

### Domain concept

```toml
[topics.serialization]
name = "Serialization"
aliases = ["serialize", "deserialize", "marshalling", "encoding data"]
category = "domain"
tags = ["data", "io"]
brief = "Converting data structures to/from bytes or text formats"

[[topics.serialization.relations]]
kind = "parent"
target = "encoding"
```

### Error message

```toml
[topics.e0382]
name = "E0382: borrow of moved value"
aliases = ["moved value", "use after move", "value moved here"]
category = "error"
tags = ["ownership", "borrow-checker"]
brief = "Attempting to use a value after it has been moved"

[[topics.e0382.relations]]
kind = "requires"
target = "ownership"

[[topics.e0382.relations]]
kind = "requires"
target = "move-semantics"
```


## Format options

### Option A: TOML

Pros:
- Human-readable/writable
- Good tooling
- Comments allowed
- Already used in Rust ecosystem

Cons:
- Verbose for deeply nested data
- No schema validation built-in

### Option B: JSON

Pros:
- Universal tooling
- JSON Schema for validation
- Easy to generate/consume

Cons:
- No comments
- Verbose, harder to hand-edit

### Option C: Custom DSL

```
topic ownership {
    name: "Ownership"
    aliases: "move semantics", "owned values"
    category: lang
    tags: memory-safety, core-concept
    brief: "Rust's system for managing memory through ownership rules"

    related: borrowing, lifetimes
    contrast: garbage-collection
}

topic serde {
    name: "serde"
    category: crate
    implements: serialization
    related: serde-json, toml
}
```

Pros:
- Compact, readable
- Tailored to our needs

Cons:
- Need to write parser
- No existing tooling


### Option D: Rust code

```rust
pub static TOPICS: &[Topic] = &[
    Topic {
        id: "ownership",
        name: "Ownership",
        aliases: &["move semantics", "owned values"],
        category: Category::Lang,
        tags: &["memory-safety", "core-concept"],
        brief: "Rust's system for managing memory...",
        relations: &[
            Relation::related("borrowing"),
            Relation::related("lifetimes"),
            Relation::contrast("garbage-collection"),
        ],
    },
    // ...
];
```

Pros:
- Type-checked at compile time
- No parsing needed
- IDE support
- Can include Rust logic (computed aliases, etc.)

Cons:
- Recompile to update
- Harder for non-Rust tooling to consume


## Recommendation: TOML + Rust types

Use TOML files for the source of truth, with Rust types for deserialization.

```
topics/
  lang.toml        # language features
  std.toml         # standard library
  crates.toml      # ecosystem crates
  tools.toml       # tooling
  domains.toml     # programming domains
  patterns.toml    # idioms
  errors.toml      # compiler errors
  culture.toml     # community topics
  platforms.toml   # platform-specific
```

Rust code loads and validates at build time or runtime.


## Search engine usage

### Query expansion

User searches "hashmap" ->
1. Find topic `hashmap` (exact match on id)
2. Also find topics where "hashmap" appears in aliases
3. Return matched topics + their relations

### Faceted search

User filters by category "crate" ->
Return all topics where `category = "crate"`

### Related topics

User views topic "async-await" ->
Show sidebar with topics where:
- `async-await` has `related` relation to them
- They have `related` relation to `async-await`
- They share tags with `async-await`

### "Did you mean?"

User searches "hasmap" (typo) ->
Fuzzy match against all topic ids + aliases
Suggest "hashmap"

### Concept -> crate mapping

User searches for concept "serialization" ->
Find topics with `implements: serialization` relation
Return: serde, serde-json, toml, bincode, etc.


## Relation semantics

### `related`
Bidirectional loose association. If A is related to B, B is related to A.
Used for: general "see also" connections.

### `parent`
Hierarchical. A has parent B means A is a subtopic of B.
Inverse: B contains A.
Used for: topic hierarchy, drill-down navigation.

### `implements`
Directional. Crate/tool A implements concept B.
Used for: "what crate do I use for X?" queries.

### `requires`
Directional. Understanding A requires understanding B.
Used for: learning paths, prerequisite suggestions.

### `supersedes`
Directional. A replaces/deprecates B.
Used for: redirecting searches, "use X instead" suggestions.

### `contrast`
Bidirectional. A and B are often confused but different.
Used for: disambiguation, "X vs Y" content.


## Alias design

Aliases should capture:

1. **Alternate spellings**: "HashMap" / "hashmap" / "hash map" / "hash-map"
2. **Synonyms**: "mutex" / "mutual exclusion" / "lock"
3. **Common misspellings**: (maybe, could get noisy)
4. **Question forms**: "how to serialize" -> serialization
5. **Crate name variations**: "serde_json" / "serde-json" / "serde json"

Alias matching should be case-insensitive.


## Tag design

Tags provide secondary categorization orthogonal to primary category.

Examples:
- `memory-safety` - topics related to Rust's memory safety guarantees
- `async` - topics related to async programming
- `derive` - topics involving derive macros
- `no-std` - topics relevant to no_std environments
- `nightly` - topics requiring nightly Rust
- `deprecated` - outdated topics kept for historical reference

Tags enable cross-cutting queries like "show me all async-related crates."


## Validation rules

The index should be validated for:

1. **Referential integrity**: All relation targets must exist as topic ids
2. **Category validity**: Category must be a defined category id
3. **No self-references**: A topic cannot relate to itself
4. **Alias uniqueness**: Warn if same alias maps to multiple topics (disambiguation needed)
5. **Required fields**: id, name, category, brief are required
6. **Id format**: lowercase kebab-case, alphanumeric + hyphens


## Open questions

1. **Alias weighting**: Should some aliases rank higher than others? ("serde" exact match > "serialization framework" phrase match)

2. **Relation strength**: Should relations have weights? (strongly-related vs loosely-related)

3. **Versioning**: How to handle topics that only apply to certain Rust versions? Tag? Separate field?

4. **Localization**: Will topic names/aliases need translation? Probably not for v1.

5. **Generated vs curated**: Some topics could be auto-generated from crate metadata. How to merge with curated content?
