use std::path::Path;

use tree::{Args, Matcher, Result, StdoutUI, Tree, OSFS};

fn main() -> Result<()> {
    let args = Args::parse_cli()?;
    let matcher = Matcher::new(args.show_hidden, args.directories_only);
    let mut tree = Tree::new(matcher, OSFS::new(), StdoutUI::new());
    tree.walk(Path::new(&args.dir))?;
    Ok(())
}
