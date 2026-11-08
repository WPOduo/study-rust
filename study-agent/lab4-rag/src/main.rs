//! Lab4：RAG 文档问答（课程模块 D2-M7）
//!
//! 流程：加载文档 -> embedding 入库 -> dynamic_context 检索增强 -> 回答
//!
//! 注意：本文件是**参考骨架**。embedding / 入库的具体签名在不同小版本间变动较频繁，
//! 编译不过时请对照 rustdoc（https://docs.rs/rig/latest/rig/vector_store/）与
//! 讲义 5.6 节微调；核心概念不变。

use rig::agent::AgentBuilder;
use rig::embeddings::Embed;
use rig::prelude::*;
use rig::providers::openai::{self, OpenAI};
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::vector_store::InsertDocuments;

/// 文档类型：#[embed] 标记的字段参与向量化，其余字段作为元数据存储。
#[derive(Embed, Clone)]
struct Doc {
    #[embed]
    content: String,
    path: String,
}

fn load_docs(dir: &str) -> anyhow::Result<Vec<Doc>> {
    let mut docs = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "md") {
            let content = std::fs::read_to_string(&path)?;
            docs.push(Doc {
                content,
                path: path.to_string_lossy().into_owned(),
            });
        }
    }
    Ok(docs)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = OpenAI::from_env()?;

    // 1) embedding 模型：负责把文本变成向量
    let embedder = client.embedding(openai::TEXT_EMBEDDING_3_SMALL, None);

    // 2) 加载文档并入库（内存向量库，余弦相似度）
    let documents = load_docs("./docs")?;
    let mut store = InMemoryVectorStore::default();
    store.insert_documents(documents, &embedder).await?;

    // 3) 检索增强 Agent：每次 prompt 自动检索 top-3 注入上下文
    let model = client.completion(openai::GPT_5_2);
    let qa = AgentBuilder::new(model)
        .name("kb-qa")
        .preamble("仅依据提供的上下文回答；上下文里没有的信息，明确说不知道。")
        .dynamic_context(3, store)
        .build();

    let response = qa.prompt("rig 换模型厂商要改多少业务代码？").await?;
    println!("{}", response.output());

    // ---- 课堂练习 ----
    // 1. 问一个 docs 里没有的问题，验证它是否回答「不知道」
    // 2. 把 dynamic_context(3, ..) 改成 (1, ..)，观察回答质量变化
    // 3. 进阶：把 InMemoryVectorStore 换成 qdrant（features = ["qdrant"]），
    //    体会「换库只换类型」
    Ok(())
}
