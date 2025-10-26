use biome_configuration::{Configuration, FormatterConfiguration};
use biome_fs::{BiomePath, OpenOptions};
use biome_service::workspace::{DocumentFileSource, FormatFileParams};
use biome_service::workspace::{FileContent, OpenFileParams, OpenProjectParams, OpenProjectResult};
use biome_service::workspace::{GetFileContentParams, UpdateSettingsParams};
use biome_service::WorkspaceRef;
use rustler::serde::SerdeTerm;
use rustler::NifException;
use rustler::NifUnitEnum;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

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
    Graphql,
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
            FileType::Graphql => "graphql",
            FileType::Other => "txt",
        }
    }
}

#[rustler::nif]
fn format(path: &str, options: SerdeTerm<serde_json::Value>) -> Result<rustler::Atom, Exception> {
    let biome_config = convert_options(options.0)?;
    let rust_path = Path::new(path);
    let path = BiomePath::new(path);
    let parent = path.parent().expect("always a file");

    let filesystem = biome_fs::OsFileSystem::new(parent.into());
    let workspace =
        WorkspaceRef::Owned(biome_service::workspace::server(Arc::new(filesystem), None));

    let OpenProjectResult { project_key } = workspace.open_project(OpenProjectParams {
        path: parent.into(),
        open_uninitialized: true,
    })?;

    workspace.update_settings(UpdateSettingsParams {
        project_key,
        configuration: biome_config,
        workspace_directory: Some(parent.into()),
    })?;

    {
        if let Err(e) = workspace.open_file(OpenFileParams {
            path: path.clone(),
            content: FileContent::FromServer,
            document_file_source: Some(DocumentFileSource::from_path(&path, false)),
            persist_node_cache: false,
            project_key,
        }) {
            match e {
                biome_service::WorkspaceError::FileSystem(_) => {
                    File::open(rust_path).map_err(|e| Exception {
                        message: e.kind().to_string(),
                    })?;
                }
                e => return Err(e.into()),
            }
        }

        let contents = workspace.get_file_content(GetFileContentParams {
            path: path.clone(),
            project_key,
        })?;
        let previous_contents = contents.clone();

        let printed = workspace.format_file(FormatFileParams {
            path: path.clone(),
            project_key,
        })?;

        let formated_code = printed.into_code();

        if previous_contents == formated_code {
            return Ok(unchanged());
        } else {
            let mut open_file = workspace
                .fs()
                .open_with_options(&path, OpenOptions::default().write(true))?;

            open_file.set_content(formated_code.as_bytes())?;

            return Ok(formatted());
        }
    }
}

#[rustler::nif]
fn format_string(
    id: &str,
    file_type: FileType,
    code: String,
    options: SerdeTerm<serde_json::Value>,
) -> Result<String, Exception> {
    let biome_config = convert_options(options.0)?;
    inner_format_string(id, file_type, code, biome_config)
}

fn inner_format_string(
    id: &str,
    file_type: FileType,
    code: String,
    biome_config: Configuration,
) -> Result<String, Exception> {
    let workspace =
        biome_service::workspace::server(Arc::new(biome_fs::MemoryFileSystem::default()), None);

    let path = BiomePath::new(format!("{}.{}", id, file_type.extension()));

    let OpenProjectResult { project_key } = workspace.open_project(OpenProjectParams {
        path: path.clone(),
        open_uninitialized: true,
    })?;

    workspace.update_settings(UpdateSettingsParams {
        project_key,
        configuration: biome_config,
        workspace_directory: Some(path.clone()),
    })?;

    {
        workspace.open_file(OpenFileParams {
            path: path.clone(),
            content: FileContent::from_client(code),
            document_file_source: Some(DocumentFileSource::from_path(&path, false)),
            persist_node_cache: false,
            project_key,
        })?;

        let printed = workspace.format_file(FormatFileParams { path, project_key })?;

        Ok(printed.into_code())
    }
}

fn convert_options(options: serde_json::Value) -> Result<Configuration, Exception> {
    let mut biome_config: Configuration =
        serde_json::from_value(options).map_err(|e| Exception {
            message: e.to_string(),
        })?;

    if let Some(ref mut formatter) = biome_config.formatter {
        formatter.format_with_errors = Some(true.into());
    } else {
        biome_config.formatter = Some(FormatterConfiguration {
            format_with_errors: Some(true.into()),
            ..Default::default()
        })
    }

    Ok(biome_config)
}

rustler::init!("Elixir.BiomeJS.Native", [format, format_string]);
