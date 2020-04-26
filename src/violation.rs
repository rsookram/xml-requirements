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
        line_num: u32,
        col_num: u32,
        tag: &str,
        attribute: &str,
    ) -> Violation {
        Violation {
            path: path.clone(),
            line_num,
            col_num,
            tag: tag.to_string(),
            attribute: attribute.to_string(),
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{} {} missing {}",
            self.path.to_str().unwrap(),
            self.line_num,
            self.col_num,
            self.tag,
            self.attribute,
        )
    }
}
