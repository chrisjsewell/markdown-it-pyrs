from markdown_it_pyrs import MarkdownIt, Node
import pytest


def test_enable() -> None:
    mdit = MarkdownIt()
    with pytest.raises(TypeError):
        mdit.enable(1)  # type: ignore[arg-type]
    with pytest.raises(ValueError):
        mdit.enable("unknown")  # type: ignore[arg-type]
    mdit.enable("heading")
    with pytest.raises(ValueError):
        mdit.enable_many(["heading", "unknown"])  # type: ignore[list-item]
    with pytest.raises(TypeError):
        mdit.enable_many(1)  # type: ignore[arg-type]
    mdit.enable_many(["heading", "code"])


def test_zero() -> None:
    mdit = MarkdownIt("zero")
    assert mdit.render("# markdown-it rulezz!") == "# markdown-it rulezz!\n"


def test_zero_header() -> None:
    mdit = MarkdownIt("zero").enable("heading")
    assert mdit.render("# markdown-it rulezz!") == "<h1>markdown-it rulezz!</h1>\n"


def test_gfm() -> None:
    mdit = MarkdownIt("gfm")
    assert mdit.render("# markdown-it rulezz!") == "<h1>markdown-it rulezz!</h1>\n"


def test_node() -> None:
    node = Node("root")
    assert node.name == "root"
    assert node.children == []
    assert node.srcmap is None
    assert node.attrs == {}
    assert node.meta == {}

    node.attrs = {"foo": "bar"}
    assert node.attrs == {"foo": "bar"}

    child = Node("child")
    node.children = [child]
    assert node.children == [child]
    node.children[0].name = "other"
    assert child.name == "other"


def test_tree_walk() -> None:
    mdit = MarkdownIt()
    assert [str(n) for n in mdit.tree("- a *b*").walk()] == [
        "Node(root)",
        "Node(bullet_list)",
        "Node(list_item)",
        "Node(text)",
        "Node(em)",
        "Node(text)",
    ]


def test_tree_pretty() -> None:
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
