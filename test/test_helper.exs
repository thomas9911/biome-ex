defmodule BiomeJSTest.WindowsFix do
  def sigil_PLAIN(text, _),
    do: String.replace(:elixir_interpolation.unescape_string(text), "\r\n", "\n")
end

ExUnit.start()
