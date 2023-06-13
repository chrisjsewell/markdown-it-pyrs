use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
/// Single node in the Markdown AST tree.
pub struct Node {
    #[pyo3(get)]
    /// The absolute path to the rust Node implementation.
    pub _rust_path: Option<String>,

    #[pyo3(get)]
    /// The type of the node
    pub name: String,

    #[pyo3(get)]
    /// Array of child nodes.
    // See https://github.com/PyO3/pyo3/discussions/3223#discussioncomment-6144333
    pub children: Vec<Py<Node>>,

    #[pyo3(get)]
    /// Byte offset mapping of the (start, end) of the source syntax.
    pub srcmap: Option<(usize, usize)>,

    #[pyo3(get)]
    /// Additional attributes to be added to resulting html.
    pub attrs: HashMap<String, String>,

    #[pyo3(get)]
    /// Custom data specific to the node type.
    pub meta: HashMap<String, PyObject>,
}

impl Node {
    fn _walk(&self, py: Python) -> Vec<Py<Node>> {
        let mut nodes: Vec<Py<Node>> = Vec::new();
        for child in self.children.iter() {
            nodes.push(child.clone_ref(py));
            for node in child.borrow(py)._walk(py) {
                nodes.push(node);
            }
        }
        nodes
    }
}

#[pymethods]
impl Node {
    #[new]
    pub fn new(name: &str) -> Self {
        Self {
            _rust_path: None,
            name: name.to_string(),
            children: Vec::new(),
            srcmap: None,
            attrs: HashMap::new(),
            meta: HashMap::new(),
        }
    }
    fn __repr__(&self) -> String {
        format!("Node({})", self.name)
    }
    fn __str__(&self) -> String {
        self.__repr__()
    }

    /// Recursively yield all descendant nodes in the tree starting at self.
    ///
    /// The order mimics the order of the underlying linear token
    /// stream (i.e. depth first).
    #[pyo3(signature = (*, include_self=true))]
    fn walk(slf: Py<Self>, py: Python, include_self: bool) -> Vec<Py<Node>> {
        let mut nodes: Vec<Py<Node>> = Vec::new();
        if include_self {
            nodes.push(slf.clone_ref(py));
        }
        nodes.extend(slf.borrow(py)._walk(py));
        nodes
    }

    /// create a pretty string representation of the node
    ///
    /// :param attrs: whether to include attributes in the output
    /// :param srcmap: whether to include source map in the output
    /// :param content: whether to include content meta field in the output
    /// :param meta: whether to include meta fields in the output
    /// :param recurse: whether to recurse into child nodes
    /// :param indent: number of spaces to increase indent for each level
    /// :param indent_current: number of spaces to indent the current level
    #[pyo3(signature = (*, attrs=false, srcmap=false, content=false, meta=false, recurse=true, indent=2, indent_current=0))]
    fn pretty(
        &self,
        attrs: bool,
        srcmap: bool,
        content: bool,
        meta: bool,
        recurse: bool,
        indent: usize,
        indent_current: usize,
    ) -> String {
        let mut inner = self.name.clone();
        if attrs {
            for (key, value) in self.attrs.iter() {
                inner.push_str(&format!(" {}=\"{}\"", key, value));
            }
        }
        if srcmap {
            if let Some((start, end)) = self.srcmap {
                inner.push_str(&format!(" srcmap=\"{}:{}\"", start, end));
            }
        }
        let mut s = format!("{}<{}>\n", " ".repeat(indent_current), inner);
        if meta {
            for (key, value) in self.meta.iter() {
                let mut meta_value = format!("{}", value);
                meta_value =
                    meta_value.replace("\n", &format!("\n{}", " ".repeat(indent_current + indent)));
                s.push_str(&format!(
                    "{}{}: {}\n",
                    " ".repeat(indent_current + indent),
                    key,
                    meta_value
                ));
            }
        }
        if content {
            if let Some(value) = self.meta.get("content") {
                let mut content_value = format!("{}", value);
                content_value = content_value
                    .replace("\n", &format!("\n{}", " ".repeat(indent_current + indent)));
                s.push_str(&format!(
                    "{}{}\n",
                    " ".repeat(indent_current + indent),
                    content_value
                ));
            }
        }
        if recurse {
            for child in self.children.iter() {
                Python::with_gil(|py| {
                    s.push_str(&child.borrow(py).pretty(
                        attrs,
                        srcmap,
                        content,
                        meta,
                        recurse,
                        indent,
                        indent_current + indent,
                    ));
                });
            }
        }
        s
    }
}

/// Take a markdown_it::Node and return a Python compatible Node
pub fn create_node(py: Python, node: &markdown_it::Node) -> Node {
    // default to a node with the same name as the markdown_it::Node
    let mut py_node = Node::new("unknown");
    py_node._rust_path = Some(node.name().to_string());

    // aspects that are common to all nodes
    for (key, value) in node.attrs.iter() {
        py_node.attrs.insert(key.to_string(), value.to_string());
    }
    if let Some(srcmap) = node.srcmap {
        py_node.srcmap = Some(srcmap.get_byte_offsets());
    }

    if let Some(_) = node.cast::<markdown_it::parser::core::Root>() {
        py_node.name = "root".to_string();
    } else if let Some(node_value) = node.cast::<markdown_it::parser::inline::Text>() {
        py_node.name = "text".to_string();
        py_node.meta.insert(
            "content".to_string(),
            node_value.content.to_string().into_py(py),
        );
    } else if let Some(node_value) = node.cast::<markdown_it::parser::inline::TextSpecial>() {
        py_node.name = "text_special".to_string();
        py_node.meta.insert(
            "content".to_string(),
            node_value.content.to_string().into_py(py),
        );
        py_node.meta.insert(
            "markup".to_string(),
            node_value.markup.to_string().into_py(py),
        );
        py_node
            .meta
            .insert("info".to_string(), node_value.info.into_py(py));
    } else if let Some(_) =
        node.cast::<markdown_it::plugins::cmark::block::blockquote::Blockquote>()
    {
        py_node.name = "blockquote".to_string();
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::code::CodeBlock>()
    {
        py_node.name = "code_block".to_string();
        py_node.meta.insert(
            "content".to_string(),
            node_value.content.to_string().into_py(py),
        );
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::fence::CodeFence>()
    {
        py_node.name = "fence".to_string();
        py_node
            .meta
            .insert("info".to_string(), node_value.info.to_string().into_py(py));
        py_node
            .meta
            .insert("marker".to_string(), node_value.marker.into_py(py));
        py_node
            .meta
            .insert("marker_len".to_string(), node_value.marker_len.into_py(py));
        py_node.meta.insert(
            "content".to_string(),
            node_value.content.to_string().into_py(py),
        );
        py_node.meta.insert(
            "lang_prefix".to_string(),
            node_value.lang_prefix.into_py(py),
        );
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::heading::ATXHeading>()
    {
        py_node.name = "heading".to_string();
        py_node
            .meta
            .insert("level".to_string(), node_value.level.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::hr::ThematicBreak>()
    {
        py_node.name = "hr".to_string();
        py_node
            .meta
            .insert("marker".to_string(), node_value.marker.into_py(py));
        py_node
            .meta
            .insert("marker_len".to_string(), node_value.marker_len.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::lheading::SetextHeader>()
    {
        py_node.name = "lheading".to_string();
        py_node
            .meta
            .insert("level".to_string(), node_value.level.into_py(py));
        py_node
            .meta
            .insert("marker".to_string(), node_value.marker.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::list::BulletList>()
    {
        py_node.name = "bullet_list".to_string();
        py_node
            .meta
            .insert("marker".to_string(), node_value.marker.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::list::OrderedList>()
    {
        py_node.name = "ordered_list".to_string();
        py_node
            .meta
            .insert("start".to_string(), node_value.start.into_py(py));
        py_node
            .meta
            .insert("marker".to_string(), node_value.marker.into_py(py));
    } else if let Some(_) = node.cast::<markdown_it::plugins::cmark::block::list::ListItem>() {
        py_node.name = "list_item".to_string();
    } else if let Some(_) = node.cast::<markdown_it::plugins::cmark::block::paragraph::Paragraph>()
    {
        py_node.name = "paragraph".to_string();
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::inline::autolink::Autolink>()
    {
        py_node.name = "autolink".to_string();
        py_node
            .meta
            .insert("url".to_string(), node_value.url.to_string().into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::inline::backticks::CodeInline>()
    {
        py_node.name = "code_inline".to_string();
        py_node
            .meta
            .insert("marker".to_string(), node_value.marker.into_py(py));
        py_node
            .meta
            .insert("marker_len".to_string(), node_value.marker_len.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::inline::emphasis::Em>()
    {
        py_node.name = "em".to_string();
        py_node
            .meta
            .insert("marker".to_string(), node_value.marker.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::inline::emphasis::Strong>()
    {
        py_node.name = "strong".to_string();
        py_node
            .meta
            .insert("marker".to_string(), node_value.marker.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::inline::image::Image>()
    {
        py_node.name = "image".to_string();
        py_node
            .meta
            .insert("url".to_string(), node_value.url.to_string().into_py(py));
        match &node_value.title {
            Some(title) => {
                py_node
                    .meta
                    .insert("title".to_string(), title.to_string().into_py(py));
            }
            None => {}
        }
    } else if let Some(node_value) = node.cast::<markdown_it::plugins::cmark::inline::link::Link>()
    {
        py_node.name = "link".to_string();
        py_node
            .meta
            .insert("url".to_string(), node_value.url.to_string().into_py(py));
        match &node_value.title {
            Some(title) => {
                py_node
                    .meta
                    .insert("title".to_string(), title.to_string().into_py(py));
            }
            None => {}
        }
    } else if let Some(_) = node.cast::<markdown_it::plugins::cmark::inline::newline::Hardbreak>() {
        py_node.name = "hardbreak".to_string();
    } else if let Some(_) = node.cast::<markdown_it::plugins::cmark::inline::newline::Softbreak>() {
        py_node.name = "softbreak".to_string();
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::html::html_inline::HtmlInline>()
    {
        py_node.name = "html_inline".to_string();
        py_node.meta.insert(
            "content".to_string(),
            node_value.content.to_string().into_py(py),
        );
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::html::html_block::HtmlBlock>()
    {
        py_node.name = "html_block".to_string();
        py_node.meta.insert(
            "content".to_string(),
            node_value.content.to_string().into_py(py),
        );
    } else if let Some(node_value) = node.cast::<markdown_it::plugins::extra::linkify::Linkified>()
    {
        py_node.name = "linkify".to_string();
        py_node
            .meta
            .insert("url".to_string(), node_value.url.to_string().into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::extra::strikethrough::Strikethrough>()
    {
        py_node.name = "strikethrough".to_string();
        py_node
            .meta
            .insert("marker".to_string(), node_value.marker.into_py(py));
    } else if let Some(node_value) = node.cast::<markdown_it::plugins::extra::tables::Table>() {
        py_node.name = "table".to_string();
        py_node.meta.insert(
            "alignments".to_string(),
            node_value
                .alignments
                .iter()
                .map(|x| match x {
                    markdown_it::plugins::extra::tables::ColumnAlignment::None => {
                        "none".to_string()
                    }
                    markdown_it::plugins::extra::tables::ColumnAlignment::Left => {
                        "left".to_string()
                    }
                    markdown_it::plugins::extra::tables::ColumnAlignment::Center => {
                        "center".to_string()
                    }
                    markdown_it::plugins::extra::tables::ColumnAlignment::Right => {
                        "right".to_string()
                    }
                })
                .collect::<Vec<String>>()
                .into_py(py),
        );
    } else if let Some(_) = node.cast::<markdown_it::plugins::extra::tables::TableBody>() {
        py_node.name = "tbody".to_string();
    } else if let Some(_) = node.cast::<markdown_it::plugins::extra::tables::TableRow>() {
        py_node.name = "trow".to_string();
    } else if let Some(_) = node.cast::<markdown_it::plugins::extra::tables::TableCell>() {
        py_node.name = "tcell".to_string();
    } else if let Some(_) = node.cast::<markdown_it::plugins::extra::tables::TableHead>() {
        py_node.name = "thead".to_string();
    }

    py_node
}
