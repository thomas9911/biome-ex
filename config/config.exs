import Config

if Mix.env() == :test do
  config :biomejs, mode: :debug, force_build: true
end
