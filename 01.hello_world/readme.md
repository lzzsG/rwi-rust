01.hello_world

1. **编写并运行 Rust 程序**：
   - 新建一个名为 `main.rs` 的文件，文件名使用下划线分隔单词。
   - 在文件中输入以下代码，定义 `main` 函数并使用 `println!` 宏打印 "Hello, world!"：

     ```rust
     fn main() {
         println!("Hello, world!");
     }
     ```

   - 保存文件后，在终端中运行以下命令编译和执行程序：

     ```sh
     rustc main.rs
     ./main  # 在 Windows 上使用 .\main.exe
     ```

2. **分析 `main.rs` 文件**：
   - `main` 函数是 Rust 程序的入口，必须用 `{}` 包裹函数体，且符合 Rust 的代码风格。
   - `println!` 是一个宏，用于打印字符串到屏幕上，行尾用分号结束。

3. **编译与运行的步骤**：
   - Rust 是一种预编译语言，需要先使用 `rustc` 编译源代码，然后再运行生成的可执行文件。
   - 编译命令：`rustc main.rs`，生成二进制文件后在终端中运行 `./main` 或 `.\main.exe`。

4. **Rustfmt 自动格式化工具**：
   - 使用 `rustfmt` 工具可以自动格式化代码，确保代码风格一致。Rust 标准发行版中已包含 `rustfmt`。
   - 终端中输入以下命令来格式化整个项目的代码：`cargo fmt`这将会格式化项目中的所有 Rust 文件。
