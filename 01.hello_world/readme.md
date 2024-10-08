要编写并运行一个简单的 Rust 程序，首先新建一个名为 `main.rs` 的源文件，文件名应使用下划线分隔单词。然后在文件中输入以下代码：

```rust
fn main() {
    println!("Hello, world!");
}
```

这个程序定义了一个 `main` 函数，这是所有 Rust 可执行程序的入口点。`println!` 是一个宏，用于将字符串 "Hello, world!" 打印到屏幕上。

保存文件后，在终端中运行以下命令编译并执行程序：

```sh
rustc main.rs
./main
```

在 Windows 上，执行 `.\main.exe`。成功运行后，终端将输出 "Hello, world!"。

Rust 程序的编译和运行是两个独立的步骤，先使用 `rustc` 编译代码，生成一个二进制可执行文件，再运行该文件。Rust 是一种预编译静态类型语言，这意味着你可以将生成的可执行文件分发给其他人，他们无需安装 Rust 即可运行该程序。随着项目复杂度增加，可以使用 Cargo 管理构建过程和依赖。
