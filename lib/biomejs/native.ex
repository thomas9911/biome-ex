defmodule BiomeJS.Native do
  @moduledoc false

  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :biomejs,
    crate: "biomejs_native",
    base_url: "https://github.com/thomas9911/biome-ex/releases/download/v#{version}",
    force_build: true,
    targets:
      Enum.uniq(["aarch64-unknown-linux-musl" | RustlerPrecompiled.Config.default_targets()]),
    version: version,
    nif_versions: ["2.16"],
    mode: :debug

  def format(_file), do: :erlang.nif_error(:nif_not_loaded)
  def format_string(_id, _file_type, _code), do: :erlang.nif_error(:nif_not_loaded)
end
