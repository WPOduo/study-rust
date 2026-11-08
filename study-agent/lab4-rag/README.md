# Lab4：RAG 文档问答（D2-M7）

## 目标

- 理解 embedding：文本 → 向量，语义近则距离近；检索 = 拿问题向量找最近文档块
- 掌握 `dynamic_context(n, index)`：每轮 prompt 自动注入 top-n，**模型自己不知道检索这回事**

## 步骤

```bash
cargo run -p lab4-rag
```

`./docs` 下放了两个示例 md；可自行增删文档再问。

## 检查点

1. RAG 和「把全部文档塞进 preamble」的区别？（成本、上下文上限、噪声）
2. 检索发生在哪一侧？（应用侧，自动完成；对模型只是普通上下文）
3. `dynamic_context(3, ..)` 的 3 调大有什么代价？

## 扩展练习

- 让回答带上引用来源（提示：Doc 的 `path` 元数据会随上下文一起给到模型）
- 换用 Qdrant / LanceDB（开对应 feature），对比代码改动量
- 大库优化：`InMemoryVectorStore::builder()` 切 `IndexStrategy::LSH`

## 已知注意

入库/embedding 的具体签名可能随小版本调整（见 main.rs 顶部注释）；
编译报错时先看 rustdoc，不要改动 `dynamic_context` 之外的 Agent 部分。
