use std::path::PathBuf;

use anyhow::{anyhow, Result};
use serde::Serialize;
use serde_yaml::Value;

use crate::project_root;

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ItemIDNode {
    Leaf { node: String, value: u32 },
    Node { node: String, children: Vec<ItemIDNode> },
}

impl TryFrom<(Value, Value)> for ItemIDNode {
    type Error = anyhow::Error;

    fn try_from((k, v): (Value, Value)) -> Result<Self> {
        match (k, v) {
            (Value::String(s), Value::Number(n)) => {
                Ok(ItemIDNode::Leaf { node: s, value: n.as_u64().unwrap() as u32 })
            },
            (Value::String(s), Value::Mapping(m)) => Ok(ItemIDNode::Node {
                node: s,
                children: m.into_iter().map(|(k, v)| ItemIDNode::try_from((k, v))).try_fold(
                    Vec::new(),
                    |mut o: Vec<_>, i: Result<ItemIDNode>| -> Result<Vec<_>> {
                        let i = i?;
                        o.push(i);
                        Result::Ok(o)
                    },
                )?,
            }),
            (a, b) => Err(anyhow!("invalid value {:?} {:?}", a, b)),
        }
    }
}

fn codegen_dir() -> PathBuf {
    project_root().join("xtask").join("src").join("codegen")
}

fn item_ids_json_path() -> PathBuf {
    project_root().join("practice-tool").join("src").join("widgets").join("item_ids.json")
}

fn read_yml(name: &str) -> Result<Value> {
    let file = std::fs::File::open(codegen_dir().join(name))?;
    serde_yaml::from_reader(file).map_err(|e| e.into())
}

/// Convert a top-level YAML mapping into a list of item ID nodes.
fn mapping_to_nodes(val: Value) -> Result<Vec<ItemIDNode>> {
    match val {
        Value::Mapping(m) => m.into_iter().map(|(k, v)| ItemIDNode::try_from((k, v))).collect(),
        _ => Err(anyhow!("invalid input format")),
    }
}

pub(crate) fn codegen() -> Result<()> {
    let mut nodes = mapping_to_nodes(read_yml("item_ids.yml")?)?;
    nodes.extend(mapping_to_nodes(read_yml("item_ids_cer.yml")?)?);

    serde_json::to_writer_pretty(std::fs::File::create(item_ids_json_path())?, &nodes)?;

    Ok(())
}
