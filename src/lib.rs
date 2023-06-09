use pyo3::prelude::*;

enum Config {
    CommonMark,
    Zero,
}

enum Plugin {
    Linkify,
    Replacements,
    SmartQuotes,
    Strikethrough,
    Table,
}

/// Main parser class
#[pyclass]
struct MarkdownIt {
    /// The initial configuration of the parser
    config: Config,
    extensions: Vec<Plugin>,
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
        for plugin in &self.extensions {
            match plugin {
                Plugin::Linkify => {
                    markdown_it::plugins::extra::linkify::add(&mut parser);
                }
                Plugin::Replacements => {
                    markdown_it::plugins::extra::typographer::add(&mut parser);
                }
                Plugin::SmartQuotes => {
                    markdown_it::plugins::extra::smartquotes::add(&mut parser);
                }
                Plugin::Strikethrough => {
                    markdown_it::plugins::extra::strikethrough::add(&mut parser);
                }
                Plugin::Table => {
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
                extensions: Vec::new(),
            })
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown config: {}",
                config
            )))
        }
    }

    /// Enable a rule
    fn enable(&mut self, extension: &str) -> PyResult<()> {
        let mut found = true;
        match extension {
            "linkify" => self.extensions.push(Plugin::Linkify),
            "replacements" => self.extensions.push(Plugin::Replacements),
            "smartquotes" => self.extensions.push(Plugin::SmartQuotes),
            "strikethrough" => self.extensions.push(Plugin::Strikethrough),
            "table" => self.extensions.push(Plugin::Table),
            _ => {
                found = false;
            }
        }
        if found {
            Ok(())
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown rule: {}",
                extension
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
