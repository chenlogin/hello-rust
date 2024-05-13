
### 安装
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### 查看版本
```
rustc —version
```
### 项目构建
```
cargo —version
cargo new hello-rust 创建hello-rust项目
cargo build 构建项目
cargo run 编译并运行项目
cargo test 测试项目
cargo check 检查代码是否可以编译通过，不实际生成可执行文件
cargo doc --open 本地文档
cargo publish 将库发布到 crates.io
```

- 识别答题卡第一道选择题的填涂状态
```
打开图片 => 找到指定区域 => 彩图转为8位灰度图 => 灰度图高斯模糊 => 复制、二值化处理(确认阈值) => 判断非零占比 => 保存处理后图片
```

- 执行cargo build后，通常会在项目根目录下的target文件夹中生成一系列文件。
  - 可执行文件：如果项目是一个可执行程序，cargo build会在target/debug 目录下生成一个可执行文件。这个文件的名字通常是项目的名称。
  - 依赖库：如果项目依赖于其他外部库，这些库会被下载并编译到target/debug/deps 目录中。这些库文件通常以.rlib或.dylib（在macOS和Linux上）或.dll（在Windows上）为后缀。
  - 增量编译文件：Cargo使用增量编译来提高后续构建的速度。这些增量编译的文件存储在target/debug/incremental 目录中。
  - 其他中间文件：构建过程中可能会生成其他中间文件，如对象文件（.o或.obj文件）、依赖文件等，这些通常也存储在target/debug 目录下的各个子目录中。
  - 构建脚本输出：如果项目包含构建脚本（build.rs），则该脚本的输出可能也会存储在target目录下的某个位置，具体位置取决于脚本的配置。
  - 执行cargo build --release 那么生成的文件将针对性能进行优化，并且通常会位于target/release目录下，而不是target/debug目录

- Rust编译成wasm文件
  - wasm-pack
  - WebAssembly（WASM）是一个简单的机器模型和可执行格式， 由两种格式组成
    * 后缀为 .wat 的文本格式（称为“WebAssembly Text”），可以被人类理解，使用 S-表达式。
    * 后缀为 .wasm 的二进制格式是较低级别的，人无法读懂，它旨在供 wasm 虚拟机直接使用。
- Rust编译成.node文件
  - napi-rs


