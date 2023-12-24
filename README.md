# Rust 简易论文查重

## 部署

修改配置见 `Rocket.toml`。

1. 直接下载

在 release 中，找到 `linux.zip`。解压，在文件夹内：
```sh
./plagiarism-detector-rust
```

2. 手动编译

**生产：**

```sh
cargo run --release
```

**测试：**

```sh
cargo run
```

## API

### 请求：

- `POST /`

| 名称    | 类型   | 内容                                                                               |
| :------ | :----- | :--------------------------------------------------------------------------------- |
| `id`    | String | 论文 id，不允许重复                                                                |
| `text`  | String | 文本内容                                                                           |
| `write` | bool   | `false`：检查论文相似度 <br> `true`：检查之后将论文加入数据库 **!!暂不支持修改!!** |


```json
{
    "id": "1",
    "text": "基于 jieba 分词，tf-idf 算法求相似度   ！（*@1=-+!9）",
    "write": true
}
```

### 响应：

| 名称         | 类型   | 内容                                                                   |
| :----------- | :----- | :--------------------------------------------------------------------- |
| `msg`        | String | 返回信息                                                               |
| `similarity` | []     | 表示前 5 个相似度最高的论文，每项第一个值表示相似度，第二个值为论文 id |


```json
{
    "msg": "加入成功",
    "similarity": [
        [
            0.97821377847197237,
            "3"
        ],
        [
            0.23219838816700674,
            "1"
        ],
        [
            0.09556481196700674,
            "5"
        ],
        [
            0.0009258006340497831,
            "9"
        ],
        [
            0,
            "4"
        ]
    ]
}
```

## 技术栈

- [Rocket](https://rocket.rs/) - A web framework for Rust that makes it simple to write fast, type-safe, secure web applications with incredible usability, productivity and performance
- [SeaORM](https://www.sea-ql.org/SeaORM/) - 🐚 SeaORM is a relational ORM to help you build web services in Rust
- [jieba-rs](https://github.com/messense/jieba-rs) - The Jieba Chinese Word Segmentation Implemented in Rust

