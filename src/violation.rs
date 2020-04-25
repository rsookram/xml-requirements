use roxmltree::Document;
use std::fmt;
use std::path::PathBuf;

pub struct Violation {
    pub path: PathBuf,
    pub line_num: u32,
    pub col_num: u32,
    pub tag: String,
    pub attribute: String,
}

impl Violation {
    pub fn new(
        path: &PathBuf,
        doc: &Document,
        node: &roxmltree::Node,
        attribute: &str,
    ) -> Violation {
        let pos = doc.text_pos_at(node.range().start);

        Violation {
            path: path.clone(),
            line_num: pos.row,
            col_num: pos.col,
            tag: node.tag_name().name().to_string(),
            attribute: attribute.to_string(),
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{} {} missing {} attribute",
            self.path.to_str().unwrap(),
            self.line_num,
            self.col_num,
            self.tag,
            self.attribute,
        )
    }
}
