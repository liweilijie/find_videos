# find video

在硬盘里面由于文件太多了，有视频有文档有代码，有时候很难找。

而且还有多块硬盘和家里的**NAS**以及库房的**NAS**系统里面也有。于是想做一个检索的工具试试。

- 利用**sqlite3**做数据库即可.
- **scan** 扫描整个盘的数据.
- **find** 查询包含有的关键字.
- 所有扫描的数据都保存在`~/.sofaraway.db` 下面.


```bash
# install sqlx sqlx-cli
cargo install sqlx-cli
# 生成init 目录 r: revers
sqlx migrate add init -r

sqlx migrate run --database-url="sqlite://sofaraway.sqlite"
sqlx migrate revert --database-url="sqlite://sofaraway.sqlite"

# 增加一个新的 sql migrations 文件一定要用这个命令，要不然就会报下面这个错。
sqlx migrate add events -r
```


`error: while executing migrations: error returned from database: (code: 1555) UNIQUE constraint failed: _sqlx_migrations.version`

这个报错不知道如何修复。 最后找到原因了一定要用 `sqlx migrate add events -r` 来新增加，这样时间就不会重复。

## sqlite3 相关

 创建一个空数据库： `sqlite3 sofaraway.sqlite "VACUUM;"`
 
## rust

```rust
#[allow(dead_code)]
pub fn new_delete(full_path: &str) -> Event {}
```

利用 `async channel` 来 实现效率提升。
```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}
```