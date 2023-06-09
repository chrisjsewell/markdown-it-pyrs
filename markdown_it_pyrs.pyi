from typing import Literal

__version__: str

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
    ) -> None:
        """Enable a rule.

        :param name: Rule name.
        """
    def render(self, src: str) -> str:
        """Render Markdown to HTML.

        :param src: Markdown source.
        :returns: HTML.
        """
