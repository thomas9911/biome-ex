defmodule BiomeJSTest do
  use ExUnit.Case, async: true
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

  @tag :tmp_dir
  test "format file js", %{tmp_dir: tmp_dir} do
    test_file = Path.join(tmp_dir, "javascript.js")

    File.write!(test_file, """
    const a = 1;

    const b = () => {



      console.log(
        1
      )
    }

    """)

    assert {:ok, :formatted} = BiomeJS.format(test_file)

    assert """
           const a = 1;

           const b = () => {
           \tconsole.log(1);
           };
           """ == File.read!(test_file)
  end

  @tag :tmp_dir
  test "already formatted file js", %{tmp_dir: tmp_dir} do
    test_file = Path.join(tmp_dir, "javascript.js")

    File.write!(test_file, """
    const a = 1;

    const b = () => {
    \tconsole.log(1);
    };
    """)

    assert {:ok, :unchanged} = BiomeJS.format(test_file)
  end

  @tag :tmp_dir
  test "format file ts", %{tmp_dir: tmp_dir} do
    test_file = Path.join(tmp_dir, "typescript.ts")

    File.write!(test_file, """
    const a: number
    = 1;

    const b =():void => {



      console.log(
        1
      )
    }

    """)

    assert {:ok, :formatted} = BiomeJS.format(test_file)

    assert """
           const a: number = 1;

           const b = (): void => {
           \tconsole.log(1);
           };
           """ == File.read!(test_file)
  end
end
