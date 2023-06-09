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
