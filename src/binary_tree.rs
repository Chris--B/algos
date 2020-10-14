struct Node<T>
where
    T: Ord,
{
    item: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
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
        use std::cmp::Ordering;

        // Select which half of the tree to search depending on the relation
        // between `target` and our current item
        match self.item().cmp(target) {
            Ordering::Equal => Some(self),
            // The invariant of our tree is that all elements Less than `self.item`
            // are accessible through `self.left`.
            Ordering::Less => self.left()?.find(target),
            // Likewise for Greater and `self.right`.
            Ordering::Greater => self.right()?.find(target),
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
    root: Option<Node<T>>,
}

impl<T> Default for BinaryTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        BinaryTree { root: None }
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
        self.root.is_some()
    }

    pub fn insert(&mut self, _item: T) {
        todo!();
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
