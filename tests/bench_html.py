from pathlib import Path

import pytest


@pytest.fixture
def spec_text():
    return Path(__file__).parent.joinpath("fixtures", "spec.md").read_text()


@pytest.mark.benchmark(group="html")
def test_markdown_it_py(benchmark, spec_text):
    import markdown_it

    parser = markdown_it.MarkdownIt("commonmark")
    benchmark.extra_info["version"] = markdown_it.__version__
    benchmark(parser.render, spec_text)


@pytest.mark.benchmark(group="html")
def test_markdown_it_pyrs(benchmark, spec_text):
    import markdown_it_pyrs

    parser = markdown_it_pyrs.MarkdownIt("commonmark")
    benchmark.extra_info["version"] = markdown_it_pyrs.__version__
    benchmark(parser.render, spec_text)
