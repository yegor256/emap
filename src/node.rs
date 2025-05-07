// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NodeId {
    node_id: usize,
}

#[derive(Debug)]
pub(crate) struct Node<V> {
    next: NodeId,
    prev: NodeId,
    value: Option<V>,
}

impl NodeId {
    pub const UNDEF: usize = usize::MAX;

    #[inline]
    #[must_use]
    pub const fn new(node_id: usize) -> Self {
        Self { node_id }
    }

    #[inline]
    #[must_use]
    pub const fn is_undef(self) -> bool {
        self.node_id == Self::UNDEF
    }

    #[inline]
    #[must_use]
    pub const fn is_def(self) -> bool {
        self.node_id != Self::UNDEF
    }

    #[inline]
    #[must_use]
    pub const fn get(self) -> usize {
        self.node_id
    }
}

impl<V> Node<V> {
    #[inline]
    #[must_use]
    pub const fn new(next: usize, prev: usize, value: Option<V>) -> Self {
        Self {
            next: NodeId::new(next),
            prev: NodeId::new(prev),
            value,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_some(&self) -> bool {
        self.value.is_some()
    }

    #[inline]
    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.value.is_none()
    }

    #[inline]
    #[must_use]
    pub const fn get_next(&self) -> NodeId {
        self.next
    }

    #[inline]
    #[must_use]
    pub const fn get_prev(&self) -> NodeId {
        self.prev
    }

    #[inline]
    pub const fn update_next(&mut self, new_next: NodeId) {
        self.next = new_next;
    }

    #[inline]
    pub const fn update_prev(&mut self, new_prev: NodeId) {
        self.prev = new_prev;
    }

    #[inline]
    pub fn replace_value(&mut self, value: Option<V>) {
        self.value = value;
    }

    #[inline]
    #[must_use]
    pub const fn get(&self) -> Option<&V> {
        match &self.value {
            Some(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn get_mut(&mut self) -> Option<&mut V> {
        match &mut self.value {
            Some(v) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod node_id_tests {
    use super::*;

    #[test]
    fn node_id_basic() {
        let id = NodeId::new(42);
        assert_eq!(id.get(), 42);
        assert!(id.is_def());
        assert!(!id.is_undef());
    }

    #[test]
    fn node_id_undef() {
        let undef = NodeId::new(NodeId::UNDEF);
        assert!(undef.is_undef());
        assert!(!undef.is_def());
        assert_eq!(undef.get(), NodeId::UNDEF);
    }

    #[test]
    fn node_id_equality() {
        let id1 = NodeId::new(10);
        let id2 = NodeId::new(10);
        let id3 = NodeId::new(20);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn node_id_const() {
        const TEST_ID: NodeId = NodeId::new(100);
        assert_eq!(TEST_ID.get(), 100);
    }
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn node_creation() {
        let node = Node::new(1, 2, Some("value"));
        assert_eq!(node.get_next().get(), 1);
        assert_eq!(node.get_prev().get(), 2);
        assert!(node.is_some());
        assert_eq!(node.get(), Some(&"value"));
    }

    #[test]
    fn node_empty() {
        let node: Node<String> = Node::new(NodeId::UNDEF, NodeId::UNDEF, None);
        assert!(node.is_none());
        assert!(node.get().is_none());
        assert!(node.get_next().is_undef());
    }

    #[test]
    fn node_value_mutation() {
        let mut node = Node::new(0, 0, Some(10));
        *node.get_mut().unwrap() = 20;
        assert_eq!(node.get(), Some(&20));

        node.replace_value(None);
        assert!(node.is_none());
    }

    #[test]
    fn node_pointer_updates() {
        let mut node = Node::new(1, 2, Some(3.14));
        node.update_next(NodeId::new(10));
        node.update_prev(NodeId::new(20));

        assert_eq!(node.get_next().get(), 10);
        assert_eq!(node.get_prev().get(), 20);
    }

    #[test]
    fn node_debug() {
        let node = Node::new(1, 2, Some("test"));
        assert!(format!("{:?}", node).contains("test"));
    }

    #[test]
    fn node_complex_type() {
        #[derive(Debug, PartialEq)]
        struct Point(i32, i32);

        let mut node = Node::new(0, 0, Some(Point(1, 2)));
        assert_eq!(node.get(), Some(&Point(1, 2)));

        node.get_mut().unwrap().0 = 3;
        assert_eq!(node.get(), Some(&Point(3, 2)));
    }
}
