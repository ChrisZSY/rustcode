# rustcode
github提交代码忽略文件规则：https://github.com/github/gitignore.git

rust & cargo 修改库源：

rustup换源（中科大源）：
在.bashrc中写入:    
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

cargo换源：
新建（或编辑已有）文件 cat $HOME/.cargo/config，写入：
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index/"
如果cargo版本小于0.13.0，在开头添加：
[registry]
index = "https://mirrors.ustc.edu.cn/crates.io-index/"

config文件格式说明:
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
指定镜像
replace-with = '镜像源名' # 如：tuna、sjtu、ustc，或者 rustcc

注：以下源配置一个即可，无需全部
中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"
