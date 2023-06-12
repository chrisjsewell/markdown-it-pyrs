use pyo3::prelude::*;

mod nodes;

/// Main parser class
#[pyclass]
struct MarkdownIt {
    parser: markdown_it::MarkdownIt,

    xhtml_out: bool,
}

#[pymethods]
impl MarkdownIt {
    #[new]
    #[pyo3(signature = (config="commonmark"))]
    fn new(config: &str) -> PyResult<Self> {
        match config {
            "commonmark" => {
                let mut parser = markdown_it::MarkdownIt::new();
                markdown_it::plugins::cmark::add(&mut parser);
                markdown_it::plugins::html::add(&mut parser);
                Ok(MarkdownIt {
                    parser,
                    xhtml_out: true,
                })
            }
            "zero" => Ok(MarkdownIt {
                parser: markdown_it::MarkdownIt::new(),
                xhtml_out: false,
            }),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown config: {}",
                config
            ))),
        }
    }

    /// Enable a plugin
    fn enable(slf: Py<Self>, py: Python, name: &str) -> PyResult<Py<Self>> {
        {
            let mut slf_mut = slf.borrow_mut(py);
            match name {
                "blockquote" => {
                    markdown_it::plugins::cmark::block::blockquote::add(&mut slf_mut.parser);
                }
                "code" => {
                    markdown_it::plugins::cmark::block::code::add(&mut slf_mut.parser);
                }
                "fence" => {
                    markdown_it::plugins::cmark::block::fence::add(&mut slf_mut.parser);
                }
                "heading" => {
                    markdown_it::plugins::cmark::block::heading::add(&mut slf_mut.parser);
                }
                "hr" => {
                    markdown_it::plugins::cmark::block::hr::add(&mut slf_mut.parser);
                }
                "lheading" => {
                    markdown_it::plugins::cmark::block::lheading::add(&mut slf_mut.parser);
                }
                "list" => {
                    markdown_it::plugins::cmark::block::list::add(&mut slf_mut.parser);
                }
                "paragraph" => {
                    markdown_it::plugins::cmark::block::paragraph::add(&mut slf_mut.parser);
                }
                "reference" => {
                    markdown_it::plugins::cmark::block::reference::add(&mut slf_mut.parser);
                }
                "autolink" => {
                    markdown_it::plugins::cmark::inline::autolink::add(&mut slf_mut.parser);
                }
                "backticks" => {
                    markdown_it::plugins::cmark::inline::backticks::add(&mut slf_mut.parser);
                }
                "emphasis" => {
                    markdown_it::plugins::cmark::inline::emphasis::add(&mut slf_mut.parser);
                }
                "entity" => {
                    markdown_it::plugins::cmark::inline::entity::add(&mut slf_mut.parser);
                }
                "escape" => {
                    markdown_it::plugins::cmark::inline::escape::add(&mut slf_mut.parser);
                }
                "image" => {
                    markdown_it::plugins::cmark::inline::image::add(&mut slf_mut.parser);
                }
                "link" => {
                    markdown_it::plugins::cmark::inline::link::add(&mut slf_mut.parser);
                }
                "newline" => {
                    markdown_it::plugins::cmark::inline::newline::add(&mut slf_mut.parser);
                }
                "html_block" => {
                    markdown_it::plugins::html::html_block::add(&mut slf_mut.parser);
                }
                "html_inline" => {
                    markdown_it::plugins::html::html_inline::add(&mut slf_mut.parser);
                }
                "linkify" => {
                    markdown_it::plugins::extra::linkify::add(&mut slf_mut.parser);
                }
                "replacements" => {
                    markdown_it::plugins::extra::typographer::add(&mut slf_mut.parser);
                }
                "smartquotes" => {
                    markdown_it::plugins::extra::smartquotes::add(&mut slf_mut.parser);
                }
                "strikethrough" => {
                    markdown_it::plugins::extra::strikethrough::add(&mut slf_mut.parser);
                }
                "table" => {
                    markdown_it::plugins::extra::tables::add(&mut slf_mut.parser);
                }
                _ => {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "Unknown rule: {}",
                        name
                    )))
                }
            }
        }
        Ok(slf)
    }

    /// Render markdown string into HTML.
    fn render(&self, src: &str) -> String {
        let ast = self.parser.parse(src);
        match self.xhtml_out {
            true => {
                return ast.xrender();
            }
            false => {
                return ast.render();
            }
        }
    }

    /// Create a syntax tree from the markdown string.
    fn tree(&self, py: Python, src: &str) -> nodes::Node {
        let ast = self.parser.parse(src);

        fn walk_recursive<'a>(py: Python, node: &'a markdown_it::Node, py_node: &mut nodes::Node) {
            for n in node.children.iter() {
                let mut py_node_child = nodes::create_node(py, &n);

                stacker::maybe_grow(64 * 1024, 1024 * 1024, || {
                    walk_recursive(py, n, &mut py_node_child);
                });

                py_node.children.push(Py::new(py, py_node_child).unwrap());
            }
        }

        let mut py_node = nodes::create_node(py, &ast);
        walk_recursive(py, &ast, &mut py_node);

        py_node
    }
}

/// A Python interface to markdown_it.rs
#[pymodule]
// Note: The name of this function must match the `lib.name` setting in the `Cargo.toml`,
// else Python will not be able to import the module.
fn markdown_it_pyrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<MarkdownIt>()?;
    m.add_class::<nodes::Node>()?;
    Ok(())
}
