from markdown_it_pyrs import MarkdownIt
import pytest


def test_enable_unknown():
    mdit = MarkdownIt()
    with pytest.raises(ValueError):
        mdit.enable("unknown")


def test_zero():
    mdit = MarkdownIt("zero")
    assert mdit.render("# markdown-it rulezz!") == "# markdown-it rulezz!\n"


def test_zero_header():
    mdit = MarkdownIt("zero").enable("heading")
    assert mdit.render("# markdown-it rulezz!") == "<h1>markdown-it rulezz!</h1>\n"


def test_tree():
    mdit = MarkdownIt()
    assert (
        mdit.tree("# markdown-it rulezz!").pretty(srcmap=True, meta=True)
        == """\
<root srcmap="0:21">
  <heading srcmap="0:21">
    level: 1
    <text srcmap="2:21">
      content: markdown-it rulezz!
"""
    )
