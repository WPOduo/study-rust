# Lab2：工具调用（D1-M4）

## 目标

- 会用 `#[rig_tool]` 写生产级工具（80% 的 Agent 日常工作量）
- 亲眼看到 M1 讲的工具调用循环：模型只「发调用请求」，执行在你的代码里

## 步骤

```bash
cargo run -p lab2-tools
```

观察终端：模型应先调 `list_md_files`，再逐个调 `word_count`，最后汇总。

## 检查点

1. 模型给了不存在的路径，你的工具该返回什么？（带原因的错误，让模型能重试）
2. `ToolContext` 会出现在给模型的参数 schema 里吗？（不会）
3. 为什么 `description` 里要写「Use it rather than …」这类选用提示？

## 扩展练习

- 新增工具 `char_count(path)`，对比模型在两个相似工具间的选择
- 故意把 `word_count` 的 description 写含糊，观察误调用率变化
- 用 `agent.prompt(task).stream()` 在终端实时看每一步工具调用（预习 Lab3）
