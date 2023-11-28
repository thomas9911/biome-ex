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

  @spec format(binary) :: {:ok, atom} | {:error, BiomeJS.Exception}
  defdelegate format(path), to: BiomeJS.Native

  @spec format_js_string(binary) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_js_string(code) do
    format_string(:js, code)
  end

  @spec format_ts_string(binary) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_ts_string(code) do
    format_string(:ts, code)
  end

  @spec format_jsx_string(binary) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_jsx_string(code) do
    format_string(:jsx, code)
  end

  @spec format_tsx_string(binary) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_tsx_string(code) do
    format_string(:tsx, code)
  end

  @spec format_json_string(binary) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_json_string(code) do
    format_string(:json, code)
  end

  @spec format_jsonc_string(binary) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_jsonc_string(code) do
    format_string(:jsonc, code)
  end

  @spec format_string(binary, atom) :: {:ok, binary} | {:error, BiomeJS.Exception}
  def format_string(type, code) when type in @file_types do
    random_string =
      16
      |> :rand.bytes()
      |> Base.encode32(case: :lower, padding: false)

    BiomeJS.Native.format_string(random_string, type, code)
  end

  def format_string(_, code) do
    format_string(:other, code)
  end
end
