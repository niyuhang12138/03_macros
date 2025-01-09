# Geektime Rust 语言训练营

## 第三周: 透视之眼: 深入浅出元编程

## 常用的宏编程包

```toml
[package]
name = "macros"
version = "0.1.0"
edition = "2021"
license = "MIT"

[lib]
proc-macro = true

[dev-dependencies]
anyhow = "1.0.95"
futures = { version = "0.3.31", default-features = false }
tokio = { version = "1.43.0", features = ["rt", "rt-multi-thread", "macros"] }

[dependencies]
darling = "0.20.10"
proc-macro2 = "1.0.92"
quote = "1.0.38"
syn = { version = "2.0.95", features = ["extra-traits"] }
```

![image-20250109091949850](assets/image-20250109091949850.png)
![image-20250109092014665](assets/image-20250109092014665.png)

![image-20250109101915163](assets/image-20250109101915163.png)

```bash
$ cargo install cargo-expand
$ cargo expand --example enum_macro
```

![image-20250109145051466](assets/image-20250109145051466.png)

> 方便操作创建宏的工具: darling包

![image-20250109155800244](assets/image-20250109155800244.png)
