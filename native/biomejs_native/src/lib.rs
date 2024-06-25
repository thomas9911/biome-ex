use biome_configuration::PartialFormatterConfiguration;
use biome_configuration::{
    PartialConfiguration, PartialJavascriptConfiguration, PartialJavascriptFormatter,
};
use biome_formatter::{AttributePosition, IndentWidth, LineEnding, LineWidth, QuoteStyle};
use biome_fs::BiomePath;
use biome_service::workspace::OpenFileParams;
use biome_service::workspace::OpenProjectParams;
use biome_service::workspace::RegisterProjectFolderParams;
use biome_service::workspace::UpdateProjectParams;
use biome_service::workspace::UpdateSettingsParams;
use biome_service::workspace::{DocumentFileSource, FormatFileParams};
use biome_service::WorkspaceRef;
use rustler::NifException;
use rustler::NifUnitEnum;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;
use std::path::PathBuf;

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
            FileType::Js => "js",
            FileType::Jsx => "jsx",
            FileType::Ts => "ts",
            FileType::Tsx => "tsx",
            FileType::Json => "json",
            FileType::Jsonc => "jsonc",
            FileType::Other => "txt",
        }
    }
}

#[rustler::nif]
fn format(path: &str) -> Result<rustler::Atom, Exception> {
    let workspace = WorkspaceRef::Owned(biome_service::workspace::server());
    let rust_path = Path::new(path);
    let path = BiomePath::new(path);

    workspace
        .register_project_folder(RegisterProjectFolderParams {
            path: Some(rust_path.to_path_buf()),
            set_as_current_workspace: true,
        })
        .map_err(|e| Exception {
            message: e.to_string(),
        })?;

    workspace
        .update_settings(UpdateSettingsParams {
            configuration: PartialConfiguration {
                formatter: Some(PartialFormatterConfiguration{
                    format_with_errors: Some(true),
                    ..Default::default()
                }),
                javascript: Some(PartialJavascriptConfiguration {
                    formatter: Some(PartialJavascriptFormatter {
                        quote_style: Some(QuoteStyle::Single),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            vcs_base_path: None,
            gitignore_matches: Vec::new(),
            workspace_directory: std::env::current_dir().ok(),
        })
        .unwrap();

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
        let previous_contents = contents.clone();

        workspace.open_file(OpenFileParams {
            path: path.clone(),
            content: contents,
            version: 0,
            document_file_source: Some(DocumentFileSource::from_path(&rust_path)),
        })?;

        let printed = workspace.format_file(FormatFileParams { path })?;
        let formated_code = printed.as_code();

        if previous_contents == formated_code {
            return Ok(unchanged());
        } else {
            open_file.rewind()?;

            open_file.set_len(0)?;

            open_file.write_all(formated_code.as_bytes())?;

            return Ok(formatted());
        }
    }
}

fn inner_format_string(id: &str, file_type: FileType, code: String) -> Result<String, Exception> {
    let workspace = biome_service::workspace::server();
    let rust_path = PathBuf::from(format!("{}.{}", id, file_type.extension()));
    let path = BiomePath::new(format!("{}.{}", id, file_type.extension()));

    workspace
        .register_project_folder(RegisterProjectFolderParams {
            path: Some(rust_path),
            set_as_current_workspace: true,
        })
        .map_err(|e| Exception {
            message: e.to_string(),
        })?;

    workspace
        .update_settings(UpdateSettingsParams {
            configuration: PartialConfiguration {
                formatter: Some(PartialFormatterConfiguration{
                    format_with_errors: Some(true),
                    ..Default::default()
                }),
                javascript: Some(PartialJavascriptConfiguration {
                    formatter: Some(PartialJavascriptFormatter {
                        quote_style: Some(QuoteStyle::Single),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            vcs_base_path: None,
            gitignore_matches: Vec::new(),
            workspace_directory: std::env::current_dir().ok(),
        })
        .unwrap();

    {
        workspace.open_file(OpenFileParams {
            path: path.clone(),
            content: code,
            version: 0,
            document_file_source: Some(DocumentFileSource::from_path(&path)),
        })?;

        let printed = workspace.format_file(FormatFileParams { path })?;

        Ok(printed.into_code())
    }
}

#[rustler::nif]
fn format_string(id: &str, file_type: FileType, code: String) -> Result<String, Exception> {
    inner_format_string(id, file_type, code)
}

rustler::init!("Elixir.BiomeJS.Native", [format, format_string]);
