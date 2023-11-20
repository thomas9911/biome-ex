defmodule BiomeJS.Native do
  use Rustler, otp_app: :biomejs, crate: "biomejs_native"

  # When your NIF is loaded, it will override this function.
  def format(_file), do: :erlang.nif_error(:nif_not_loaded)
end
