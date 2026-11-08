# study-agent —— rig 两天 Agent 开发培训 Lab 仓库

对应课程：`98 开源项目/72 rig-rust-agent框架.md` 中的「课程设计」章节。

## 目录与课程映射

| 目录 | 课程模块 | 内容 |
|---|---|---|
| `lab1-hello-agent` | D1 M2 | 环境准备、跑通第一个 Agent |
| `lab2-tools` | D1 M4 | `#[rig_tool]` 自定义工具、多轮工具调用循环 |
| `lab3-typed-stream` | D2 M5 | `prompt_typed` 结构化输出、`prompt().stream()` 流式 |
| `lab4-rag` | D2 M7 | embedding 入库 + `dynamic_context` 文档问答 |
| `final-a-code-review` | Final Project A | 代码审查助手（含结构化输出与工具） |
| `final-b-kb-qa` | Final Project B | 团队知识库问答（RAG + 工具 + 引用来源） |

## 环境准备

```bash
# 1. Rust 1.95+，工具链就位
rustup update

# 2. 配置 API key（任选一家；教学统一 OpenAI）
# Windows PowerShell
$env:OPENAI_API_KEY="sk-..."
# Linux/macOS
export OPENAI_API_KEY="sk-..."
```

根目录是 Cargo workspace，共享一个 `target/` 与锁定版本，任一 lab 运行：

```bash
cargo run -p lab1-hello-agent
cargo run -p lab2-tools
# ...
```

## 注意

- 所有骨架代码基于 **rig 0.46.0**（pre-1.0）。若与 rustdoc 有出入，以
  <https://docs.rs/rig/latest/rig/> 和官方仓库 `examples/` 为准，并同步修正讲义。
- 练习顺序按课程走：Lab1 → Lab2 →（D2）Lab3 → Lab4 → Final。
- 密钥只放环境变量，不要写进代码或提交 git。
