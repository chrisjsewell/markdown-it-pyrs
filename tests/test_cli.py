from markdown_it_pyrs import __version__
from markdown_it_pyrs.cli import main
import pytest


def test_version(capsys):
    with pytest.raises(SystemExit):
        main(["--version"])
    captured = capsys.readouterr()
    assert captured.out.strip().endswith(__version__)


def test_list_plugins(capsys):
    with pytest.raises(SystemExit):
        main(["--list-plugins"])
    captured = capsys.readouterr()
    assert "paragraph" in captured.out.strip()


def test_html(capsys, tmp_path):
    tmp_file = tmp_path / "test.md"
    tmp_file.write_text("foo", encoding="utf8")
    main(["html", str(tmp_file)])
    captured = capsys.readouterr()
    assert captured.out.strip() == "<p>foo</p>"


def test_html_config(capsys, tmp_path):
    tmp_file = tmp_path / "test.md"
    tmp_file.write_text("foo", encoding="utf8")
    main(["html", "-c", "zero", str(tmp_file)])
    captured = capsys.readouterr()
    assert captured.out.strip() == "foo"


def test_html_plugin(capsys, tmp_path):
    tmp_file = tmp_path / "test.md"
    tmp_file.write_text("foo", encoding="utf8")
    main(["html", "-e", "sourcepos", str(tmp_file)])
    captured = capsys.readouterr()
    assert captured.out.strip() == '<p data-sourcepos="1:1-1:3">foo</p>'


def test_ast(capsys, tmp_path):
    tmp_file = tmp_path / "test.md"
    tmp_file.write_text("foo", encoding="utf8")
    main(["ast", str(tmp_file)])
    captured = capsys.readouterr()
    assert captured.out.strip() == "<root>\n  <paragraph>\n    <text>"


def test_ast_config(capsys, tmp_path):
    tmp_file = tmp_path / "test.md"
    tmp_file.write_text("foo", encoding="utf8")
    main(["ast", "-c", "zero", str(tmp_file)])
    captured = capsys.readouterr()
    assert captured.out.strip() == "<root>\n  <text>"


def test_ast_plugin(capsys, tmp_path):
    tmp_file = tmp_path / "test.md"
    tmp_file.write_text("foo", encoding="utf8")
    main(["ast", "-e", "sourcepos", str(tmp_file)])
    captured = capsys.readouterr()
    assert captured.out.strip() == (
        '<root data-sourcepos="1:1-1:3">\n'
        '  <paragraph data-sourcepos="1:1-1:3">\n'
        '    <text data-sourcepos="1:1-1:3">'
    )


def test_ast_verbose(capsys, tmp_path):
    tmp_file = tmp_path / "test.md"
    tmp_file.write_text("foo", encoding="utf8")
    main(["ast", "-v", str(tmp_file)])
    captured = capsys.readouterr()
    assert (
        captured.out.strip() == "<root>\n  <paragraph>\n    <text>\n      content: foo"
    )
