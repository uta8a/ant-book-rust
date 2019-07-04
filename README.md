# ant-book-rust

```
tree .
---
│   │   ├── main.rs
│   │   └── test
│   │       ├── sample-1.in
│   │       └── sample-1.out
│   └── p3
│       ├── main
│       ├── main.rs
│       └── test
│           ├── sample-1.in
│           ├── sample-1.out
│           ├── sample-2.in
│           └── sample-2.out
---
c1-1: chapter 1-1
p1: problem-1
```

- kimiyukiさんのonline-judge-toolsを使ってサンプルの確認を行っています

```
oj test -c ./main
```

- ref: プログラミングコンテストチャレンジブック

# todo

- fibonacciのベンチマーク [参考](https://muunyblue.github.io/115c51eb37365df2d4f4e2482b964822.html#_6)
- fibonacci_memoはvig直下に定数をおいているので、impl N(Number Theory)以下に置きたい。`vig::N::SIZE`のようにしたい。
- stack, queueなどの使い分け[公式](https://doc.rust-lang.org/std/collections/index.html#when-should-you-use-which-collection)
- if let, while letの使い方まとめる

# Misc

- ライブラリは `vig::{S, N, G, ...}::function`という感じにしたい。それぞれのimplはSearch、Number Theory、Graphなどを表す。

# Note

- `stack` Vec push(`vec.push()`) pop(`vec.pop()`)
- `queue` VecDeque push(`push_back`) pop(`pop_front`)

