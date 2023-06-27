from typing import Any, Iterable, List, Literal, Mapping, Optional, Sequence, Tuple

__version__: str

class Node:
    """Single node in the Markdown AST tree."""

    _rust_path: Optional[str]
    """The rust module path of the node type"""

    name: str
    """The name of the node type"""

    children: Sequence["Node"]
    """The children of the node"""

    srcmap: Optional[Tuple[int, int]]
    """Byte offset mapping of the (start, end) of the source syntax."""

    attrs: Mapping[str, str]
    """Additional attributes to be added to resulting html."""

    meta: Mapping[str, Any]
    """Custom data specific to the node type."""

    def __init__(self, name: str) -> None:
        """Initialize a Node instance.

        :param name: The type of the node.
        """
    def __repr__(self) -> str:
        """Return a string representation of the node."""
    def __str__(self) -> str:
        """Return a string representation of the node."""
    def walk(self, *, include_self: bool = True) -> Iterable["Node"]:
        """Recursively yield all descendant nodes in the tree.

        The order mimics the order of the underlying linear token
        stream (i.e. depth first).

        :param include_self: whether to include self in the output
        """
    def pretty(
        self,
        *,
        attrs: bool = False,
        srcmap: bool = False,
        content: bool = False,
        meta: bool = False,
        recurse: bool = True,
        indent: int = 2,
        indent_current: int = 0,
    ) -> str:
        """Return a pretty string representation of the node.

        :param attrs: whether to include attributes in the output
        :param srcmap: whether to include source map in the output
        :param content: whether to include content meta field in the output
        :param meta: whether to include meta fields in the output
        :param recurse: whether to recurse into child nodes
        :param indent: number of spaces to increase indent for each level
        :param indent_current: number of spaces to indent the current level
        """

_PLUGIN_NAME = Literal[
    "blockquote",
    "code",
    "fence",
    "heading",
    "hr",
    "lheading",
    "list",
    "paragraph",
    "reference",
    "autolink",
    "backticks",
    "emphasis",
    "entity",
    "escape",
    "image",
    "link",
    "newline",
    "html_block",
    "html_inline",
    "linkify",
    "replacements",
    "smartquotes",
    "sourcepos",
    "strikethrough",
    "table",
    "front_matter",
    "tasklist",
    "footnote",
    "heading_anchors",
    "autolink_ext",
]

class MarkdownIt:
    """Markdown parser class."""

    def __init__(
        self, config: Literal["commonmark", "gfm", "zero"] = "commonmark"
    ) -> None:
        """Initialize a MarkdownIt instance.

        :param config: Configuration preset name.
        """
    @staticmethod
    def list_plugins() -> List[_PLUGIN_NAME]:
        """List available plugins."""
    def enable(
        self,
        name: _PLUGIN_NAME,
    ) -> "MarkdownIt":
        """Enable a plugin rule.

        :param name: Plugin name.
        """
    def enable_many(
        self,
        names: List[_PLUGIN_NAME],
    ) -> "MarkdownIt":
        """Enable multiple plugin rules.

        :param names: Plugin names.
        """
    def render(self, src: str) -> str:
        """Render Markdown to HTML.

        :param src: Markdown source.
        :returns: HTML.
        """
    def tree(self, src: str) -> Node:
        """Create a syntax tree from the Markdown source.

        :param src: Markdown source.
        """
