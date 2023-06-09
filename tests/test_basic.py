from markdown_it_pyrs import MarkdownIt


def test_basic():
    mdit = MarkdownIt()
    assert mdit.render("# markdown-it rulezz!") == "<h1>markdown-it rulezz!</h1>\n"
