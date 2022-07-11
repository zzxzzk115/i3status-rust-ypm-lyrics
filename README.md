# i3status-rust-ypm-lyrics

![bar](bar.png)

![GitHub](https://img.shields.io/github/license/zzxzzk115/i3status-rust-ypm-lyrics) ![Language](https://img.shields.io/badge/language-rust-yellow)

在i3status-rust中显示YesPlayMusic的歌词

# 效果截图
![showcase](showcase.png)

# 前置依赖
- YesPlayMusic >= 0.4
- i3status-rust

以Arch为例:

安装YesPlayMusic:
```bash
yay -S yesplaymusic
```

安装i3status-rust:
```bash
yay -S i3status-rust
```

# 编译并安装

目前还没有提供各个架构预编译好的可执行程序，故需要自行编译并安装。

编译前，需要准备好Rust环境。

准备好后，就可以开始编译并安装了：

```bash
# 克隆git仓库
git clone https://github.com/zzxzzk115/i3status-rust-ypm-lyrics.git
# 进入项目根目录
cd i3status-rust-ypm-lyrics/
# 使用cargo进行build，编译发布版本
cargo build --release
# 安装: 拷贝到用户的bin目录
sudo cp target/release/i3ypml /usr/bin/
```

# i3status-rust配置

```bash
sudo vim ~/.config/i3status-rust/config.toml
```

在你觉得合适的位置插入以下内容：

```toml
[[block]]
block = "music"
player = "yesplaymusic"
buttons = ["prev", "play", "next"]

[[block]]
block = "custom"
command = "/usr/bin/i3ypml"
interval = 0.2
```

# 开发进度

目前版本实现的是最基本的功能，还存在一些问题：

- 存在一定延迟
- 没有考虑鲁棒性
- 歌词字符串没有左右滚动效果
- 没有用缓存来优化
- 没有使用GitHub Actions来进行自动化测试与构建
- 没有发布包到各个平台，目前只能使用Rust Cargo来build release

# 许可证 License
MIT
