### Cargo

Cargo 是 Rust 的构建系统和包管理器，能够管理项目的构建、依赖和运行。通过 `cargo new` 命令创建项目后，Cargo 会生成项目目录、`Cargo.toml` 文件和 `src` 文件夹。`Cargo.toml` 文件定义了项目的元信息和依赖项。

使用 Cargo 可以通过 `cargo build` 来编译项目，生成的可执行文件存放在 `target/debug` 目录下。`cargo run` 可以一步完成编译和运行，而 `cargo check` 用于快速检查代码是否能编译，但不会生成可执行文件。最终发布项目时，可以使用 `cargo build --release` 进行优化编译。

随着项目的复杂性增加，Cargo 的优势会更加明显，因此养成使用 Cargo 的习惯非常重要。
