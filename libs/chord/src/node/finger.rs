use crate::Node;

pub struct Finger {
    pub(crate) start: u64,
    pub node: Node,
}

impl Finger {
    /// Generate a finger id for a given node id and finger index.
    /// The finger id is calculated using the following formula:
    /// ```text
    /// (node_id + 2^(index - 1)) % 2^m
    /// ```
    ///
    /// Ref: https://pdos.csail.mit.edu/papers/ton:chord/paper-ton.pdf
    /// Ref: https://en.wikipedia.org/wiki/Chord_(peer-to-peer)#Finger_table
    ///
    /// # Arguments
    ///
    /// * `node_id` - The id of the node
    /// * `index` - The index of the finger
    pub(crate) fn finger_id(node_id: u64, index: u8) -> u64 {
        Self::sized_finger_id(64_u8, node_id, index)
    }

    pub(crate) fn sized_finger_id(size: u8, node_id: u64, index: u8) -> u64 {
        if index == 0 {
            return node_id;
        }

        let offset: u128 = 2_u128.pow((index - 1) as u32);
        let power: u128 = 2_u128.pow(size as u32);

        let id = (node_id as u128 + offset) % power;

        id as u64
    }

    /// Initialize a new finger table for a given node id and its successor.
    /// All the fingers in the table will point to the same successor.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the node
    /// * `node` - The successor of the node
    pub(crate) fn init_finger_table(id: u64, node: Node) -> Vec<Self> {
        Self::sized_finger_table(64, id, node)
    }

    #[cfg(test)]
    /// Initialize a new finger table for a given node id and its successor with a specific size.
    /// All the fingers in the table will point to the same successor.
    /// This function is only used for testing.
    /// The size of the finger table is 64 by default.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the finger table
    /// * `id` - The id of the node
    /// * `node` - The successor of the node
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::net::SocketAddr;
    /// use chord_rs::Node;
    ///
    /// let node = Node::with_id(1, SocketAddr::from(([127, 0, 0, 1], 42001)));
    /// let finger_table = Finger::init_sized_finger_table(6, node.id(), node);
    /// assert_eq!(finger_table.len(), 6);
    /// ```
    pub(crate) fn init_sized_finger_table(size: u8, id: u64, node: Node) -> Vec<Self> {
        Self::sized_finger_table(size, id, node)
    }

    fn sized_finger_table(size: u8, id: u64, node: Node) -> Vec<Self> {
        let mut fingers = Vec::with_capacity(size as usize);

        // We start at 1 because the calculation of the finger id is based on the index
        // of the finger. The calculation assumes that the index starts at 1.
        for i in 1..size+1 {
            let finger_id = Self::sized_finger_id(size, id, i);
            fingers.push(Finger { start: finger_id, node: node.clone() });
        }

        fingers
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use super::*;

    #[test]
    fn it_should_generate_finger_id() {
        let node_id: u64 = 1;

        assert_eq!(Finger::finger_id(node_id, 0), 1);
        assert_eq!(Finger::finger_id(node_id, 1), 2);
        assert_eq!(Finger::finger_id(node_id, 2), 3);
        assert_eq!(Finger::finger_id(node_id, 3), 5);
        assert_eq!(Finger::finger_id(node_id, 4), 9);
        assert_eq!(Finger::finger_id(node_id, 5), 17);
        assert_eq!(Finger::finger_id(node_id, 6), 33);
        assert_eq!(Finger::finger_id(node_id, 7), 65);
        assert_eq!(Finger::finger_id(node_id, 8), 129);
        assert_eq!(Finger::finger_id(node_id, 9), 257);
        assert_eq!(Finger::finger_id(node_id, 10), 513);
        assert_eq!(Finger::finger_id(node_id, 11), 1025);
        assert_eq!(Finger::finger_id(node_id, 12), 2049);
        assert_eq!(Finger::finger_id(node_id, 13), 4097);
        assert_eq!(Finger::finger_id(node_id, 14), 8193);
        assert_eq!(Finger::finger_id(node_id, 15), 16385);
        assert_eq!(Finger::finger_id(node_id, 32), 2147483649);
        assert_eq!(Finger::finger_id(node_id, 64), 9223372036854775809);
        assert_eq!(Finger::finger_id(node_id, 65), 1);

        const M: u8 = 6;
        assert_eq!(Finger::sized_finger_id(M, node_id, 0), 1);
        assert_eq!(Finger::sized_finger_id(M, node_id, 1), 2);
        assert_eq!(Finger::sized_finger_id(M, node_id, 2), 3);
        assert_eq!(Finger::sized_finger_id(M, node_id, 3), 5);
        assert_eq!(Finger::sized_finger_id(M, node_id, 4), 9);
        assert_eq!(Finger::sized_finger_id(M, node_id, 5), 17);
        assert_eq!(Finger::sized_finger_id(M, node_id, 6), 33);
        assert_eq!(Finger::sized_finger_id(M, node_id, 7), 1);
    }

    #[test]
    fn it_should_generate_finger_table() {
        let node_id: u64 = 1;
        let node = Node::with_id(2, SocketAddr::from(([127, 0, 0, 1], 42001)));

        let fingers = Finger::init_finger_table(node_id, node.clone());

        assert_eq!(fingers.len(), 64);
        assert_eq!(fingers[0].start, 2);
        assert_eq!(fingers[1].start, 3);
        assert_eq!(fingers[2].start, 5);
        assert_eq!(fingers[3].start, 9);
        assert_eq!(fingers[4].start, 17);
        assert_eq!(fingers[5].start, 33);
        assert_eq!(fingers[15].start, 32769);
        assert_eq!(fingers[63].start, 9223372036854775809);

        let fingers = Finger::sized_finger_table(6, 5, node);

        assert_eq!(fingers.len(), 6);
        assert_eq!(fingers[0].start, 6);
        assert_eq!(fingers[1].start, 7);
        assert_eq!(fingers[2].start, 9);
        assert_eq!(fingers[3].start, 13);
        assert_eq!(fingers[4].start, 21);
        assert_eq!(fingers[5].start, 37);
    }
}
