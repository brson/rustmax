A collection of useful Rust crates.

This crate documents and reexports selected high-quality Rust crates
suitable for typical Rust programs.

- [Using `rustx` for crate discovery]
- [Using `rustx` as a library]
- [Feature profiles]
  - [Profile: `rx-profile-no-std`]
  - [Profile: `rx-profile-std`]
  - [Profile: `rx-profile-net`]
  - [Profile: `rx-profile-cli`]
  - [Profile: `rx-profile-build-script`]
  - [Profile: `rx-profile-proc-macro`]
  - [Profile: `rx-profile-full`]


# Using `rustx` for crate discovery.

If you just need to know decent crates for common programming tasks,
read this documentation to find them.


# Using `rustx` as a library.

todo

In your manifest `Cargo.toml`:

```toml
[dependencies]
rustx = "0.1.0"
```

Or if using a workspace, in your workspace `Cargo.toml`

```toml
[workspace.dependencies]
rustx = "0.1.0"
```

And in your manifest `Cargo.toml`

```toml
[dependencies]
rustx.workspace = true
```


# Feature profiles

The main way of configuring the `rustx` crates is by enabling
the appropriate _profile_ cargo features.

The default profile feature is `rx-profile-std`.
This feature augments the Rust `std` library with crates
that are widely used with a variety of Rust programs,
as well as minor helpers missing from the standard library.

If the default features are disabled:

```toml
[dependencies]
rustx.version = "0.1.0"
rustx.default-features = false
```

Then `rustx` reexports no crates.
Profiles can then be added by adding cargo features:

```toml
[dependencies]
rustx.version = "0.1.0"
rustx.default-features = false
rustx.features = ["rx-profile-no-std"]
```


## Profile: `rx-profile-no-std`

This profile includes crates that do not require Rust `std`,
and provide features used by many Rust programs.

Crates in this profile:

- [`anyhow`]
- [`backtrace`]
- [`base64`]
- [


## Profile: `rx-profile-std`


## Profile: `rx-profile-net`


## Profile: `rx-profile-cli`


## Profile: `rx-profile-build-script`


## Profile: `rx-profile-proc-macro`


## Profile: `rx-profile-full`


# Features

todo

- `default`
  - `all-crates`
  - `std`
- `std`
  - `anyhow/std`
- `backtrace`
  - `std`
  - `anyhow/backtrace`
- `all-crates`


# TODO

- update big_s and og_fmt to be no_std
- update fmt to `use format as fmt`
