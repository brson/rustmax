## Use `rustfmt::skip`

Sometimes formatting is important.
Don't be afraid annotate with `#[rustfmt::skip]`.

## Use `Option` with `?`

The `?` operator works with `Option` to early-return on `None`,
making option-heavy code much cleaner.

```rust
fn find_user_email(id: u32) -> Option<String> {
    let user = database.find_user(id)?;  // Return None if user not found
    let profile = user.get_profile()?;   // Return None if no profile
    let email = profile.email?;          // Return None if no email
    Some(email)
}
```

An alternative to `match` or `if let` statements, `?` lets you chain
operations that might fail, automatically propagating `None` up the call stack.
