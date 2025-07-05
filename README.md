## 新しいプロジェクトの作成方法

```sh
$ mkdir abc_999
$ cd abc_999
$ cargo init
```

- 以下を`Cargo.toml`に追加します。

```toml
# dependencies added to new project
[dependencies]
proconio = { version = "0.5.0", features = ["derive"] }

[profile.release]
lto = true
panic = 'abort'

[[bin]]
name = "a"
path = "src/a.rs"

[[bin]]
name = "b"
path = "src/b.rs"

[[bin]]
name = "c"
path = "src/c.rs"
```

## フォーマッター

```sh
$ cargo fmt
```

## クリッパー

```sh
$ cargo clippy -- -A clippy::all
```
