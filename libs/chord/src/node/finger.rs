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
        if index == 0 {
            return node_id;
        }

        let offset: u128 = 2_u128.pow((index - 1) as u32);
        const POWER: u128 = 2_u128.pow(64);

        let id = (node_id as u128 + offset) % POWER;

        id as u64
    }

    pub(crate) fn init_finger_table(id: u64, node: Node) -> Vec<Self> {
        let mut fingers = Vec::with_capacity(64);

        for i in 1..65 {
            let finger_id = Self::finger_id(id, i);

            fingers.push(Finger { start: finger_id, node: node.clone() });
        }

        fingers
    }
}

#[cfg(test)]
mod tests {
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
    }
}
