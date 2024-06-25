defmodule BiomeJS do
  @moduledoc """
  Format code using BiomeJS
  """

  @file_types [
    :js,
    :jsx,
    :ts,
    :tsx,
    :json,
    :jsonc,
    :other
  ]

  @typedoc """
  Options for BiomeJS, they are the same as in `biome.json`
  """
  @type options :: keyword()

  @spec format(binary, options) :: {:ok, atom} | {:error, BiomeJS.Exception}
  def format(path, options \\ %{}) do
    BiomeJS.Native.format(path, options)
  end

  @spec format_js_string(binary, options) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_js_string(code, options \\ %{}) do
    format_string(:js, code, options)
  end

  @spec format_ts_string(binary, options) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_ts_string(code, options \\ %{}) do
    format_string(:ts, code, options)
  end

  @spec format_jsx_string(binary, options) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_jsx_string(code, options \\ %{}) do
    format_string(:jsx, code, options)
  end

  @spec format_tsx_string(binary, options) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_tsx_string(code, options \\ %{}) do
    format_string(:tsx, code, options)
  end

  @spec format_json_string(binary, options) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_json_string(code, options \\ %{}) do
    format_string(:json, code, options)
  end

  @spec format_jsonc_string(binary, options) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_jsonc_string(code, options \\ %{}) do
    format_string(:jsonc, code, options)
  end

  @spec format_string(atom, binary, options) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_string(type, code, options \\ %{})

  def format_string(type, code, options) when type in @file_types do
    random_string =
      16
      |> :rand.bytes()
      |> Base.encode32(case: :lower, padding: false)

    BiomeJS.Native.format_string(random_string, type, code, options)
  end

  def format_string(_, code, options) do
    format_string(:other, code, options)
  end
end
