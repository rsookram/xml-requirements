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
            self.path.display(),
            self.line_num,
            self.col_num,
            self.tag,
            self.attribute,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let v = Violation::new(
            &"/path/to/file.xml".into(),
            3,
            8,
            "LinearLayout",
            "android:orientation",
        );

        let actual = format!("{}", v);

        assert_eq!(
            "/path/to/file.xml:3:8 LinearLayout missing android:orientation",
            actual
        );
    }
}
