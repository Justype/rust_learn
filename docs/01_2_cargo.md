# Cargo

- Cargo 是 Rust 的构建系统和包管理工具
    - 构建代码、下载依赖库、构建这些库…
- 安装 Rust 时，会安装 Cargo
    - 使用 `cargo --version` 查看安装的版本

## 创建项目

- `cargo new hello_cargo`
    - 项目名称为`hello_cargo`
    - 会创建一个新目录，与项目名相同
        - 目录顶层有 `Cargo.toml`
        - `src` 文件夹放源代码

## 解析

### Cargo.toml

- TOML:(Tom's Obvious, Minimal Language) 格式
    - 是Cargo的配置格式

```toml
# [] 都会开启一个区域

[package] # 用来配置包的
name = "hello_cargo"    # 项目名
version = "0.1.0"       # 项目版本
authors = ["justype <3424005241@qq.com>"]   # 作者
edition = "2018"        # Rust 版本

[dependencies]  # 列出项目的依赖项
# Rust 里，包被称为 crate (大木箱，板条箱(运货用))
```

### src/main.rs

- 源代码应放到 `src` 目录下
- 顶层目录可放置 README、license、配置文件 等 与源代码无关的文件

## 构建项目 cargo build

- `cargo build`
    - 创建可执行文件
- 第一次运行，会在顶层目录生成 `cargo.lock` 文件
    - 该文件负责追踪项目依赖的精确版本
    - 不需要手动修改

## cargo run

编译 + 运行

注意
- 如果之前编译成功过，且源代码没有改变，就会直接执行

## cargo check

- 检查代码，确保能够通过编译，但不会产生任何可执行文件
- `cargo check` 比 `cargo build` 快的多
    - 编写代码时，可反复使用 `cargo check` 检查代码，提高效率

## 为发布而构建

- cargo build --release
    - 编译时会进行优化
        - 运行更快，编译更慢
    - 会在 `target/release` 而不是 `target/debug` 生成文件