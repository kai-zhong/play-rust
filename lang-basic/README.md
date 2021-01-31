lang-basic是一个包（Package），用于学习Rust语言的基础知识。在开启这个学习项目之前，已初略阅读了The Rust Programming Language的前十章节，因此该项目从Rust的Package机制展开学习。


#### Packages和Crates
- 用cargo生成了`lang-basic`包(Package)，并详细理解binary crate和library crate的区别。
- Rust的一个Package可以包含多个binary crate，按照rust的约定，src/main.rs作为一个与lang-basic同名的binary create，src/bin/ 目录下每一个文件作为其它的binary crate。因此在src/bin目录下创建了一个叫做secondary_main.rs的文件并成功编译。