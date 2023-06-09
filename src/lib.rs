use pyo3::prelude::*;

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
        let mut found = true;
        let config_enum = match config {
            "commonmark" => Config::CommonMark,
            "zero" => Config::Zero,
            _ => {
                found = false;
                Config::Zero
            }
        };
        if found {
            Ok(MarkdownIt {
                config: config_enum,
                enable_list: Vec::new(),
            })
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown config: {}",
                config
            )))
        }
    }

    /// Enable a rule
    fn enable(&mut self, name: &str) -> PyResult<()> {
        let mut found = true;
        match name {
            "blockquote" => self.enable_list.push(Rules::Blockquote),
            "code" => self.enable_list.push(Rules::Code),
            "fence" => self.enable_list.push(Rules::Fence),
            "heading" => self.enable_list.push(Rules::Heading),
            "hr" => self.enable_list.push(Rules::Hr),
            "lheading" => self.enable_list.push(Rules::Lheading),
            "list" => self.enable_list.push(Rules::List),
            "paragraph" => self.enable_list.push(Rules::Paragraph),
            "reference" => self.enable_list.push(Rules::Reference),
            "autolink" => self.enable_list.push(Rules::Autolink),
            "backticks" => self.enable_list.push(Rules::Backticks),
            "emphasis" => self.enable_list.push(Rules::Emphasis),
            "entity" => self.enable_list.push(Rules::Entity),
            "escape" => self.enable_list.push(Rules::Escape),
            "image" => self.enable_list.push(Rules::Image),
            "link" => self.enable_list.push(Rules::Link),
            "newline" => self.enable_list.push(Rules::Newline),
            "html_block" => self.enable_list.push(Rules::HtmlBlock),
            "html_inline" => self.enable_list.push(Rules::HtmlInline),
            "linkify" => self.enable_list.push(Rules::Linkify),
            "replacements" => self.enable_list.push(Rules::Replacements),
            "smartquotes" => self.enable_list.push(Rules::Smartquotes),
            "strikethrough" => self.enable_list.push(Rules::Strikethrough),
            "table" => self.enable_list.push(Rules::Table),
            _ => {
                found = false;
            }
        }
        if found {
            Ok(())
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

        return Ok(ast.render());
    }

    /// Parse the source string to a string representation of the AST
    fn _ast_debug(&self, src: &str) -> PyResult<String> {
        let parser = &mut self.create_parser();
        let ast = parser.parse(src);

        // walk through the ast and save the node_type to a vector
        let mut node_types: Vec<String> = Vec::new();
        ast.walk(|node, depth| {
            let prefix = " ".repeat(depth.try_into().unwrap());
            let name = &node.name()[node.name().rfind("::").map(|x| x + 2).unwrap_or_default()..]
                .to_lowercase();
            node_types.push(format!("{}<{}>", prefix, name))
        });

        return Ok(node_types.join("\n"));
    }
}

/// A Python interface to markdown_it.rs
#[pymodule]
// Note: The name of this function must match the `lib.name` setting in the `Cargo.toml`,
// else Python will not be able to import the module.
fn markdown_it_pyrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<MarkdownIt>()?;
    Ok(())
}
