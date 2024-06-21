use biome_configuration::PartialFormatterConfiguration;
use biome_service::workspace::DocumentFileSource;
use rustler::NifUnitEnum;
use biome_fs::BiomePath;
use biome_configuration::PartialConfiguration;
use biome_service::workspace::FileGuard;
use biome_service::workspace::OpenFileParams;
use biome_service::workspace::UpdateSettingsParams;
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
        Exception {
            message: err.to_string(),
        }
    }
}

#[derive(Debug, NifUnitEnum)]
enum FileType {
    Js,
    Jsx,
    Ts,
    Tsx,
    Json,
    Jsonc,
    Other,
}

impl FileType {
    fn extension(&self) -> &'static str {
        match self {
            FileType::Js => ".js",
            FileType::Jsx => ".jsx",
            FileType::Ts => ".ts",
            FileType::Tsx => ".tsx",
            FileType::Json => ".json",
            FileType::Jsonc => ".jsonc",
            FileType::Other => ".txt",
        }
    }
}

// impl From<FileType> for Language {
//     fn from(file_type: FileType) -> Language {
//         match file_type {
//             FileType::Js => Language::JavaScript,
//             FileType::Jsx => Language::JavaScriptReact,
//             FileType::Ts => Language::TypeScript,
//             FileType::Tsx => Language::TypeScriptReact,
//             FileType::Json => Language::Json,
//             FileType::Jsonc => Language::Jsonc,
//             FileType::Other => Language::Unknown,
//         }
//     }
// }

#[rustler::nif]
fn format(path: &str) -> Result<rustler::Atom, Exception> {
    let workspace = WorkspaceRef::Owned(biome_service::workspace::server());
    let rust_path = Path::new(path);
    let path = BiomePath::new(path);

    {
        let mut open_file = File::options()
            .read(true)
            .write(true)
            .open(rust_path)
            .map_err(|e| Exception {
                message: e.kind().to_string(),
            })?;

        let mut contents = String::new();
        open_file.read_to_string(&mut contents)?;

        let guard = FileGuard::open(
            &*workspace,
            OpenFileParams {
                path: path.clone(),
                content: contents,
                version: 0,
                document_file_source: Some(DocumentFileSource::from_path(&rust_path)),
            },
        )?;

        let printed = guard.format_file()?;

        let current_file_contents = guard.get_file_content()?;
        let formated_code = printed.as_code();

        if current_file_contents == formated_code {
            return Ok(unchanged());
        } else {
            open_file.rewind()?;

            open_file.set_len(0)?;

            open_file.write_all(printed.as_code().as_bytes())?;

            guard.change_file(1, printed.into_code())?;
            return Ok(formatted());
        }
    }
}

fn inner_format_string(id: &str, file_type: FileType, code: String) -> Result<String, Exception> {
    let workspace = biome_service::workspace::server();
    workspace.update_settings(UpdateSettingsParams {
        configuration: PartialConfiguration {
            formatter: Some(PartialFormatterConfiguration {
                format_with_errors: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        },
        vcs_base_path: None,
        gitignore_matches: vec![],
        workspace_directory: None
    })?;

    let workspace_ref = WorkspaceRef::Owned(workspace);
    let path = BiomePath::new(format!("{}{}", id, file_type.extension()));

    {
        let guard = FileGuard::open(
            &*workspace_ref,
            OpenFileParams {
                path,
                content: code,
                version: 0,
                document_file_source: Some(DocumentFileSource::from_extension(file_type.extension()))
            },
        )?;

        let printed = guard.format_file()?;

        Ok(printed.into_code())
    }
}

#[rustler::nif]
fn format_string(id: &str, file_type: FileType, code: String) -> Result<String, Exception> {
    inner_format_string(id, file_type, code)
}

rustler::init!("Elixir.BiomeJS.Native", [format, format_string]);
