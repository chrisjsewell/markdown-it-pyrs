use pyo3::prelude::*;

mod nodes;

/// Main parser class
#[pyclass]
pub struct MarkdownIt {
    parser: markdown_it::MarkdownIt,
    xhtml_out: bool,
}

impl MarkdownIt {
    fn _enable(&mut self, name: &str) -> Result<(), PyErr> {
        match name {
            "blockquote" => {
                markdown_it::plugins::cmark::block::blockquote::add(&mut self.parser);
            }
            "code" => {
                markdown_it::plugins::cmark::block::code::add(&mut self.parser);
            }
            "fence" => {
                markdown_it::plugins::cmark::block::fence::add(&mut self.parser);
            }
            "heading" => {
                markdown_it::plugins::cmark::block::heading::add(&mut self.parser);
            }
            "hr" => {
                markdown_it::plugins::cmark::block::hr::add(&mut self.parser);
            }
            "lheading" => {
                markdown_it::plugins::cmark::block::lheading::add(&mut self.parser);
            }
            "list" => {
                markdown_it::plugins::cmark::block::list::add(&mut self.parser);
            }
            "paragraph" => {
                markdown_it::plugins::cmark::block::paragraph::add(&mut self.parser);
            }
            "reference" => {
                markdown_it::plugins::cmark::block::reference::add(&mut self.parser);
            }
            "autolink" => {
                markdown_it::plugins::cmark::inline::autolink::add(&mut self.parser);
            }
            "backticks" => {
                markdown_it::plugins::cmark::inline::backticks::add(&mut self.parser);
            }
            "emphasis" => {
                markdown_it::plugins::cmark::inline::emphasis::add(&mut self.parser);
            }
            "entity" => {
                markdown_it::plugins::cmark::inline::entity::add(&mut self.parser);
            }
            "escape" => {
                markdown_it::plugins::cmark::inline::escape::add(&mut self.parser);
            }
            "image" => {
                markdown_it::plugins::cmark::inline::image::add(&mut self.parser);
            }
            "link" => {
                markdown_it::plugins::cmark::inline::link::add(&mut self.parser);
            }
            "newline" => {
                markdown_it::plugins::cmark::inline::newline::add(&mut self.parser);
            }
            "html_block" => {
                markdown_it::plugins::html::html_block::add(&mut self.parser);
            }
            "html_inline" => {
                markdown_it::plugins::html::html_inline::add(&mut self.parser);
            }
            "linkify" => {
                markdown_it::plugins::extra::linkify::add(&mut self.parser);
            }
            "replacements" => {
                markdown_it::plugins::extra::typographer::add(&mut self.parser);
            }
            "smartquotes" => {
                markdown_it::plugins::extra::smartquotes::add(&mut self.parser);
            }
            "sourcepos" => {
                markdown_it::plugins::sourcepos::add(&mut self.parser);
            }
            "strikethrough" => {
                markdown_it::plugins::extra::strikethrough::add(&mut self.parser);
            }
            "table" => {
                markdown_it::plugins::extra::tables::add(&mut self.parser);
            }
            "front_matter" => {
                markdown_it_front_matter::add(&mut self.parser);
            }
            "tasklist" => {
                markdown_it_tasklist::add(&mut self.parser);
            }
            "footnote" => {
                markdown_it_footnote::add(&mut self.parser);
            }
            "heading_anchors" => {
                markdown_it_heading_anchors::add(&mut self.parser);
            }
            "autolink_ext" => {
                markdown_it_autolink::add(&mut self.parser);
            }
            _ => {
                return {
                    Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "Unknown plugin: {}",
                        name
                    )))
                }
            }
        }
        Ok(())
    }
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
                Ok(Self {
                    parser,
                    xhtml_out: true,
                })
            }
            "gfm" => {
                let mut parser = markdown_it::MarkdownIt::new();
                markdown_it::plugins::cmark::add(&mut parser);
                markdown_it::plugins::html::add(&mut parser);
                markdown_it::plugins::extra::tables::add(&mut parser);
                markdown_it::plugins::extra::strikethrough::add(&mut parser);
                markdown_it_autolink::add(&mut parser);
                markdown_it_tasklist::add(&mut parser);
                Ok(Self {
                    parser,
                    xhtml_out: true,
                })
            }
            "zero" => Ok(Self {
                parser: markdown_it::MarkdownIt::new(),
                xhtml_out: false,
            }),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown config: {}",
                config
            ))),
        }
    }

    // keep this private for now, whilst we work out how to expose it properly
    fn _unset_lang_prefix(&mut self) {
        markdown_it::plugins::cmark::block::fence::set_lang_prefix(&mut self.parser, "");
    }

    #[staticmethod]
    fn list_plugins() -> Vec<String> {
        vec![
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
            "sourcepos",
            "strikethrough",
            "table",
            "front_matter",
            "tasklist",
            "footnote",
            "heading_anchors",
            "autolink_ext",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// Enable a plugin
    fn enable(slf: Py<Self>, py: Python, name: &str) -> PyResult<Py<Self>> {
        slf.borrow_mut(py)._enable(name)?;
        Ok(slf)
    }

    /// Enable multiple plugins
    fn enable_many(slf: Py<Self>, py: Python, names: Vec<&str>) -> PyResult<Py<Self>> {
        for name in names {
            slf.borrow_mut(py)._enable(name)?;
        }
        Ok(slf)
    }

    /// Render markdown string into HTML.
    fn render(&self, src: &str) -> String {
        let ast = self.parser.parse(src);
        match self.xhtml_out {
            true => ast.xrender(),
            false => ast.render(),
        }
    }

    /// Create a syntax tree from the markdown string.
    fn tree(&self, py: Python, src: &str) -> nodes::Node {
        let ast = self.parser.parse(src);

        fn walk_recursive(py: Python, node: &markdown_it::Node, py_node: &mut nodes::Node) {
            for n in node.children.iter() {
                let mut py_node_child = nodes::create_node(py, n);

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
    // let plugins_module = PyModule::new(py, "plugins")?;
    // plugins_module.add_function(wrap_pyfunction!(plugins::add_heading_anchors, plugins_module)?)?;
    // m.add_submodule(plugins_module)?;
    Ok(())
}
