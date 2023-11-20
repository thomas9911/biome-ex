use biome_fs::RomePath;
use biome_service::file_handlers::Language;
use biome_service::workspace::FileGuard;
use biome_service::workspace::{ChangeFileParams, FormatFileParams, OpenFileParams};
use biome_service::WorkspaceRef;

use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;

#[rustler::nif]
fn format(path: &str) -> Result<(), String> {
    let workspace = WorkspaceRef::Owned(biome_service::workspace::server());
    let rust_path = Path::new(path);
    let path = RomePath::new(path);
    
    {
        let mut open_file = File::options()
            .read(true)
            .write(true)
            .open(rust_path)
            .map_err(|err| err.to_string())?;
        let mut contents = String::new();
        open_file
            .read_to_string(&mut contents)
            .map_err(|err| err.to_string())?;
        // let contents = std::fs::read_to_string(&rust_path).map_err(|err| err.to_string())?;
        let guard = FileGuard::open(
            &*workspace,
            OpenFileParams {
                path: path.clone(),
                content: contents,
                version: 0,
                language_hint: Language::from_path(&rust_path),
            },
        )
        .map_err(|err| err.to_string())?;

        let printed = guard.format_file().map_err(|err| err.to_string())?;

        let current_file_contents = guard.get_file_content().map_err(|err| err.to_string())?;
        let formated_code = printed.as_code();

        if current_file_contents == formated_code {
            return Ok(())
        } else {
            open_file.rewind().map_err(|err| err.to_string())?;
            open_file
                .set_len(0)
                .map_err(|err| err.to_string())?;
    
            open_file
                .write_all(printed.as_code().as_bytes())
                .map_err(|err| err.to_string())?;
    
            guard
                .change_file(1, printed.into_code())
                .map_err(|err| err.to_string())?;
        }
    }

    // workspace.open_file(OpenFileParams{
    //     path: path.clone(),
    //     content: String::new(),
    //     version: 0,
    //     language_hint: Language::from_path(&rust_path),
    // }).map_err(|err| err.to_string())?;
    // let printed = workspace.format_file(FormatFileParams{
    //     path: path.clone(),
    // }).map_err(|err| err.to_string())?;
    // workspace.change_file(ChangeFileParams{
    //     path: path,
    //     content: printed.into_code(),
    //     version: 1,
    // }).map_err(|err| err.to_string())?;

    Ok(())
}

rustler::init!("Elixir.BiomeJS.Native", [format]);
