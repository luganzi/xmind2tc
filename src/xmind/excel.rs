use std::path::Path;

use anyhow::Result;
use simple_excel_writer::{row, Row, Workbook};

use super::{parser, TestCase};

pub fn parse_xmind_to_xls(filename: &str) -> Result<()> {
    let xmind = parser::load_xmind(filename)?;
    let test_case_list = parser::xmind_to_testcase(&xmind);

    let path = Path::new(filename).with_extension("xlsx");
    let xlsx = path.to_str().unwrap_or("test case.xlsx");

    let mut wb = create_workbook_by_xmind(xlsx)?;
    let mut sheet = wb.create_sheet("测试用例");

    wb.write_sheet(&mut sheet, |w| {
        w.append_row(xls_header_row())?;

        for tc in test_case_list {
            w.append_row(test_case_to_row(tc))?;
        }

        Ok(())
    })?;

    wb.close()?;

    Ok(())
}

fn create_workbook_by_xmind(filename: &str) -> Result<Workbook> {
    let path = Path::new(filename);
    if path.exists() {
        std::fs::remove_file(path)?;
    }

    let wb = Workbook::create(filename);
    Ok(wb)
}

fn test_case_to_row(tc: TestCase) -> Row {
    row![
        tc.id.as_str(),
        tc.project,
        tc.module,
        tc.sub_module,
        tc.function.as_str(),
        tc.steps.join("\n"),
        tc.expect,
        tc.priority,
        tc.func_type
    ]
}

fn xls_header_row() -> Row {
    row![
        "编号",
        "项目",
        "模块",
        "功能点",
        "测试点",
        "测试步骤",
        "预期结果",
        "优先级",
        "用例类型"
    ]
}
