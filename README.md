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
- fibonacci_memoはvig直下に定数をおいているので、impl N(Number Theory)以下に置きたい。

# Misc

- ライブラリは `vig::{S, N, G, ...}::function`という感じにしたい。それぞれのimplはSearch、Number Theory、Graphなどを表す。