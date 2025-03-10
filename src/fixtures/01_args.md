---
args:
  - FUNCTION
read:
  - "src/str.rs"
  - "src/args.rs"
edit:
  - "src/main.rs"
---

# Add unit tests for {{ FUNCTION }}

## Step 1 - think about what should be tested

Read `{{ FUNCTION }}` and think about how a Senior Rust Software Engineer would want to test it.

## Step 2 - add placeholder tests

Add placeholders for each of those unit tests using `todo!()`

Example:

```rs
#[test]
fn test_{{ FUNCTION }}_does_X() {
    todo!()
}
```

## Step 3 - implement tests

Now implement those unit tests
