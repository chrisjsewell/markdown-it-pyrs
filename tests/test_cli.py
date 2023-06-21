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


def test_render(capsys, tmp_path):
    tmp_file = tmp_path / "test.md"
    tmp_file.write_text("foo", encoding="utf8")
    main([str(tmp_file)])
    captured = capsys.readouterr()
    assert captured.out.strip() == "<p>foo</p>"


def test_render_config(capsys, tmp_path):
    tmp_file = tmp_path / "test.md"
    tmp_file.write_text("foo", encoding="utf8")
    main(["-c", "zero", str(tmp_file)])
    captured = capsys.readouterr()
    assert captured.out.strip() == "foo"


def test_render_plugin(capsys, tmp_path):
    tmp_file = tmp_path / "test.md"
    tmp_file.write_text("foo", encoding="utf8")
    main(["-e", "sourcepos", str(tmp_file)])
    captured = capsys.readouterr()
    assert captured.out.strip() == '<p data-sourcepos="1:1-1:3">foo</p>'
