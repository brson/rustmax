# Rust CLI One Liners

## Find all Rust files

```
fd -e rs
```

## Replace text in all Rust files

```
fd -e rs -x sd "before_regex" "after"
```

## Delete trailing newlines in all Python and Rust files

```
fd -e py -e rs . src/testing/python_driver/ -x sd "\s+$" "\n"
```
