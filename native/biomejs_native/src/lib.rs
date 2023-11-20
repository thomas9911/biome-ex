use biome_fs::RomePath;
use biome_service::file_handlers::Language;
use biome_service::workspace::FileGuard;
use biome_service::workspace::{ChangeFileParams, FormatFileParams, OpenFileParams};
use biome_service::WorkspaceRef;

use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;

use rustler::NifException;

rustler::atoms! {
    unchanged,
    formatted,
}

#[derive(Debug, NifException)]
#[module = "BiomeJS.Exception"]
pub struct Exception {
    message: String,
}

impl<E: std::error::Error> From<E> for Exception {
    fn from(err: E) -> Exception {
        Exception{
            message: err.to_string()
        }
    }
}

#[rustler::nif]
fn format(path: &str) -> Result<rustler::Atom, Exception> {
    let workspace = WorkspaceRef::Owned(biome_service::workspace::server());
    let rust_path = Path::new(path);
    let path = RomePath::new(path);

    {
        let mut open_file = File::options()
            .read(true)
            .write(true)
            .open(rust_path)?;

        let mut contents = String::new();
        open_file
            .read_to_string(&mut contents)?;

        let guard = FileGuard::open(
            &*workspace,
            OpenFileParams {
                path: path.clone(),
                content: contents,
                version: 0,
                language_hint: Language::from_path(&rust_path),
            },
        )?;

        let printed = guard.format_file()?;

        let current_file_contents = guard.get_file_content()?;
        let formated_code = printed.as_code();

        if current_file_contents == formated_code {
            return Ok(unchanged())
        } else {
            open_file.rewind()?;
            
            open_file
                .set_len(0)?;
    
            open_file
                .write_all(printed.as_code().as_bytes())?;
    
            guard
                .change_file(1, printed.into_code())?;
            return Ok(formatted())
        }
    }
}

rustler::init!("Elixir.BiomeJS.Native", [format]);
