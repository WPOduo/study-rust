# Lab1：Hello Agent（D1-M2）

## 目标

- 理解 Agent 三要素：**模型（能力上限）+ preamble（行为边界）+ prompt（本次任务）**
- 理解 rig 的 provider 中立：换厂商只换 client

## 步骤

```bash
cargo run -p lab1-hello-agent
```

## 检查点（口头验收）

1. 模型能自己执行工具吗？（不能——M1 的结论，下一课验证）
2. `preamble` 和用户消息有什么区别？
3. `temperature=0` 能保证输出完全一样吗？

## 扩展练习

- 把 `openai::OpenAI` 换成 Anthropic client，统计业务代码改动行数
- 给 agent 加 `.name("comedian")` 与 `.description(..)`，观察是否影响行为
- （思考）为什么 rig 没有做 workflow DSL，而是用普通 Rust 组合？
