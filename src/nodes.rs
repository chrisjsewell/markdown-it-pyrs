use itertools::Itertools;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
/// Single node in the Markdown AST tree.
pub struct Node {
    #[pyo3(get, set)]
    /// The absolute path to the rust Node implementation.
    pub _rust_path: Option<String>,

    #[pyo3(get, set)]
    /// The type of the node
    pub name: String,

    #[pyo3(get, set)]
    /// Array of child nodes.
    // See https://github.com/PyO3/pyo3/discussions/3223#discussioncomment-6144333
    pub children: Vec<Py<Node>>,

    #[pyo3(get, set)]
    /// Byte offset mapping of the (start, end) of the source syntax.
    pub srcmap: Option<(usize, usize)>,

    #[pyo3(get, set)]
    /// Additional attributes to be added to resulting html.
    pub attrs: HashMap<String, String>,

    #[pyo3(get, set)]
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

    /// add a key/value pair of node type specific data
    fn add_data(&mut self, key: &str, value: Py<PyAny>) {
        self.meta.insert(key.to_string(), value);
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
    #[allow(clippy::too_many_arguments)]
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
            for (key, value) in self.attrs.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
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
            for (key, value) in self.meta.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
                let mut meta_value = format!("{}", value);
                meta_value =
                    meta_value.replace('\n', &format!("\n{}", " ".repeat(indent_current + indent)));
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
                    .replace('\n', &format!("\n{}", " ".repeat(indent_current + indent)));
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

    if node.cast::<markdown_it::parser::core::Root>().is_some() {
        py_node.name = "root".to_string();
    } else if let Some(node_value) = node.cast::<markdown_it::parser::inline::Text>() {
        py_node.name = "text".to_string();
        py_node.add_data("content", node_value.content.to_string().into_py(py));
    } else if let Some(node_value) = node.cast::<markdown_it::parser::inline::TextSpecial>() {
        py_node.name = "text_special".to_string();
        py_node.add_data("content", node_value.content.to_string().into_py(py));
        py_node.add_data("markup", node_value.markup.to_string().into_py(py));
        py_node.add_data("info", node_value.info.into_py(py));
    } else if node
        .cast::<markdown_it::plugins::cmark::block::blockquote::Blockquote>()
        .is_some()
    {
        py_node.name = "blockquote".to_string();
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::code::CodeBlock>()
    {
        py_node.name = "code_block".to_string();
        py_node.add_data("content", node_value.content.to_string().into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::fence::CodeFence>()
    {
        py_node.name = "fence".to_string();
        py_node.add_data("info", node_value.info.to_string().into_py(py));
        py_node.add_data("marker", node_value.marker.into_py(py));
        py_node.add_data("marker_len", node_value.marker_len.into_py(py));
        py_node.add_data("content", node_value.content.to_string().into_py(py));
        py_node.add_data("lang_prefix", node_value.lang_prefix.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::heading::ATXHeading>()
    {
        py_node.name = "heading".to_string();
        py_node.add_data("level", node_value.level.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::hr::ThematicBreak>()
    {
        py_node.name = "hr".to_string();
        py_node.add_data("marker", node_value.marker.into_py(py));
        py_node.add_data("marker_len", node_value.marker_len.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::lheading::SetextHeader>()
    {
        py_node.name = "lheading".to_string();
        py_node.add_data("level", node_value.level.into_py(py));
        py_node.add_data("marker", node_value.marker.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::list::BulletList>()
    {
        py_node.name = "bullet_list".to_string();
        py_node.add_data("marker", node_value.marker.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::block::list::OrderedList>()
    {
        py_node.name = "ordered_list".to_string();
        py_node.add_data("start", node_value.start.into_py(py));
        py_node.add_data("marker", node_value.marker.into_py(py));
    } else if node
        .cast::<markdown_it::plugins::cmark::block::list::ListItem>()
        .is_some()
    {
        py_node.name = "list_item".to_string();
    } else if node
        .cast::<markdown_it::plugins::cmark::block::paragraph::Paragraph>()
        .is_some()
    {
        py_node.name = "paragraph".to_string();
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::inline::autolink::Autolink>()
    {
        py_node.name = "autolink".to_string();
        py_node.add_data("url", node_value.url.to_string().into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::inline::backticks::CodeInline>()
    {
        py_node.name = "code_inline".to_string();
        py_node.add_data("marker", node_value.marker.into_py(py));
        py_node.add_data("marker_len", node_value.marker_len.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::inline::emphasis::Em>()
    {
        py_node.name = "em".to_string();
        py_node.add_data("marker", node_value.marker.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::inline::emphasis::Strong>()
    {
        py_node.name = "strong".to_string();
        py_node.add_data("marker", node_value.marker.into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::cmark::inline::image::Image>()
    {
        py_node.name = "image".to_string();
        py_node.add_data("url", node_value.url.to_string().into_py(py));
        if let Some(title) = &node_value.title {
            py_node.add_data("title", title.to_string().into_py(py));
        }
    } else if let Some(node_value) = node.cast::<markdown_it::plugins::cmark::inline::link::Link>()
    {
        py_node.name = "link".to_string();
        py_node.add_data("url", node_value.url.to_string().into_py(py));
        match &node_value.title {
            Some(title) => {
                py_node.add_data("title", title.to_string().into_py(py));
            }
            None => {}
        }
    } else if node
        .cast::<markdown_it::plugins::cmark::inline::newline::Hardbreak>()
        .is_some()
    {
        py_node.name = "hardbreak".to_string();
    } else if node
        .cast::<markdown_it::plugins::cmark::inline::newline::Softbreak>()
        .is_some()
    {
        py_node.name = "softbreak".to_string();
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::html::html_inline::HtmlInline>()
    {
        py_node.name = "html_inline".to_string();
        py_node.add_data("content", node_value.content.to_string().into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::html::html_block::HtmlBlock>()
    {
        py_node.name = "html_block".to_string();
        py_node.add_data("content", node_value.content.to_string().into_py(py));
    } else if let Some(node_value) = node.cast::<markdown_it::plugins::extra::linkify::Linkified>()
    {
        py_node.name = "linkify".to_string();
        py_node.add_data("url", node_value.url.to_string().into_py(py));
    } else if let Some(node_value) =
        node.cast::<markdown_it::plugins::extra::strikethrough::Strikethrough>()
    {
        py_node.name = "strikethrough".to_string();
        py_node.add_data("marker", node_value.marker.into_py(py));
    } else if let Some(node_value) = node.cast::<markdown_it::plugins::extra::tables::Table>() {
        py_node.name = "table".to_string();
        py_node.add_data(
            "alignments",
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
    } else if node
        .cast::<markdown_it::plugins::extra::tables::TableBody>()
        .is_some()
    {
        py_node.name = "tbody".to_string();
    } else if node
        .cast::<markdown_it::plugins::extra::tables::TableRow>()
        .is_some()
    {
        py_node.name = "trow".to_string();
    } else if node
        .cast::<markdown_it::plugins::extra::tables::TableCell>()
        .is_some()
    {
        py_node.name = "tcell".to_string();
    } else if node
        .cast::<markdown_it::plugins::extra::tables::TableHead>()
        .is_some()
    {
        py_node.name = "thead".to_string();
    } else if let Some(node_value) = node.cast::<markdown_it_front_matter::FrontMatter>() {
        py_node.name = "front_matter".to_string();
        py_node.add_data("content", node_value.content.to_string().into_py(py));
    } else if let Some(node_value) = node.cast::<markdown_it_tasklist::TodoCheckbox>() {
        py_node.name = "todo_checkbox".to_string();
        py_node.add_data("checked", node_value.checked.into_py(py));
        py_node.add_data("disabled", node_value.disabled.into_py(py));
    } else if node
        .cast::<markdown_it_footnote::inline::InlineFootnote>()
        .is_some()
    {
        py_node.name = "footnote_inline".to_string();
    } else if let Some(node_value) =
        node.cast::<markdown_it_footnote::references::FootnoteReference>()
    {
        py_node.name = "footnote_ref".to_string();
        py_node.add_data("def_id", node_value.def_id.into_py(py));
        py_node.add_data("ref_id", node_value.ref_id.into_py(py));
        match &node_value.label {
            Some(label) => {
                py_node.add_data("label", label.into_py(py));
            }
            None => {}
        }
    } else if let Some(node_value) =
        node.cast::<markdown_it_footnote::definitions::FootnoteDefinition>()
    {
        py_node.name = "footnote_def".to_string();
        py_node.add_data("def_id", node_value.def_id.into_py(py));
        py_node.add_data("inline", node_value.inline.into_py(py));
        match &node_value.label {
            Some(label) => {
                py_node.add_data("label", label.into_py(py));
            }
            None => {}
        }
    } else if node
        .cast::<markdown_it_footnote::collect::FootnotesContainerNode>()
        .is_some()
    {
        py_node.name = "footnote_container".to_string();
    } else if let Some(node_value) =
        node.cast::<markdown_it_footnote::back_refs::FootnoteRefAnchor>()
    {
        py_node.name = "footnote_ref_anchor".to_string();
        py_node.add_data("ref_ids", node_value.ref_ids.to_object(py));
    } else if let Some(node_value) = node.cast::<markdown_it_heading_anchors::HeadingAnchor>() {
        py_node.name = "heading_anchor".to_string();
        py_node.add_data("href", node_value.href.to_object(py));
        match &node_value.id {
            Some(id) => {
                py_node.add_data("id", id.into_py(py));
            }
            None => {}
        }
    }

    py_node
}
