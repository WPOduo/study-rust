//! Lab2：自定义工具（课程模块 D1-M4）
//!
//! 目标：
//! 1. 用 `#[rig_tool]` 宏把普通 async fn 变成模型可调用的工具
//! 2. 观察多轮工具调用循环：模型声明调用 -> 你的代码执行 -> 结果回灌 -> 循环
//! 3. 体会 description 与错误信息对模型行为的影响
//!
//! 骨架基于 rig 0.46.0；若与 rustdoc 有出入，以 https://docs.rs/rig/latest/rig/ 为准。

use rig::agent::AgentBuilder;
use rig::prelude::*;
use rig::providers::openai::{self, OpenAI};
use rig::tool::rig_tool;

/// 工具错误：必须是模型能读懂、能自我纠正的信息，不要 panic。
#[derive(Debug)]
enum ToolError {
    Io(std::io::Error),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for ToolError {}

impl From<std::io::Error> for ToolError {
    fn from(e: std::io::Error) -> Self {
        ToolError::Io(e)
    }
}

/// 统计一个文本文件的单词数。
/// 注意：doc 注释/描述要写「做什么 + 什么时候选它」，这是模型选工具的唯一依据。
#[rig_tool]
async fn word_count(path: String) -> Result<usize, ToolError> {
    let text = std::fs::read_to_string(&path)?;
    Ok(text.split_whitespace().count())
}

/// 列出目录下所有 .md 文件的相对路径。
#[rig_tool]
async fn list_md_files(dir: String) -> Result<Vec<String>, ToolError> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "md") {
            out.push(path.to_string_lossy().into_owned());
        }
    }
    Ok(out)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let model = OpenAI::from_env()?.completion(openai::GPT_5_2);

    let agent = AgentBuilder::new(model)
        .name("file-stats")
        .preamble("你是文件统计助手。只依据工具返回的事实回答，不要臆造数字。")
        // #[rig_tool] 会生成大写静态量 WORD_COUNT / LIST_MD_FILES；
        // 若你的版本生成的是类型，改为 .tool(WordCount::default())
        .tool(WORD_COUNT)
        .tool(LIST_MD_FILES)
        .default_max_turns(8) // 防失控循环；注意不是 max_turns()
        .build();

    let task = "统计 data 目录下每个 .md 文件的字数，并给出总字数。";
    let response = agent.prompt(task).await?;
    println!("{}", response.output());

    // ---- 课堂练习 ----
    // 1. 传一个不存在的路径给 word_count，观察模型是否根据错误信息重试
    // 2. 把 word_count 的实现包进 blocking(..)，思考为什么阻塞 IO 不能直接放 async fn
    // 3. 进阶：手写 impl Tool 版本的 word_count（参考讲义 5.3 第二种写法）
    Ok(())
}
