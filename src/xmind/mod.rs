mod parser;
mod excel;
mod stack;

pub use excel::parse_xmind_to_xls;
use serde::Deserialize;

type Xmind = Vec<XmindSheet>;

#[derive(Deserialize, Debug)]
struct XmindSheet {
    title: String,
    #[serde(rename = "rootTopic")]
    pub root_topic: Option<XmindNode>,
}

#[derive(Deserialize, Debug)]
struct XmindNode {
    title: String,
    children: Option<XmindAttached>,
}

#[derive(Deserialize, Debug)]
struct XmindAttached {
    attached: Vec<XmindNode>,
}

impl XmindNode {
    fn get_children(&self) -> &[XmindNode] {
        match self.children {
            Some(ref x) => x.attached.as_slice(),
            _ => [].as_slice(),
        }
    }
}

#[derive(Debug)]
pub struct TestCase<'a> {
    id: String,
    project: &'a str,
    module: &'a str,
    sub_module: &'a str,
    function: String,
    steps: Vec<String>,
    expect: &'a str,
    priority: &'a str,
    func_type: &'a str,
}