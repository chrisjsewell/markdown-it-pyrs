"""A Python interface for markdown-it.rs, using Rust for blazingly fast Markdown parsing ⚡️"""
from .markdown_it_pyrs import *  # noqa: F403

__all__ = ("MarkdownIt", "Node", "__version__")  # noqa: F405
