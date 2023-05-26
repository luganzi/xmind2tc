use std::io::Read;

use anyhow::{Context, Result};

use super::{stack::Stack, TestCase, Xmind};

const CONTENT_JSON_FILENAME: &str = "content.json";

pub(super) fn load_xmind(filename: &str) -> Result<Xmind> {
    let fp = std::fs::File::open(filename)
        .with_context(|| format!("unable to open file: {}", &filename))?;
    let mut zip = zip::ZipArchive::new(fp)?;
    let mut content = zip
        .by_name(CONTENT_JSON_FILENAME)
        .with_context(|| format!("unable to locate file: {}", CONTENT_JSON_FILENAME))?;

    let mut buffer = String::new();
    content.read_to_string(&mut buffer)?;

    let xmind: Xmind =
        serde_json::from_str(&buffer).with_context(|| format!("failed to parse xmind file"))?;

    Ok(xmind)
}

pub(super) fn xmind_to_testcase<'a>(xmind: &'a Xmind) -> Vec<TestCase<'a>> {
    let mut test_case_list = Vec::new();

    for sheet in xmind.iter() {
        // require xmind sheet name ends with '.tc'
        if !sheet.title.to_uppercase().ends_with(".TC") {
            continue;
        }

        let root_node = match sheet.root_topic {
            Some(ref n) => n,
            None => continue,
        };

        let mut left_stack = Stack::new();
        let mut right_stack = Stack::new();

        left_stack.push(root_node);
        right_stack.push(root_node.get_children());

        let mut test_case_id = 1;
        while left_stack.len() > 0 {
            let right_top_nodes = match right_stack.peek() {
                Some(ns) => *ns,
                None => [].as_slice(),
            };

            if right_top_nodes.len() > 0 {
                // continue to move node
                let move_left_node = &right_top_nodes[0];
                left_stack.push(move_left_node);

                right_stack.pop();
                right_stack.push(&right_top_nodes[1..]);
                right_stack.push(move_left_node.get_children());
            } else {
                // when right top nodes is empty, there is no children
                // read test case from left stack
                if left_stack.len() >= 5 {
                    let expect = &left_stack.pop().unwrap().title;
                    // left pop, right should pop
                    right_stack.pop();

                    let mut iter = left_stack.iter();

                    let project_node = iter.next().unwrap();
                    let module_node = iter.next().unwrap();
                    let sub_module_node = iter.next().unwrap();
                    let function_node = iter.next().unwrap();

                    let mut function = String::new();
                    function.push_str(function_node.title.as_str());
                    let mut steps = Vec::new();

                    let mut is_condition = function_node.get_children().len() > 1;
                    let mut step_no = 1;
                    for n in iter {
                        if is_condition {
                            function.push_str(", ");
                            function.push_str(&n.title);
                        } else {
                            steps.push(format!("{}.{}", step_no, &n.title));
                            step_no += 1;
                        }

                        is_condition = n.get_children().len() > 1;
                    }

                    let test_case = TestCase {
                        id: format!("FUNC-{}-{:03}", &module_node.title, test_case_id),
                        project: &project_node.title,
                        module: &module_node.title,
                        sub_module: &sub_module_node.title,
                        function: function,
                        steps,
                        expect,
                        priority: "高",
                        func_type: "功能测试",
                    };
                    test_case_id += 1;

                    test_case_list.push(test_case);
                }

                loop {
                    if right_stack.len() > 0 && right_stack.peek().unwrap().len() == 0 {
                        right_stack.pop();
                        left_stack.pop();
                        continue;
                    }
                    break;
                }
            }
        }
    }

    test_case_list
}
