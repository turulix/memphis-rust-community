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

    pub(crate) fn next(&self) -> Option<&T> {
        self.partitions
            .get((self.index + 1) % self.partitions.len())
    }
}
