defmodule BiomeJSTest do
  use ExUnit.Case
  doctest BiomeJS

  test "format js" do
    assert {:ok, text} =
             BiomeJS.format_js_string("""
             const a = 1;

             const b = () => {



               console.log(
                 1
               )
             }

             """)

    assert """
           const a = 1;

           const b = () => {
           \tconsole.log(1);
           };
           """ == text
  end
end
