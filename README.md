# Cargo space manager
能简单操作cargo space的clt.

## 支持的命令
- `init`: 生成带`[workspace]`的`Cargo.toml`，生成`.gitignore`，生成`README.md`.
- `add`: 执行带`--vcs=no`的`cargo new`命令，同时在中`[workspace]`中注册。
- `remove`: 在文件系统和`[workspace]`中删除项目。

## 开发目标：
- 明确开发时执行的`pwd`.
- 合理且全面的`test`，包括临时文件夹的建立。
- 合理的人机交互信息与`error`传递。
- *设计优先*，先设计再实现。

## 设计流程
- 设计核心struct和action的抽象.
- 设计并实现test，`cargo test -- --nocapture`。
- 设计 交互信息。
- 开发实际功能。
- 总结出可能出现的error。