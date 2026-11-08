# Lab3：结构化输出与流式（D2-M5）

## 目标

- `prompt_typed::<T>()`：输出即类型，`T: JsonSchema + DeserializeOwned`
- `prompt(..).stream()`：统一事件流（文本增量、推理增量、工具调用参数增量、FinalResponse）

## 步骤

```bash
cargo run -p lab3-typed-stream
```

## 检查点

1. `prompt_typed` 的 schema 校验失败会发生什么？（重试；调用方必须处理失败）
2. 三种 `output_mode`（native / tool / prompted）分别适合什么模型？
3. 流式 item 里工具调用参数没收完时，会不会提前执行工具？（不会）

## 扩展练习

- 把流式打印改成打字机效果（只渲染文本增量）
- 结构化输出 + 工具组合：reviewer 先用 `read_file` 工具拿代码，再输出 `CodeReview`
