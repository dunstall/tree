use crate::formatter::Formatter;
use crate::fs::File;
use crate::summary::Summary;
use crate::UI;

pub struct StdoutUI {
    formatter: Formatter,
}

impl StdoutUI {
    pub fn new(formatter: Formatter) -> StdoutUI {
        StdoutUI {
            formatter: formatter,
        }
    }
}

impl UI for StdoutUI {
    fn file(&self, file: File, depth: usize, is_last: bool, is_dir: bool) {
        print!("{}", self.formatter.file(file, depth, is_last, is_dir));
    }

    fn summary(&self, summary: &Summary) {
        print!(
            "\n{}",
            self.formatter.summary(summary.n_dirs, summary.n_files)
        );
    }

    fn add_dir(&mut self, depth: usize) {
        self.formatter.add_dir(depth)
    }

    fn remove_dir(&mut self, depth: usize) {
        self.formatter.remove_dir(depth)
    }
}
