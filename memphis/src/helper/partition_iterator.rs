pub(crate) struct PartitionIterator<T> {
    partitions: Vec<T>,
    index: usize,
}

impl<T> PartitionIterator<T> {
    pub(crate) fn new(partitions: Vec<T>) -> Self {
        Self {
            partitions,
            index: 0,
        }
    }
}

impl<T> Iterator for PartitionIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.index = (self.index + 1) % self.partitions.len();
        self.partitions.get(self.index).cloned()
    }
}
