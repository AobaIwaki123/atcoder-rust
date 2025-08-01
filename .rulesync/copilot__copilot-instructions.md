---
root: false
targets:
  - copilot
description: GitHub Copilot instructions
globs:
  - '**/*'
---

以下に、Rust らしいコードの例とスタイル指針を示します。
Copilot によるコード補完時は以下を意識してください。

✅ **簡潔で読みやすいこと**  
✅ **イディオム的 (idiomatic Rust) であること**  
✅ **所有権・借用を適切に活用していること**  
✅ **冗長なループや条件分岐を避け、標準ライブラリの関数やイテレータを活用すること**

## 例 1

```rust
// 配列の要素間の差分が s 以下かどうかをチェックする簡潔な例
let is_ok = t[0] <= s && t.windows(2).all(|w| w[1] - w[0] <= s);
```

## 例 2

```rust
// ベクトルの要素をまとめて文字列に変換し、スペースで結合する例
let vec: Vec<_> = a.iter().map(|x| x.to_string()).collect();
println!("{}", vec.join(" "));
```


このようなスタイルを参考に、今後のコード補完やリファクタリング時の提案を行ってください。

## 基本文法

### 重複排除 & 自動並び替えを行う型

```rust
use std::collections::BTreeSet;
let mut set = BTreeSet::new();
````