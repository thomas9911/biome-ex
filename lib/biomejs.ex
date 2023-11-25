defmodule BiomeJS do
  @moduledoc """
  Format code using BiomeJS
  """

  @spec format(binary) :: {:ok, atom} | {:error, BiomeJS.Exception}
  defdelegate format(path), to: BiomeJS.Native

  @spec format_js_string(binary) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_js_string(code) do
    random_string =
      16
      |> :rand.bytes()
      |> Base.encode32(case: :lower, padding: false)

    BiomeJS.Native.format_js_string("#{random_string}.js", code)
  end
end
