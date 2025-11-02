# AI 生成です。

# nanai_simple_lang

## 目的

簡潔な言語である
基本的にRustと全く同じ構文。
Rustの難しい要素であるライフタイムをなくす。

## nomパーサーの試し方

### 1. nomパーサーを有効にしてビルド

```sh
cargo build --features nom
```

### 2. nomパーサーのテストを実行（例: let文）

`src/parser/let_stmt.rs` の `nom_let_parser::parse_let` を使ったテスト例:

```rust
#[cfg(test)]
mod tests {
    use super::nom_let_parser::parse_let;
    #[test]
    fn test_nom_let() {
        let input = "let mut x: i32 = y";
        let res = parse_let(input);
        assert!(res.is_ok());
        let (_, (mutable, name, ty, value)) = res.unwrap();
        assert!(mutable);
        assert_eq!(name, "x");
        assert_eq!(ty, Some("i32".to_string()));
        assert_eq!(value, "y");
    }
}
```

### 3. 通常の実行

```sh
cargo run --bin nasl main.nasl
```

---

- `--features nom` でnomパーサーが有効化されます。
- nomパーサーのテストは `cargo test --features nom` で実行できます。
- 通常のREPL/ファイル実行は `cargo run` でOK。
