use core::cmp::Ordering;
use core::fmt::{self, Debug};

struct Node<T>
where
    T: Ord, // TODO: Loosen this to PartialOrd somehow.
{
    item: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Clone for Node<T>
where
    T: Ord + Clone,
{
    fn clone(&self) -> Self {
        let item = self.item.clone();
        let left = self.left.as_ref().cloned();
        let right = self.right.as_ref().cloned();

        Node { item, left, right }
    }
}

impl<T> Debug for Node<T>
where
    T: Ord + Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Node")
            .field("item", &self.item)
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(item: T) -> Self {
        Node {
            item,
            left: None,
            right: None,
        }
    }

    fn item(&self) -> &T {
        &self.item
    }

    fn left(&self) -> Option<&Node<T>> {
        self.left.as_ref().map(|n| n.as_ref())
    }

    fn right(&self) -> Option<&Node<T>> {
        self.right.as_ref().map(|n| n.as_ref())
    }

    fn height(&self) -> usize {
        let left_h = self.left().map(|n| n.height()).unwrap_or_default();
        let right_h = self.right().map(|n| n.height()).unwrap_or_default();

        // The height of this node is the larger of either subtree.
        // This node also counts for height, so include it.
        1 + left_h.max(right_h)
    }

    fn find(&self, target: &T) -> Option<&Node<T>> {
        // Select which half of the tree to search depending on the relation
        // between `target` and our current item
        match target.cmp(self.item()) {
            // Trivial case - we found the node!
            Ordering::Equal => Some(self),

            // The invariant of our tree is that all elements Less than `self.item`
            // are accessible through `self.left`.
            Ordering::Less => self.left()?.find(target),

            // Likewise for Greater and `self.right`.
            Ordering::Greater => self.right()?.find(target),
        }
    }

    fn insert(&mut self, new_node: Node<T>) -> bool {
        match new_node.item().cmp(self.item()) {
            Ordering::Equal => {
                return false;
            }

            Ordering::Less => match &mut self.left {
                Some(node) => {
                    node.insert(new_node);
                }
                None => {
                    self.left = Some(Box::new(new_node));
                }
            },

            Ordering::Greater => match &mut self.right {
                Some(node) => {
                    node.insert(new_node);
                }
                None => {
                    self.right = Some(Box::new(new_node));
                }
            },
        };

        true
    }

    // Helper method to prep this node to be removed.
    //
    // The item that this node held and the adjusted subtree are returned.
    // The adjusted subtree should be placed where this node was, and the item
    // should be returned up to the caller on the Tree object.
    fn remove_self(self) -> (T, Option<Box<Node<T>>>) {
        let Node { item, left, right } = self;

        match (left, right) {
            (None, None) => {
                // No subtree to manage, we can return a NULL node
                (item, None)
            }
            (Some(node), None) | (None, Some(node)) => {
                // There's only one sub tree, we should return that in our place
                (item, Some(node))
            }
            (Some(_l), Some(_r)) => {
                // hard part
                todo!()
            }
        }
    }

    fn remove_item(&mut self, item: &T) -> Option<T> {
        match item.cmp(&self.item) {
            Ordering::Equal => {
                // We shouldn't have gotten into this node if our current item
                // was the item to remove.
                unreachable!(
                    "Node at 0x{:x} is equal to item to remove. This should not happen",
                    self as *const _ as usize
                );
            }

            Ordering::Less => {
                if self.left().map(|n| n.item()) == Some(item) {
                    // We found our node!
                    // Replace it with is subtree, adjusting as necessary
                    let left: Node<T> = *self.left.take().unwrap();
                    // Adjust the subtree and move out our item
                    let (item, node) = left.remove_self();
                    // and hook it up
                    self.left = node;

                    Some(item)
                } else {
                    // Continue searching down the left
                    if let Some(item) = self.left.as_mut().and_then(|n| n.remove_item(item)) {
                        // The left side found the item and removed it - continue returning it
                        Some(item)
                    } else {
                        // The left side did not contain the item, therefore it isn't in our tree.
                        // There's nothing to remove.
                        None
                    }
                }
            }

            Ordering::Greater => {
                // We found our node:
                if self.right().map(|n| n.item()) == Some(item) {
                    // We found our node!
                    // Replace it with is subtree, adjusting as necessary
                    let right: Node<T> = *self.right.take().unwrap();
                    // Adjust the subtree and move out our item
                    let (item, node) = right.remove_self();
                    // and hook it up
                    self.right = node;

                    Some(item)
                } else {
                    // Continue searching down the right
                    if let Some(item) = self.right.as_mut().and_then(|n| n.remove_item(item)) {
                        // The right side found the item and removed it - continue returning it
                        Some(item)
                    } else {
                        // The right side did not contain the item, therefore it isn't in our tree.
                        // There's nothing to remove.
                        None
                    }
                }
            }
        }
    }

    fn min(&self) -> &Node<T> {
        let mut node = self;

        // The invariant of our tree is that the left node is always Less than
        // the current node.
        // Therefore, walk left until we're out of nodes.
        while let Some(left) = node.left() {
            node = left;
        }

        node
    }

    fn max(&self) -> &Node<T> {
        let mut node = self;

        // The invariant of our tree is that the right node is always Greater than
        // the current node.
        // Therefore, walk right until we're out of nodes.
        while let Some(right) = node.right() {
            node = right;
        }

        node
    }

    fn for_each<'a>(&'a self, f: &mut impl FnMut(&'a T)) {
        // Process the left side of the tree, if present, first.
        // We do this first to give our traversal in-order semantics.
        if let Some(left) = self.left() {
            left.for_each(f);
        }

        // Process this item now, since all items Less than it have been
        // processed already.
        f(self.item());

        // Continue down the right side, if present, last.
        if let Some(right) = self.right() {
            right.for_each(f);
        }
    }
}

pub struct BinaryTree<T>
where
    T: Ord,
{
    root: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Default for BinaryTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        BinaryTree { root: None, len: 0 }
    }
}

impl<T> Clone for BinaryTree<T>
where
    T: Ord + Clone,
{
    fn clone(&self) -> Self {
        let root = self.root.clone();
        let len = self.len;

        BinaryTree { root, len }
    }
}

impl<T> PartialEq for BinaryTree<T>
where
    T: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for (left, right) in self.iter().zip(other.iter()) {
            if left != right {
                return false;
            }
        }

        true
    }
}

impl<T> Debug for BinaryTree<T>
where
    T: Ord + Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BinaryTree")
            .field("root", &self.root)
            .finish()
    }
}

macro_rules! impl_from_array {
    ($($array_len:expr,)+) => {
        $(
            impl<T> From<[T; $array_len]> for BinaryTree<T>
            where
                T: Ord + Clone, // TODO: We should remove the Clone bound.
            {
                fn from(ts: [T; $array_len]) -> Self {
                    ts.iter().cloned().collect()
                }
            }
        )+
    }
}

impl_from_array![
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, //
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19, //
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29, //
    30, 31, 32,
];

impl<T> From<&[T]> for BinaryTree<T>
where
    T: Ord + Clone, // TODO: We should remove the Clone bound.
{
    fn from(slice: &[T]) -> Self {
        slice.iter().cloned().collect()
    }
}

impl<T> From<Vec<T>> for BinaryTree<T>
where
    T: Ord,
{
    fn from(slice: Vec<T>) -> Self {
        slice.into_iter().collect()
    }
}

impl<T> From<BinaryTree<T>> for Vec<T>
where
    T: Ord + Clone, // TODO: We should remove the Clone bound.
{
    fn from(tree: BinaryTree<T>) -> Vec<T> {
        tree.iter().cloned().collect()
    }
}

impl<T> BinaryTree<T>
where
    T: Ord,
{
    /// Create an empty binary tree
    pub fn new() -> Self {
        BinaryTree::default()
    }

    /// Number of items stored in this tree.
    pub fn len(&self) -> usize {
        // TODO: Cache this value instead
        let mut len = 0;
        self.for_each(|_t| len += 1);

        len
    }

    /// Whether there are any items in this tree.
    pub fn is_empty(&self) -> bool {
        // TODO: Use len() instead (after it's cached)
        self.root.is_none()
    }

    /// Adds a value to the tree.
    /// If the tree did not have this value present, `true` is returned.
    /// If the tree did have this value present, `false` is returned.
    pub fn insert(&mut self, item: T) -> bool {
        let new_node = Node::new(item);
        let inserted = match &mut self.root {
            Some(root) => root.insert(new_node),
            None => {
                self.root = Some(Box::new(new_node));
                true
            }
        };

        if !inserted {
            self.len += 1;
        }

        inserted
    }

    /// Removes an item and returns it if found
    pub fn remove_item(&mut self, item: &T) -> Option<T> {
        if self.root.as_ref().map(|r| r.item()) == Some(item) {
            // We found our node! (that was fast?)
            // Replace it with is subtree, adjusting as necessary
            let root: Node<T> = *self.root.take().unwrap();
            // Adjust the subtree and move out our item
            let (item, node) = root.remove_self();
            // and hook it up
            self.root = node;

            Some(item)
        } else {
            self.root.as_mut().and_then(|r| r.remove_item(item))
        }
    }

    /// Height of the tree
    ///
    /// The tree's height is the maximum number of nodes from the root to a
    /// leaf node. This is approximately `O(lg N)`, where `N` = `self.len()`.
    pub fn height(&self) -> usize {
        self.root.as_ref().map(|r| r.height()).unwrap_or_default()
    }

    /// Returns true if the tree contains an element with the given value.
    pub fn contains(&self, item: &T) -> bool {
        self.root.as_ref().and_then(|r| r.find(item)).is_some()
    }

    /// Returns the minimum item in the tree, or `None` if there are no items.
    pub fn min(&self) -> Option<&T> {
        self.root.as_ref().map(|r| r.min().item())
    }

    /// Returns the maximum item in the tree, or `None` if there are no items.
    pub fn max(&self) -> Option<&T> {
        self.root.as_ref().map(|r| r.max().item())
    }

    /// Call `f` once per item in the tree
    ///
    /// Nodes are traversed in Depth First order, meaning they are accessed
    /// in increasing order.
    pub fn for_each<'a>(&'a self, mut f: impl FnMut(&'a T)) {
        if let Some(r) = self.root.as_ref() {
            r.for_each(&mut f);
        }
    }

    /// Iterate over the nodes in-order, with each processed node Greater than
    /// or Equal to the previous node.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        // Cheat: Collect a buffer of nodes and use its iterator
        let mut nodes: Vec<&'a T> = Vec::with_capacity(self.len());

        self.for_each(|t| nodes.push(t));

        nodes.into_iter()
    }
}

impl<T> std::iter::FromIterator<T> for BinaryTree<T>
where
    T: Ord,
{
    fn from_iter<I: std::iter::IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = BinaryTree::new();

        for item in iter {
            tree.insert(item);
        }

        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn check_basic_builder() {
        let items: Vec<_> = (-10..=10).collect();
        let tree: BinaryTree<i32> = items.clone().into();

        let tree_items: Vec<_> = tree.clone().into();
        assert_eq!(items, tree_items);

        // Check misc things
        assert_eq!(items.iter().min(), tree.min());
        assert_eq!(items.iter().max(), tree.max());
    }

    #[test]
    fn check_insert() {
        let mut tree = BinaryTree::new();

        // First insert succeeds
        assert_eq!(tree.insert(1), true);

        // Second one fails
        assert_eq!(tree.insert(1), false);

        // Unrelated one succeeds
        assert_eq!(tree.insert(2), true);
    }

    #[test]
    fn check_contains() {
        let items: Vec<_> = (-10..=10).collect();
        let tree: BinaryTree<i32> = items.clone().into();

        for (i, item) in items.iter().enumerate() {
            assert!(
                tree.contains(item),
                "items[{}] == {}, but not found in tree",
                i,
                item
            );
        }
    }

    // This tree borrowed from:
    // Skiena's Algorithm Design Manual pg 81, section 3.4.1
    const SKIENA_TREE: &[i32] = &[2, 1, 7, 4, 8, 3, 6, 5];

    #[test]
    fn check_len_and_height() {
        let mut tree = BinaryTree::default();
        assert_eq!(0, tree.len());
        assert!(tree.is_empty());

        let expected_heights = [1_usize, 2, 2, 3, 3, 4, 4, 5];

        for (i, (item, expected_height)) in SKIENA_TREE
            .iter()
            .copied()
            .zip(expected_heights.iter().copied())
            .enumerate()
        {
            let item: i32 = item;
            let expected_height: usize = expected_height;
            tree.insert(item);

            dbg!(i, &tree);
            assert_eq!(
                expected_height,
                tree.height(),
                "Checking height after inserting {}",
                item
            );
        }
    }

    /// Remove a left node with 0 children
    #[test]
    fn check_delete_skiena_ex_3() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_eq!(tree.len(), SKIENA_TREE.len());

        let removed = tree.remove_item(&3);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = vec![2, 1, 7, 4, 8, 6, 5].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(Some(3), removed);
        assert_eq!(SKIENA_TREE.len() - 1, tree.len());
    }

    /// Remove a right node with 0 children
    #[test]
    fn check_delete_skiena_ex_8() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_eq!(tree.len(), SKIENA_TREE.len());

        let removed = tree.remove_item(&8);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = vec![2, 1, 7, 4, 3, 6, 5].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(Some(8), removed);
        assert_eq!(SKIENA_TREE.len() - 1, tree.len());
    }

    // Remove a node with 1 child
    #[test]
    fn check_delete_skiena_ex_6() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_eq!(tree.len(), SKIENA_TREE.len());

        let removed = tree.remove_item(&6);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = vec![2, 1, 7, 4, 8, 3, 5].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(Some(6), removed);
        assert_eq!(SKIENA_TREE.len() - 1, tree.len());
    }

    // Remove a node with 2 children
    #[test]
    fn check_delete_skiena_ex_4() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_eq!(tree.len(), SKIENA_TREE.len());

        let removed = tree.remove_item(&4);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = vec![2, 1, 7, 5, 8, 3, 6].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(Some(4), removed);
        assert_eq!(SKIENA_TREE.len() - 1, tree.len());
    }

    #[test]
    fn check_delete_root_simple() {
        let mut tree: BinaryTree<i32> = [1].into();

        assert_eq!(tree.remove_item(&1), Some(1));
        assert_eq!(tree, BinaryTree::new());
    }

    #[test]
    fn check_delete_skiena_ex_root() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_eq!(tree.len(), SKIENA_TREE.len());

        let removed = tree.remove_item(&2);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = vec![3, 1, 7, 4, 8, 6, 5].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(Some(2), removed);
        assert_eq!(SKIENA_TREE.len() - 1, tree.len());
    }

    #[test]
    fn check_delete_non_existing() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_eq!(tree.len(), SKIENA_TREE.len());

        // Remove an item once
        let removed = tree.remove_item(&4);
        assert_eq!(Some(4), removed);

        // Then attempt to remove it once more.
        let removed = tree.remove_item(&4);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = vec![2, 1, 7, 5, 8, 3, 6].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(removed, None);
        assert_eq!(SKIENA_TREE.len() - 1, tree.len());
    }
}
