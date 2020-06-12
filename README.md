[![CircleCI](https://circleci.com/gh/mmmpa/skip_do.svg?style=svg)](https://circleci.com/gh/mmmpa/treeer)

# SkipDo

An iterator that skip items and use skipped items

```rust
use skip_do::SkipDo;
let mut v = vec![1, 2, 3, 4];
let mut v2 = vec![];
v = v
    .into_iter()
    .skip_do(|x| x <= &2, |x| v2.push(x))
    .map(|x| x)
    .collect();
assert_eq!(v, [3, 4]);
assert_eq!(v2, [1, 2]);
```
