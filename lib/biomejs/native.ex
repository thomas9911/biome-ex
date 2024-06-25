defmodule BiomeJS.Native do
  @moduledoc false

  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :biomejs,
    crate: "biomejs_native",
    base_url: "https://github.com/thomas9911/biome-ex/releases/download/v#{version}",
    force_build: Application.compile_env(:biomejs, :force_build, false),
    targets:
      Enum.uniq(["aarch64-unknown-linux-musl" | RustlerPrecompiled.Config.default_targets()]),
    version: version,
    nif_versions: ["2.16"],
    mode: Application.compile_env(:biomejs, :mode, :release)

  def format(_file, _options), do: :erlang.nif_error(:nif_not_loaded)
  def format_string(_id, _file_type, _code, _options), do: :erlang.nif_error(:nif_not_loaded)
end
