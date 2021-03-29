use std::path::{Path, PathBuf};
use std::vec::Vec;

pub use crate::error::Result;
pub use crate::rule::{OverrideRule, PathRule, PriorityRule, Rule};

// See https://git-scm.com/docs/gitignore#_pattern_format.
// Note order of returned priority rule important (deeper directories and lower
// within file take priority).
pub struct IgnoreConfig {
    content: String,
    root: PathBuf,
}

// TODO(AD) Maybe IgnoreConfigRule needed to handle matching paths relative to
// the path of the gitignore
impl IgnoreConfig {
    pub fn new(content: &str, root: &Path) -> IgnoreConfig {
        IgnoreConfig {
            content: content.to_string(),
            root: root.to_path_buf(),
        }
    }

    pub fn rule(&self) -> impl Rule {
        let mut rules: Vec<Box<dyn Rule>> = vec![];
        for line in self.content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with("#") {
                continue;
            }

            if line.starts_with("!") {
                if let Some(line) = line.strip_prefix("!") {
                    let rule = Box::new(PathRule::new(Path::new(line)));
                    rules.push(Box::new(OverrideRule::new(rule)));
                }
            } else {
                let path = if Path::new(line).is_absolute() {
                    let path = Path::new(line);
                    if let Ok(path) = path.strip_prefix("/") {
                        self.root.join(path)
                    } else {
                        path.to_path_buf()
                    }
                } else {
                    Path::new(line).to_path_buf()
                };

                let rule = Box::new(PathRule::new(&path));
                rules.push(rule);
            }
        }
        // Reverse since the rules are in priority order from last to first.
        rules.reverse();
        PriorityRule::new(rules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO(AD) Test absolute path / taken relative to root.

    #[test]
    fn test_override_ignore() {
        let content = r#"
        myfile
        !myfile
        "#;

        // TODO(AD)
        let rule = IgnoreConfig::new(content, Path::new("")).rule();
        assert!(!rule.is_ignored(Path::new("myfile")));
        // assert!(rule.is_ignored(Path::new("mydir/myfile")));
        // assert!(rule.is_ignored(Path::new("myfile/myotherfile")));
    }

    #[test]
    fn test_ignore_file() {
        // TODO(AD)
        let rule = IgnoreConfig::new("myfile", Path::new("")).rule();
        assert!(rule.is_ignored(Path::new("myfile")));
        // assert!(rule.is_ignored(Path::new("mydir/myfile")));
        // assert!(rule.is_ignored(Path::new("myfile/myotherfile")));

        assert!(!rule.is_ignored(Path::new("notmyfile")));
        // assert!(!rule.is_ignored(Path::new("notmydir/notmyfile")));
    }

    #[test]
    fn test_ignore_comments() {
        let content = r#"
# commented


#commented
ignored
  # commented
"#;
        let rule = IgnoreConfig::new(content, Path::new("")).rule();
        assert!(rule.is_ignored(Path::new("ignored")));
        assert!(!rule.is_ignored(Path::new("commented")));
        assert!(!rule.is_ignored(Path::new("#commented")));
    }

    #[test]
    fn test_empty() {
        let rule = IgnoreConfig::new("", Path::new("")).rule();
        assert!(!rule.is_ignored(Path::new("myfile")));
    }
}