use crate::{
    styles::{Style, StyleProp},
    Arena, Index,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NodeIndex(pub Index);

#[derive(Debug, Clone)]
pub struct Node {
    pub children: Vec<Index>,
    pub id: Index,
    pub styles: Style,
    pub z: f32,
}

impl Node {}

pub struct NodeBuilder {
    children: Vec<Index>,
    id: Index,
    styles: Style,
}

impl NodeBuilder {
    pub fn empty() -> Self {
        Self {
            children: Vec::new(),
            id: Index::default(),
            styles: Style::default(),
        }
    }

    pub fn new(id: Index, styles: Style) -> Self {
        Self {
            children: Vec::new(),
            id,
            styles,
        }
    }

    pub fn with_id(mut self, id: Index) -> Self {
        self.id = id;
        self
    }

    pub fn with_children(mut self, children: Vec<Index>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn with_styles(mut self, styles: Style) -> Self {
        self.styles = styles;
        self
    }

    pub fn build(self) -> Node {
        Node {
            children: self.children,
            id: self.id,
            styles: self.styles,
            z: 0.0,
        }
    }
}

impl<'a> morphorm::Node<'a> for NodeIndex {
    type Data = Arena<Option<Node>>;

    fn layout_type(&self, store: &'_ Self::Data) -> Option<morphorm::LayoutType> {
        if let Some(node) = store.get(self.0) {
            if let Some(node) = node {
                return match node.styles.layout_type {
                    StyleProp::Default => Some(morphorm::LayoutType::default()),
                    StyleProp::Value(prop) => Some(prop),
                    _ => None,
                };
            }
        }
        return None;
    }

    fn position_type(&self, store: &'_ Self::Data) -> Option<morphorm::PositionType> {
        if let Some(node) = store.get(self.0) {
            if let Some(node) = node {
                return match node.styles.position_type {
                    StyleProp::Default => Some(morphorm::PositionType::default()),
                    StyleProp::Value(prop) => Some(prop),
                    _ => None,
                };
            }
        }
        return None;
    }

    fn width(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Some(node) = store.get(self.0) {
            if let Some(node) = node {
                return match node.styles.width {
                    StyleProp::Default => Some(morphorm::Units::Stretch(1.0)),
                    StyleProp::Value(prop) => Some(prop),
                    _ => None,
                };
            }
        }
        return None;
    }

    fn height(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Some(node) = store.get(self.0) {
            if let Some(node) = node {
                return match node.styles.height {
                    StyleProp::Default => Some(morphorm::Units::Stretch(1.0)),
                    StyleProp::Value(prop) => Some(prop),
                    _ => None,
                };
            }
        }
        return None;
    }

    fn min_width(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn min_height(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn max_width(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn max_height(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn left(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Some(node) = store.get(self.0) {
            if let Some(node) = node {
                return match node.styles.left {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop),
                    _ => None,
                };
            }
        }
        return None;
    }

    fn right(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Some(node) = store.get(self.0) {
            if let Some(node) = node {
                return match node.styles.right {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop),
                    _ => None,
                };
            }
        }
        return None;
    }

    fn top(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Some(node) = store.get(self.0) {
            if let Some(node) = node {
                return match node.styles.top {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop),
                    _ => None,
                };
            }
        }
        return None;
    }

    fn bottom(&self, store: &'_ Self::Data) -> Option<morphorm::Units> {
        if let Some(node) = store.get(self.0) {
            if let Some(node) = node {
                return match node.styles.bottom {
                    StyleProp::Default => Some(morphorm::Units::Auto),
                    StyleProp::Value(prop) => Some(prop),
                    _ => None,
                };
            }
        }
        return None;
    }

    fn min_left(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn max_left(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn min_right(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn max_right(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn min_top(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn max_top(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn min_bottom(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn max_bottom(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn child_left(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn child_right(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn child_top(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn child_bottom(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn row_between(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn col_between(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn grid_rows(&self, _store: &'_ Self::Data) -> Option<Vec<morphorm::Units>> {
        Some(vec![])
    }

    fn grid_cols(&self, _store: &'_ Self::Data) -> Option<Vec<morphorm::Units>> {
        Some(vec![])
    }

    fn row_index(&self, _store: &'_ Self::Data) -> Option<usize> {
        Some(0)
    }

    fn col_index(&self, _store: &'_ Self::Data) -> Option<usize> {
        Some(0)
    }

    fn row_span(&self, _store: &'_ Self::Data) -> Option<usize> {
        Some(1)
    }

    fn col_span(&self, _store: &'_ Self::Data) -> Option<usize> {
        Some(1)
    }

    fn border_left(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn border_right(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn border_top(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }

    fn border_bottom(&self, _store: &'_ Self::Data) -> Option<morphorm::Units> {
        Some(morphorm::Units::Auto)
    }
}
