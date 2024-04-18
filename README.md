
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
cargo doc --open 本地文档
cargo publish 将库发布到 crates.io
```

- 识别答题卡第一道选择题的填涂状态
```
打开图片 => 找到指定区域 => 彩图转为8位灰度图 => 灰度图高斯模糊 => 复制、二值化处理(确认阈值) => 判断非零占比 => 保存处理后图片
```