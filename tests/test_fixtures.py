import json
from pathlib import Path

from markdown_it_pyrs import MarkdownIt
import pytest

FIXTURE_PATH = Path(__file__).parent.joinpath("fixtures")


@pytest.mark.parametrize(
    "entry", json.loads(FIXTURE_PATH.joinpath("commonmark_spec.json").read_text())
)
def test_cmark_spec(entry):
    md = MarkdownIt("commonmark")
    output = md.render(entry["markdown"])
    expected = entry["html"]

    assert output == expected


@pytest.mark.param_file(FIXTURE_PATH.joinpath("commonmark_extras.md"))
def test_commonmark_extras(file_params):
    if file_params.title in (
        "Don't output empty class here:",
        "Tabs should not cause hardbreak, EOL tabs aren't stripped in commonmark 0.27",
        "Newline in image description",
    ):
        pytest.skip("known issue")
    md = MarkdownIt()
    md = MarkdownIt("commonmark")
    # md.options["langPrefix"] = ""
    text = md.render(file_params.content).replace("language-", "")
    assert file_params.assert_expected(text, rstrip=True)


# @pytest.mark.param_file(FIXTURE_PATH.joinpath("linkify.md"))
# def test_linkify(file_params):
#     md = MarkdownIt().enable("linkify")
#     assert file_params.assert_expected(md.render(file_params.content), rstrip=True)


@pytest.mark.param_file(FIXTURE_PATH.joinpath("smartquotes.md"))
def test_smartquotes(file_params):
    md = MarkdownIt().enable("replacements").enable("smartquotes")
    assert file_params.assert_expected(md.render(file_params.content), rstrip=True)


@pytest.mark.param_file(FIXTURE_PATH.joinpath("typographer.md"))
def test_typographer(file_params):
    md = MarkdownIt().enable("replacements")
    assert file_params.assert_expected(md.render(file_params.content), rstrip=True)


@pytest.mark.param_file(FIXTURE_PATH.joinpath("tables.md"))
def test_table(file_params):
    md = MarkdownIt().enable("table")
    assert file_params.assert_expected(md.render(file_params.content), rstrip=True)


@pytest.mark.param_file(FIXTURE_PATH.joinpath("normalize.md"))
def test_normalize_url(file_params):
    if file_params.title in (
        "Keep %25 as is because decoding it may break urls, #720",
        "Encode link destination, decode text inside it",
    ):
        pytest.skip("known issue")
    md = MarkdownIt()
    assert file_params.assert_expected(md.render(file_params.content), rstrip=True)


@pytest.mark.param_file(FIXTURE_PATH.joinpath("strikethrough.md"))
def test_strikethrough(file_params):
    md = MarkdownIt().enable("strikethrough")
    assert file_params.assert_expected(md.render(file_params.content), rstrip=True)