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
```