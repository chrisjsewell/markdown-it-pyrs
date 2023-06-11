use pyo3::prelude::*;

mod nodes;

/// The initial configuration of the parser
enum Config {
    CommonMark,
    Zero,
}

/// All the rules that can be enabled
enum Rules {
    // commonmark block
    Blockquote,
    Code,
    Fence,
    Heading,
    Hr,
    Lheading,
    List,
    Paragraph,
    Reference,
    // commonmark inline
    Autolink,
    Backticks,
    Emphasis,
    Entity,
    Escape,
    Image,
    Link,
    Newline,
    // commonmark html
    HtmlBlock,
    HtmlInline,
    // extras
    Linkify,
    Replacements,
    Smartquotes,
    Strikethrough,
    Table,
}

/// Main parser class
#[pyclass]
struct MarkdownIt {
    /// The initial configuration of the parser
    config: Config,
    /// All the rules to be enabled
    enable_list: Vec<Rules>,

    // TODO put options in separate struct
    xhtml_out: bool,
}

impl MarkdownIt {
    /// Create a new parser instance
    fn create_parser(&self) -> markdown_it::MarkdownIt {
        let mut parser = markdown_it::MarkdownIt::new();
        match self.config {
            Config::CommonMark => {
                markdown_it::plugins::cmark::add(&mut parser);
                markdown_it::plugins::html::add(&mut parser);
            }
            Config::Zero => {}
        }
        for plugin in &self.enable_list {
            match plugin {
                Rules::Blockquote => {
                    markdown_it::plugins::cmark::block::blockquote::add(&mut parser);
                }
                Rules::Code => {
                    markdown_it::plugins::cmark::block::code::add(&mut parser);
                }
                Rules::Fence => {
                    markdown_it::plugins::cmark::block::fence::add(&mut parser);
                }
                Rules::Heading => {
                    markdown_it::plugins::cmark::block::heading::add(&mut parser);
                }
                Rules::Hr => {
                    markdown_it::plugins::cmark::block::hr::add(&mut parser);
                }
                Rules::Lheading => {
                    markdown_it::plugins::cmark::block::lheading::add(&mut parser);
                }
                Rules::List => {
                    markdown_it::plugins::cmark::block::list::add(&mut parser);
                }
                Rules::Paragraph => {
                    markdown_it::plugins::cmark::block::paragraph::add(&mut parser);
                }
                Rules::Reference => {
                    markdown_it::plugins::cmark::block::reference::add(&mut parser);
                }
                Rules::Autolink => {
                    markdown_it::plugins::cmark::inline::autolink::add(&mut parser);
                }
                Rules::Backticks => {
                    markdown_it::plugins::cmark::inline::backticks::add(&mut parser);
                }
                Rules::Emphasis => {
                    markdown_it::plugins::cmark::inline::emphasis::add(&mut parser);
                }
                Rules::Entity => {
                    markdown_it::plugins::cmark::inline::entity::add(&mut parser);
                }
                Rules::Escape => {
                    markdown_it::plugins::cmark::inline::escape::add(&mut parser);
                }
                Rules::Image => {
                    markdown_it::plugins::cmark::inline::image::add(&mut parser);
                }
                Rules::Link => {
                    markdown_it::plugins::cmark::inline::link::add(&mut parser);
                }
                Rules::Newline => {
                    markdown_it::plugins::cmark::inline::newline::add(&mut parser);
                }
                Rules::HtmlBlock => {
                    markdown_it::plugins::html::html_block::add(&mut parser);
                }
                Rules::HtmlInline => {
                    markdown_it::plugins::html::html_inline::add(&mut parser);
                }
                Rules::Linkify => {
                    markdown_it::plugins::extra::linkify::add(&mut parser);
                }
                Rules::Replacements => {
                    markdown_it::plugins::extra::typographer::add(&mut parser);
                }
                Rules::Smartquotes => {
                    markdown_it::plugins::extra::smartquotes::add(&mut parser);
                }
                Rules::Strikethrough => {
                    markdown_it::plugins::extra::strikethrough::add(&mut parser);
                }
                Rules::Table => {
                    markdown_it::plugins::extra::tables::add(&mut parser);
                }
            }
        }
        parser
    }
}

#[pymethods]
impl MarkdownIt {
    #[new]
    #[pyo3(signature = (config="commonmark"))]
    fn new(config: &str) -> PyResult<Self> {
        match config {
            "commonmark" => Ok(MarkdownIt {
                config: Config::CommonMark,
                enable_list: Vec::new(),
                xhtml_out: true,
            }),
            "zero" => Ok(MarkdownIt {
                config: Config::Zero,
                enable_list: Vec::new(),
                xhtml_out: false,
            }),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown config: {}",
                config
            ))),
        }
    }

    /// Enable a rule
    fn enable(slf: Py<Self>, name: &str) -> PyResult<Py<Self>> {
        let mut found = true;
        Python::with_gil(|py| {
            let mut slf_mut = slf.borrow_mut(py);
            match name {
                "blockquote" => slf_mut.enable_list.push(Rules::Blockquote),
                "code" => slf_mut.enable_list.push(Rules::Code),
                "fence" => slf_mut.enable_list.push(Rules::Fence),
                "heading" => slf_mut.enable_list.push(Rules::Heading),
                "hr" => slf_mut.enable_list.push(Rules::Hr),
                "lheading" => slf_mut.enable_list.push(Rules::Lheading),
                "list" => slf_mut.enable_list.push(Rules::List),
                "paragraph" => slf_mut.enable_list.push(Rules::Paragraph),
                "reference" => slf_mut.enable_list.push(Rules::Reference),
                "autolink" => slf_mut.enable_list.push(Rules::Autolink),
                "backticks" => slf_mut.enable_list.push(Rules::Backticks),
                "emphasis" => slf_mut.enable_list.push(Rules::Emphasis),
                "entity" => slf_mut.enable_list.push(Rules::Entity),
                "escape" => slf_mut.enable_list.push(Rules::Escape),
                "image" => slf_mut.enable_list.push(Rules::Image),
                "link" => slf_mut.enable_list.push(Rules::Link),
                "newline" => slf_mut.enable_list.push(Rules::Newline),
                "html_block" => slf_mut.enable_list.push(Rules::HtmlBlock),
                "html_inline" => slf_mut.enable_list.push(Rules::HtmlInline),
                "linkify" => slf_mut.enable_list.push(Rules::Linkify),
                "replacements" => slf_mut.enable_list.push(Rules::Replacements),
                "smartquotes" => slf_mut.enable_list.push(Rules::Smartquotes),
                "strikethrough" => slf_mut.enable_list.push(Rules::Strikethrough),
                "table" => slf_mut.enable_list.push(Rules::Table),
                _ => {
                    found = false;
                }
            }
        });
        if found {
            Ok(slf)
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown rule: {}",
                name
            )))
        }
    }

    /// Render markdown string into HTML.
    fn render(&self, src: &str) -> PyResult<String> {
        let parser = &mut self.create_parser();
        let ast = parser.parse(src);

        match self.xhtml_out {
            true => {
                return Ok(ast.xrender());
            }
            false => {
                return Ok(ast.render());
            }
        }
    }

    /// Create a syntax tree from the markdown string.
    fn tree(&self, src: &str) -> nodes::Node {
        let parser = &mut self.create_parser();
        let ast = parser.parse(src);

        fn walk_recursive<'a>(node: &'a markdown_it::Node, py_node: &mut nodes::Node) {
            for n in node.children.iter() {
                let mut py_node_child = nodes::create_node(&n);

                stacker::maybe_grow(64 * 1024, 1024 * 1024, || {
                    walk_recursive(n, &mut py_node_child);
                });

                Python::with_gil(|py| {
                    py_node.children.push(Py::new(py, py_node_child).unwrap());
                });
            }
        }

        let mut py_node = nodes::create_node(&ast);

        walk_recursive(&ast, &mut py_node);

        return py_node;
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
