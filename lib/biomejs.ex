defmodule BiomeJS do
  defdelegate format(path), to: BiomeJS.Native
end
