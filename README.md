# markdown-it-pyrs

**Currently in Beta, feedback welcome!**

A Python interface for [markdown-it.rs](https://github.com/rlidwka/markdown-it.rs), using Rust for blazingly fast Markdown parsing ⚡️

The goal of this project is to provide a fast, safe, extensible, and easy-to-use Markdown parser for Python.
It is complimentary to [markdown-it-py](https://github.com/ExecutableBookProject/markdown-it-py), which is a pure Python implementation of markdown-it, and here we aim to follow as close as possible the API for that package.

If you care primarily about speed, this is the library for you.
For example, benchmarking the two libraries when parsing the CommonMark Spec file, markdown-it-pyrs is **20x faster than markdown-it-py**.

Name (time, ms)  |   Min   |   Max   |  Mean   | Rounds
---------------- | ------- | ------- | ------- | ------
markdown-it-pyrs | 5.217   | 7.969   | 5.968   | 85
markdown-it-py   | 122.696 | 143.246 | 131.431 | 7

The drawback is that the library vendors compiled Rust code, and so:

1. Parser plugins cannot be written in Python and added dynamically to the parser.
2. It can be more difficult to integrate into environments like [pyiodide](https://pyodide.org) and py-script (but maybe not for long: <https://discuss.python.org/t/support-wasm-wheels-on-pypi/21924/3>).

## Usage

First install the package:

```bash
pip install markdown-it-pyrs
```

Then use it like you would `markdown-it-py`:

```python
from markdown_it_pyrs import MarkdownIt

md = MarkdownIt("commonmark").enable("table")
md.render("# Hello, world!")
# '<h1>Hello, world!</h1>\n'
```

`markdown-it.rs` does not generate a token stream, but instead directly generates a `Node` tree.
This is similar to the `markdown-it-py`'s `SyntaxTreeNode` class, although the API is not identical.
(source mapping is also provided by byte-offset, rather than line only)

```python
md = MarkdownIt("commonmark").enable("table")
print(
    md.tree("# Hello, world!")
      .pretty(srcmap=True, meta=True)
)
# <root srcmap="0:15">
#   <heading srcmap="0:15">
#     level: 1
#     <text srcmap="2:15">
#       content: Hello, world!
```

## Development

I'm quite new to Rust, so if you see something that could be improved, please open an issue or PR!

[PyO3](https://pyo3.rs) and [Maturin](https://www.maturin.rs) are used to build the Python package, by wrapping [markdown-it.rs](https://github.com/rlidwka/markdown-it.rs) in a Python module.

[pre-commit](https://pre-commit.com) is used to run code formatting and linting checks, and [tox](https://tox.readthedocs.io) is used to run tests.

## TODO

Improvements:

- Allow to override options:
  - xhtml_out: Use `"/"` to close single tags (e.g. `<br />`)
  - lang_prefix: Prefix for language classes on fenced code blocks
  - quotes: Quote characters, for smart quotes

- Add plugins (and way to initialise them):
  - footnotes

Open issue upstream:

- no `text_join` rule (to join adjacent `text` and `text_special` tokens)
- Capture reference nodes
- Capture link reference definitions
- Turn off code rule
- better "cross-language" AST representation
- differing behaviour of linkify and normalize_url/commonmark_extras test failures

Maintenance:

- Get `maturin develop` to run on tox calls
