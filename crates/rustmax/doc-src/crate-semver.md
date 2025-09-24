Semantic version parsing and comparison.

- Crate [`::semver`].
- [docs.rs](https://docs.rs/semver)
- [crates.io](https://crates.io/crates/semver)
- [GitHub](https://github.com/dtolnay/semver)

---

`semver` provides parsing and manipulation of semantic version numbers according
to the [semver specification][semver-spec]. Crate authors following
[Rust semver guidelines][semver-guide] keep each others' builds from breaking.


## Examples

Parsing and comparing versions:

```
use semver::Version;

let v1 = Version::parse("1.2.3").unwrap();
let v2 = Version::parse("1.2.4").unwrap();

assert_eq!(v1.major, 1);
assert_eq!(v1.minor, 2);
assert_eq!(v1.patch, 3);

assert!(v1 < v2);
```

Working with prerelease versions:

```
use semver::Version;

let stable = Version::parse("1.0.0").unwrap();
let prerelease = Version::parse("1.0.0-alpha.1").unwrap();

assert!(prerelease < stable);
assert_eq!(prerelease.pre.as_str(), "alpha.1");
```

Version requirements and compatibility:

```
use semver::{Version, VersionReq};

let req = VersionReq::parse(">=1.2.0, <2.0.0").unwrap();
let version = Version::parse("1.5.0").unwrap();

assert!(req.matches(&version));
```

[`Version`]: crate::semver::Version
[`VersionReq`]: crate::semver::VersionReq
[`parse`]: crate::semver::Version::parse
[semver-spec]: https://semver.org/
[semver-guide]: https://doc.rust-lang.org/cargo/reference/semver.html
