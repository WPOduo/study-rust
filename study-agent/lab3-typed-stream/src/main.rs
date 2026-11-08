//! Lab3：结构化输出与流式（课程模块 D2-M5）
//!
//! 目标：
//! 1. `prompt_typed::<T>()`：让 LLM 输出直接变成 Rust 类型（编译期校验）
//! 2. `prompt(..).stream()`：文本/推理/工具调用的增量事件流
//!
//! 骨架基于 rig 0.46.0；若与 rustdoc 有出入，以 https://docs.rs/rig/latest/rig/ 为准。

use futures::StreamExt;
use rig::agent::AgentBuilder;
use rig::prelude::*;
use rig::providers::openai::{self, OpenAI};
use schemars::JsonSchema;
use serde::Deserialize;

/// 让模型输出的目标类型：字段名与类型就是给模型的 schema。
#[derive(Debug, Deserialize, JsonSchema)]
struct CodeReview {
    /// overall severity: low | medium | high
    severity: String,
    /// one issue per entry
    issues: Vec<String>,
    /// short summary
    summary: String,
}

const SAMPLE_CODE: &str = r#"
fn get_user(id: u32) -> Option<&str> {
    let users = vec!["alice", "bob"];
    if id > 0 && id < users.len() as u32 {
        return Some(users[id as usize]);
    }
    None
}
"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let model = OpenAI::from_env()?.completion(openai::GPT_5_2);

    let reviewer = AgentBuilder::new(model.clone())
        .name("reviewer")
        .preamble("You are a strict Rust code reviewer. Answer in English.")
        .build();

    // ---- Part 1: 结构化输出 ----
    let review = reviewer
        .prompt_typed::<CodeReview>(SAMPLE_CODE)
        .max_turns(3)
        .await?
        .output;

    println!("== typed output ==");
    println!("{}", serde_json::to_string_pretty(&review)?);

    // ---- Part 2: 流式 ----
    println!("\n== streaming ==");
    let mut stream = reviewer.prompt("Review this code again in one paragraph.").stream();
    while let Some(item) = stream.next().await {
        let item = item?; // TextDelta / ReasoningDelta / ToolCallDelta / FinalResponse
        print!("{item:?}");
    }
    println!();

    // ---- 课堂练习 ----
    // 1. 给 CodeReview 加一个 suggestions: Vec<String> 字段，看 schema 是否自动更新
    // 2. 换一个便宜的小模型跑 prompt_typed，观察 output_mode（native 可能不被支持）
    // 3. 流式部分改打 item 里的文本增量而不是 Debug，做打字机效果
    Ok(())
}
