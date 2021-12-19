use std::fs;
use std::io::Write;
use std::path::Path;

use codegen::*;

fn main() -> Result<(), ::std::io::Error> {
    let path_dir = Path::new("build");
    fs::create_dir_all(path_dir)?;

    let mut file = fs::File::create(path_dir.join("arity.rs"))?;
    writeln!(&mut file, "#![allow(clippy::all)]")?;
    writeln!(&mut file, "{}", arity::gen_arity())?;

    file = fs::File::create(path_dir.join("split.rs"))?;
    writeln!(&mut file, "#![allow(clippy::all)]")?;
    writeln!(&mut file, "{}", split::gen_split_first())?;
    writeln!(&mut file, "{}", split::gen_split_last())?;

    Ok(())
}
