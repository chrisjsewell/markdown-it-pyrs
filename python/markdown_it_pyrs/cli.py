"""A simple CLI for the package."""
from __future__ import annotations

import argparse
import sys

from . import MarkdownIt, __version__


def main(args: None | list[str] = None) -> None:
    """Run the CLI."""
    parser = argparse.ArgumentParser(
        description="Parse Markdown using markdown-it-pyrs"
    )
    parser.add_argument(
        "--version",
        "-v",
        action="version",
        version=f"%(prog)s {__version__}",
    )
    # print available plugins and exit
    parser.add_argument(
        "--list-plugins",
        "-lp",
        action=ListPlugins,
        nargs=0,
        help="Print available plugins and exit",
    )

    subparsers = parser.add_subparsers(title="Commands", metavar="", dest="subcommand")

    parser_html = subparsers.add_parser("html", help="Render to HTML")
    add_shared_args(parser_html)

    parser_ast = subparsers.add_parser("ast", help="Render to Abstract Syntax Tree")
    add_shared_args(parser_ast)
    parser_ast.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Print node metadata",
    )

    parsed_args = parser.parse_args(args)
    md = MarkdownIt(parsed_args.config)
    if parsed_args.enable:
        md.enable_many(parsed_args.enable.split(","))
    if parsed_args.subcommand == "html":
        print(md.render(parsed_args.file.read()))
    if parsed_args.subcommand == "ast":
        print(
            md.tree(parsed_args.file.read()).pretty(
                attrs=True, meta=parsed_args.verbose
            )
        )


class ListPlugins(argparse.Action):
    def __call__(self, parser, namespace, values, option_string=None):
        print("\n".join(MarkdownIt.list_plugins()))
        sys.exit(0)


def add_shared_args(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "file",
        type=argparse.FileType("r", encoding="utf8"),
        help="File to read or '-' for stdin",
    )
    parser.add_argument(
        "--config",
        "-c",
        type=str,
        choices=["commonmark", "gfm", "zero"],
        default="commonmark",
        help="Configuration preset name (default: commonmark)",
    )
    parser.add_argument(
        "--enable",
        "-e",
        type=str,
        default="",
        help="Comma-delimited list of plugin names to enable",
    )


if __name__ == "__main__":
    main()
