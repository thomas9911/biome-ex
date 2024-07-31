defmodule BiomeJS.MixProject do
  use Mix.Project

  @version "0.2.3"

  def project do
    [
      app: :biomejs,
      version: @version,
      elixir: "~> 1.12",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      aliases: aliases(),
      package: package()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp package do
    [
      licenses: ["MIT"],
      links: %{"Github" => "https://github.com/thomas9911/biome-ex"}
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.30"},
      {:rustler_precompiled, "~> 0.7"}
    ]
  end

  defp aliases do
    [
      format: ["format", "cmd --cd ./native/biomejs_native cargo fmt"]
    ]
  end
end
