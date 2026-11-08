//! Lab1：跑通第一个 Agent（课程模块 D1-M2）
//!
//! 目标：
//! 1. 理解 Agent 三要素：模型 + preamble（系统提示词）+ prompt（本次任务）
//! 2. 理解「换 provider 只换 client」
//!
//! 骨架基于 rig 0.46.0；若与 rustdoc 有出入，以 https://docs.rs/rig/latest/rig/ 为准。

use rig::agent::AgentBuilder;
use rig::prelude::*;
use rig::providers::openai::{self, OpenAI};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // client 从环境变量 OPENAI_API_KEY 读取密钥，并产出它支持的模型
    let model = OpenAI::from_env()?.completion(openai::GPT_5_2);

    let comedian = AgentBuilder::new(model)
        .preamble("You are a comedian here to entertain the user using humour and jokes.")
        .build();

    let response = comedian.prompt("Entertain me!").await?;
    println!("{}", response.output());

    // ---- 课堂练习 ----
    // 1. 把 preamble 换成中文人设，观察回答语言是否改变
    // 2. 加 .temperature(0.0) / .temperature(1.0)，各跑三次对比
    // 3. 进阶：换成 Anthropic client（只需换前两行），业务代码不动
    //    use rig::providers::anthropic::{self, ANTHROPIC};
    Ok(())
}
