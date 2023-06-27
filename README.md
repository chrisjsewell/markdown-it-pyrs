# markdown-it-pyrs

[![PyPI][pypi-badge]][pypi-link]

[pypi-badge]: https://img.shields.io/pypi/v/markdown-it-pyrs.svg
[pypi-link]: https://pypi.org/project/markdown-it-pyrs

**Currently in Beta, feedback welcome!**

A Python interface for [markdown-it.rs](https://github.com/rlidwka/markdown-it.rs) (and [plugins](https://github.com/chrisjsewell/markdown-it-plugins.rs)), using Rust for blazingly fast Markdown parsing ⚡️

The goal of this project is to provide a fast, safe, extensible, and easy-to-use Markdown parser for Python.
It is complimentary to [markdown-it-py](https://github.com/ExecutableBookProject/markdown-it-py), which is a pure Python implementation of markdown-it, and here we aim to follow as close as possible the API for that package.

If you care primarily about speed, this is the library for you.
For example, benchmarking the two libraries when parsing the CommonMark Spec file, markdown-it-pyrs is **20x faster than markdown-it-py**.

Name (time, ms)  |   Min   |   Max   |  Mean   | Rounds
---------------- | ------- | ------- | ------- | ------
markdown-it-pyrs | 5.217   | 7.969   | 5.968   | 85
markdown-it-py   | 122.696 | 143.246 | 131.431 | 7

The drawback is that the library vendors compiled Rust code, and so:

1. Parser plugins cannot currently be written in Python and added dynamically to the parser.
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
md = (
  MarkdownIt("commonmark")
  .enable("table")
  .enable_many(["linkify", "strikethrough"])
)
node = md.tree("# Hello, world!")
print(node.walk())
# [Node(root), Node(heading), Node(text)]
print(node.pretty(srcmap=True, meta=True))
# <root srcmap="0:15">
#   <heading srcmap="0:15">
#     level: 1
#     <text srcmap="2:15">
#       content: Hello, world!
```

**Note:** Attributes of the `Node` class, such as `Node.attrs`, return a **copy** of the underlying data, and so mutating it will not affect what is stored on the node, e.g.

```python
from markdown_it_pyrs import Node
node = Node("name")
# don't do this!
node.attrs["key"] = "value"
print(node.attrs) # {}
# do this instead (Python 3.9+)
node.attrs = node.attrs | {"key": "value"}
print(node.attrs) # {"key": "value"}
# Node.children is only a shallow copy though, so this is fine
child = Node("child")
node.children = [child]
node.children[0].name = "other"
print(child.name) # "other"
```

### Command Line Interface

A CLI is also provided, which can be used like this:

```bash
echo "# Hello, world!" | markdown-it-pyrs html -
# <h1>Hello, world!</h1>
echo "# Hello, world!" | markdown-it-pyrs ast -
# <root>
#   <heading>
#     <text>
```

Replace `-` with a filename to read from a file,
and see `markdown-it-pyrs --help` for more options,
including initial configuration and enabling plugins.

## Initial Configuration

Initialising `MarkdownIt("zero")` will not enable any plugins, and so you can add only the ones you need.

Use `MarkdownIt("commonmark")` to enable all the CommonMark plugins.

Use `MarkdownIt("gfm")` to enable all the CommonMark plugins, plus the GitHub Flavoured Markdown plugins.

## Plugins

All syntax rules in `markdown-it.rs` are implemented as plugins.
Plugins can be added to the parser by calling `enable` or `enable_many` with the name of the plugin.
The following plugins are currently supported:

CommonMark Blocks:

- `blockquote`: Block quotes with `>`
- `code`: Indented code blocks
- `fence`: Backtick code blocks
- `heading`: `#` ATX headings
- `hr`: `---` horizontal rules
- `lheading`: `---` underline setext headings
- `list`: `*` unordered lists and `1.` ordered lists
- `paragraph`: Paragraphs
- `reference`: Link reference definitions `[id]: src "title"`

CommonMark Inlines:

- `autolink`: `<http://example.com>`
- `backticks`: `` `code` ``
- `emphasis`: `_emphasis_`, `*emphasis*`, `**strong**`, `__strong__`
- `entity`: `&amp;`
- `escape`: backslash escaping `\`
- `image`: `![alt](src "title")`
- `link`: `[text](src "title")`, `[text][id]`, `[text]`
- `newline`: hard line breaks
- `html_block`: HTML blocks
- `html_inline`: HTML inline

GitHub Flavoured Markdown (<https://github.github.com/gfm>):

- `table`:

  ```markdown
  | foo | bar |
  | --- | --- |
  | baz | bim |
  ```
- `strikethrough`: `~~strikethrough~~`
- `tasklist`: `- [x] tasklist item`
- `autolink_ext`: Extended autolink detection with "bare URLs" like `https://example.com` and `www.example.com`

Others:

- `sourcepos`: Add source mapping to rendered HTML, looks like this: `<stuff data-sourcepos="1:1-2:3">`, i.e. `line:col-line:col`
- `replacements`: Typographic replacements, like `--` to `—`
- `smartquotes`: Smart quotes, like `"` to `“`
- `front_matter`: YAML front matter
- `footnote`: Pandoc-style footnotes (see <https://pandoc.org/MANUAL.html#footnotes>)
- `heading_anchors`: Add heading anchors, with defaults like GitHub
- `linkify`: Automatically linkify URLs with <https://crates.io/crates/linkify> (note currently this only matches URLs with a scheme, e.g. `https://example.com`)

## Development

I'm quite new to Rust, so if you see something that could be improved, issues and PRs are welcome!

[PyO3](https://pyo3.rs) and [Maturin](https://www.maturin.rs) are used to build the Python package, by wrapping [markdown-it.rs](https://github.com/rlidwka/markdown-it.rs) in a Python module.

[pre-commit](https://pre-commit.com) is used to run code formatting and linting checks, and [tox](https://tox.readthedocs.io) is used to run tests.

### TODO

Improvements:

- Allow to override options:
  - xhtml_out: Use `"/"` to close single tags (e.g. `<br />`)
  - lang_prefix: Prefix for language classes on fenced code blocks
  - quotes: Quote characters, for smart quotes

- Add plugins: ...

- Allow options for plugins:
  - heading anchors
  - tasklist checkboxes to be disabled
  - footnotes with options to turn on/off inline/collect/backrefs

- The `gfm` (Github Flavoured Markdown) initialisation mode needs improving
  - Add <https://github.github.com/gfm/#disallowed-raw-html-extension->
  - heading anchors, is not strictly in the spec, but should be noted
  - Add more testing

Open issue upstream:

- no `text_join` rule (to join adjacent `text` and `text_special` tokens)
- `heading_anchors` plugin does not allow for e.g. GitHub format where non-uniqueness is resolved by appending `-1`, `-2`, etc, and also removal of image text
- Capture reference nodes
- Capture link reference definitions
- Turn off code rule (and thus remove indent limit)
- disable rules
- better "cross-language" AST representation
- differing behaviour of linkify and normalize_url/commonmark_extras test failures
- quote characters for smart-quotes and lang_prefix for fence
  should both be variable at run-time? (currently they both must be compiled)
- fix docstring in `examples/ferris/block_rule.rs::FerrisBlockScanner::run`,
  which currently describes the JS API not the new rust one
- also some functions/methods use `//` not `///` for docstrings
- Capture "piece-wise" source maps for nested content, e.g. for when the source is split over multiple lines and nested in another block (could get inline here <https://github.com/rlidwka/markdown-it.rs/blob/6f906b38c8ffc3cc651e67b448b3655b7d0debb3/src/parser/inline/mod.rs#L115>)
- easier way to get `root.ext` items in core rules; it seems at present you have to swap memory and reswap at the end of the rule, see e.g. the `InlineParserRule`
- allow `test_rules_at_line` to parse what the calling rule is, so that other rules can decide whether to interrupt based on the calling rule (in the `check` function), I think this would then allow behaviour similar to what `alt` did (possibly needed for footnote definition parsing)
  - In general though, where back-compatibility is not required, I agree with [djot](https://github.com/jgm/djot) goal 7, i.e. that block elements should not be allowed to interrupt other block elements without a newline
- The possibility to return multiple (sequential) nodes from an `InlineRule.run`, e.g. `((node1, length1), (node2, length2), ...)`
  - This would be similar to docutils
- In the `Node` walk methods, allow the function to return something to indicate whether to continue walking the children of the node
  - Also in walk, parse the "path" to get to the current node, e.g. list of parent nodes, to allow for backtracing
  - similar to <https://github.com/syntax-tree/unist-util-visit-parents>, <https://github.com/syntax-tree/unist-util-visit>, etc
- is there a way to use a `match` statement, to match a Node against a `NodeValue` implementation? (rather than if/else for `Node.cast`)
- Rule priority as an integer (similar to RST transform priority)
  - Currently can only specify `before` or `after` another rule or all rules
  - Can feel a little unclear for plugins, when to use attrs and when to add fields to node value.

Maintenance:
