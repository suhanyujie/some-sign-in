# some-sign-in
glados 每日自动签到

## run
* 程序可以在 release 栏下载（目前只有 windows，其他系统需要自己编译）
* 新建一个配置文件放在程序同级目录下，配置文件名**必须**为  `env.local.toml`，配置文件内容[参考这个](./env.dev.toml)
* 将你的 cookie 信息放入这个配置文件中，运行程序即可。
    * 如果你不知道如何获取 cookie 值，可以[查看教程](https://blog.csdn.net/u011781521/article/details/87791125)
    * 将 cookie 值替换 `xxx`，如下图所示：
    * ![](./doc/images/cookie-replace-show1.png)

## build
* `cargo build --release`

## ref
* toml 序列化 https://www.perfcode.com/rust-serde/serde-toml.html
* toml 规范 https://toml.io/cn/v1.0.0
* Making HTTP requests in Rust with Reqwest https://blog.logrocket.com/making-http-requests-rust-reqwest/
* 可选的 cron schedule
    * https://github.com/BlackDex/job_scheduler
    * https://github.com/mvniekerk/tokio-cron-scheduler
    * 轻量的 cron https://github.com/kurtbuilds/tokio-cron

