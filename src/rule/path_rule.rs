use std::path::{Path, PathBuf};

use crate::rule::Rule;

pub struct PathRule {
    path: PathBuf,
}

impl PathRule {
    pub fn new(path: &Path) -> PathRule {
        PathRule {
            path: path.to_path_buf(),
        }
    }
}

impl Rule for PathRule {
    fn is_ignored(&self, path: &Path) -> bool {
        // TODO(AD)
        self.path == path
    }

    fn is_override(&self, _path: &Path) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO(AD)

    #[test]
    fn ignore_path() {
        let rule = PathRule::new(Path::new("a/b/c"));
        assert!(rule.is_ignored(Path::new("a/b/c")));
    }
}