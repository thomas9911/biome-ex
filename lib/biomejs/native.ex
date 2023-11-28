defmodule BiomeJS.Native do
  @moduledoc false

  use Rustler, otp_app: :biomejs, crate: "biomejs_native"

  def format(_file), do: :erlang.nif_error(:nif_not_loaded)
  def format_string(_id, _file_type, _code), do: :erlang.nif_error(:nif_not_loaded)
end
