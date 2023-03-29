# Rust 简易论文查重

![](https://img.shields.io/github/languages/code-size/SMS-COSMO/Plagiarism-Detector-Rust?color=yellow&style=flat-square)

## 部署

```bash
cargo build
cargo run
```

端口: `8000`

## API

`POST /add`: 

**请求:**

```json
{
    "id": "1",
    "text": "基于 jieba 分词，tf-idf 算法求相似度   ！（*@1=-+!9）"
}
```

其中 id 应不与已有重复。

**响应:**

```json
{
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

表示前 5 个相似度最高的论文，每项第一个值表示相似度，第二个值为论文 id。