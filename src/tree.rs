use std::path::{Path, PathBuf};

use crate::entry::Entry;
use crate::error::Result;
use crate::fs::FS;
use crate::matcher::Matcher;
use crate::ui::UI;

pub struct Tree<F, U> {
    matcher: Matcher,
    fs: F,
    ui: U,
}

impl<F, U> Tree<F, U>
where
    F: FS,
    U: UI,
{
    pub fn new(matcher: Matcher, fs: F, ui: U) -> Tree<F, U> {
        Tree {
            matcher: matcher,
            fs: fs,
            ui: ui,
        }
    }

    pub fn walk(&mut self, dir: &Path) -> Result<()> {
        self.ui.file(&Entry {
            file_name: dir.to_str().unwrap().to_string(),
            depth: 0,
            is_last: false,
        });
        let (n_dirs, n_files) = self.walk_nested(dir, 1)?;
        print!("\n{} directories, {} files", n_dirs, n_files);
        Ok(())
    }

    fn walk_nested(&mut self, dir: &Path, depth: usize) -> Result<(usize, usize)> {
        if !dir.is_dir() {
            return Ok((0, 0));
        }

        self.ui.add_indent(depth - 1);

        let mut n_dirs = 0;
        let mut n_files = 0;
        let list = self.list_dir_matches(dir)?;
        for i in 0..list.len() {
            let path = &list[i];
            if let Some(file_name) = path.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    if path.is_dir() {
                        if i == list.len() - 1 {
                            self.ui.remove_indent(depth - 1);
                        }

                        n_dirs += 1;
                        self.ui.file(&Entry {
                            file_name: file_name.to_string(),
                            depth: depth,
                            is_last: i == list.len() - 1,
                        });

                        let (n_nested_dirs, n_nested_files) = self.walk_nested(&path, depth + 1)?;
                        n_dirs += n_nested_dirs;
                        n_files += n_nested_files;
                    } else {
                        n_files += 1;
                        self.ui.file(&Entry {
                            file_name: file_name.to_string(),
                            depth: depth,
                            is_last: i == list.len() - 1,
                        });
                    }
                }
            }
        }
        self.ui.remove_indent(depth);

        Ok((n_dirs, n_files))
    }

    fn list_dir_matches(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut paths = vec![];
        for path in self.fs.list_dir(dir)? {
            if self.matcher.is_match(&path) {
                paths.push(path);
            }
        }
        Ok(paths)
    }
}
