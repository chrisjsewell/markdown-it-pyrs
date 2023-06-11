from typing import Any, Dict, List, Literal, Optional, Tuple

__version__: str

class Node:
    """Single node in the Markdown AST tree."""

    module: str
    """The rust module path of the node type"""

    name: str
    """The name of the node type"""

    children: List["Node"]
    """The children of the node"""

    srcmap: Optional[Tuple[int, int]]
    """Byte offset mapping of the (start, end) of the source syntax."""

    attrs: Dict[str, str]
    """Additional attributes to be added to resulting html."""

    meta: Dict[str, Any]
    """Custom data specific to the node type."""

    def __init__(self, path: str) -> None:
        """Initialize a Node instance.

        :param path: The rust module path of the node type.
        """
    def __repr__(self) -> str:
        """Return a string representation of the node."""
    def __str__(self) -> str:
        """Return a string representation of the node."""
    def pretty(
        self,
        *,
        attrs=False,
        srcmap=False,
        content=False,
        meta=False,
        recurse=True,
        indent=2,
        indent_current=0,
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

class MarkdownIt:
    """Markdown parser class."""

    def __init__(self, config: Literal["commonmark", "zero"] = "commonmark") -> None:
        """Initialize a MarkdownIt instance.

        :param config: Configuration preset name.
        """
    def enable(
        self,
        name: Literal[
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
            "strikethrough",
            "table",
        ],
    ) -> "MarkdownIt":
        """Enable a rule.

        :param name: Rule name.
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
