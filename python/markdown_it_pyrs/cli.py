"""A simple CLI for the package."""
from __future__ import annotations

import argparse
import sys

from . import MarkdownIt, __version__


class ListPlugins(argparse.Action):
    def __call__(self, parser, namespace, values, option_string=None):
        print("\n".join(MarkdownIt.list_plugins()))
        sys.exit(0)


def main(args: None | list[str] = None) -> None:
    """Run the CLI."""
    parser = argparse.ArgumentParser(description="Parse Markdown using markdown-it.rs.")
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
    # comma-delimited string of plugin names to enable
    parser.add_argument(
        "--enable",
        "-e",
        type=str,
        default="",
        help="Comma-delimited list of plugin names to enable",
    )
    parsed_args = parser.parse_args(args)
    md = MarkdownIt(parsed_args.config)
    if parsed_args.enable:
        md.enable_many(parsed_args.enable.split(","))
    print(md.render(parsed_args.file.read()))


if __name__ == "__main__":
    main()
