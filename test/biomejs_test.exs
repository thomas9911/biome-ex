defmodule BiomeJSTest do
  use ExUnit.Case, async: true
  doctest BiomeJS

  test "format js string" do
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

  test "format ts string" do
    assert {:ok, text} =
             BiomeJS.format_ts_string("""
             const a: number
             = 1;

             const b =():void => {



               console.log(
                 1
               )
             }

             """)

    assert """
           const a: number = 1;

           const b = (): void => {
           \tconsole.log(1);
           };
           """ == text
  end

  test "format json string" do
    assert {:ok, text} = BiomeJS.format_json_string(~s|{"test":1,"other":2}|)
    assert ~s|{ "test": 1, "other": 2 }\n| == text
  end

  test "format invalid json string does not change the input" do
    input = ~s|{"test":1,"other|
    assert {:ok, text} = BiomeJS.format_json_string(input)
    assert "#{input}\n" == text
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

  @tag :tmp_dir
  test "format file errors on file not found", %{tmp_dir: tmp_dir} do
    test_file = Path.join(tmp_dir, "not_found.ts")

    assert {:error, %BiomeJS.Exception{message: "entity not found"}} = BiomeJS.format(test_file)
  end
end
