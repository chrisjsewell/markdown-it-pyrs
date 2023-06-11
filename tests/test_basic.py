from markdown_it_pyrs import MarkdownIt


def test_render():
    mdit = MarkdownIt()
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
