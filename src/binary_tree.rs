use core::cmp::Ordering;
use core::fmt::{self, Debug};

struct Node<T>
where
    T: Ord,
{
    item: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
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
            Ordering::Equal => Some(self),
            // The invariant of our tree is that all elements Less than `self.item`
            // are accessible through `self.left`.
            Ordering::Less => self.left()?.find(target),
            // Likewise for Greater and `self.right`.
            Ordering::Greater => self.right()?.find(target),
        }
    }

    fn insert(&mut self, new_node: Node<T>) {
        match new_node.item().cmp(self.item()) {
            // This item is already in the tree and can be ignored
            Ordering::Equal => {}

            Ordering::Less => match &mut self.left {
                Some(node) => node.insert(new_node),
                None => {
                    self.left = Some(Box::new(new_node));
                }
            },

            Ordering::Greater => match &mut self.right {
                Some(node) => node.insert(new_node),
                None => {
                    self.right = Some(Box::new(new_node));
                }
            },
        };
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

impl<T> From<T> for Node<T>
where
    T: Ord,
{
    fn from(t: T) -> Self {
        Node::new(t)
    }
}

pub struct BinaryTree<T>
where
    T: Ord,
{
    root: Option<Box<Node<T>>>,
}

impl<T> Default for BinaryTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        BinaryTree { root: None }
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

impl<T> From<&[T]> for BinaryTree<T>
where
    T: Ord + Clone,
{
    fn from(ts: &[T]) -> Self {
        ts.iter().cloned().collect()
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

    /// Move `item` into the Tree
    pub fn insert(&mut self, item: T) {
        let new_node = Node::new(item);
        match &mut self.root {
            Some(root) => root.insert(new_node),
            None => self.root = Some(Box::new(new_node)),
        }
    }

    /// Removes an item and returns it if found
    pub fn remove_item(&mut self, _item: &T) -> Option<T> {
        // TODO!
        None
    }

    /// Height of the tree
    ///
    /// The tree's height is the maximum number of nodes from the root to a
    /// leaf node. This is approximately `O(lg N)`, where `N` = `self.len()`.
    pub fn height(&self) -> usize {
        if let Some(root) = self.root.as_ref() {
            root.height()
        } else {
            0
        }
    }

    /// Returns true if the tree contains an element with the given value.
    pub fn contains(&self, item: &T) -> bool {
        if let Some(root) = self.root.as_ref() {
            root.find(item).is_some()
        } else {
            false
        }
    }

    /// Returns the minimum item in the tree, or `None` if there are no items.
    pub fn min(&self) -> Option<&T> {
        if let Some(root) = self.root.as_ref() {
            Some(root.min().item())
        } else {
            None
        }
    }

    /// Returns the maximum item in the tree, or `None` if there are no items.
    pub fn max(&self) -> Option<&T> {
        if let Some(root) = self.root.as_ref() {
            Some(root.max().item())
        } else {
            None
        }
    }

    /// Call `f` once per item in the tree
    ///
    /// Nodes are traversed in Depth First order, meaning they are accessed
    /// in increasing order.
    pub fn for_each<'a>(&'a self, mut f: impl FnMut(&'a T)) {
        if let Some(root) = self.root.as_ref() {
            root.for_each(&mut f);
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

    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn check_basic_builder() {
        let items: Vec<_> = (-10..=10).collect();
        let tree: BinaryTree<i32> = items.iter().copied().collect();

        let tree_items: Vec<_> = tree.iter().copied().collect();
        assert_eq!(items, tree_items);

        // Check misc things
        assert_eq!(items.iter().min(), tree.min());
        assert_eq!(items.iter().max(), tree.max());
    }

    #[test]
    fn check_contains() {
        let items: Vec<_> = (-10..=10).collect();
        let tree: BinaryTree<i32> = items.iter().copied().collect();

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

    #[test]
    fn check_delete_skiena_ex_3() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_ne!(tree.len(), 0);

        dbg!(&tree);

        let removed = tree.remove_item(&3);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = [2, 1, 7, 4, 8, 6, 5][..].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(removed, Some(3));
    }

    #[test]
    fn check_delete_skiena_ex_6() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_ne!(tree.len(), 0);

        let removed = tree.remove_item(&6);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = [2, 1, 7, 4, 8, 3, 5][..].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(removed, Some(6));
    }

    #[test]
    fn check_delete_skiena_ex_4() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_ne!(tree.len(), 0);

        let removed = tree.remove_item(&4);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = [2, 1, 7, 5, 8, 3, 6][..].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(removed, Some(4));
    }

    #[test]
    fn check_delete_skiena_ex_root() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_ne!(tree.len(), 0);

        let removed = tree.remove_item(&2);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = [3, 1, 7, 4, 8, 6, 5][..].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(removed, Some(2));
    }

    #[test]
    fn check_delete_non_existing() {
        let mut tree: BinaryTree<i32> = SKIENA_TREE.into();
        assert_ne!(tree.len(), 0);

        // Remove an item once
        let removed = tree.remove_item(&4);
        assert_eq!(removed, Some(4));

        // Then attempt to remove it once more.
        let removed = tree.remove_item(&4);
        dbg!(&removed);

        let expected_tree: BinaryTree<_> = [2, 1, 7, 5, 8, 3, 6][..].into();
        assert_eq!(expected_tree, tree);
        assert_eq!(removed, None);
    }
}
