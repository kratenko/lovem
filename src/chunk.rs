pub struct Chunk {
    data: Vec<u8>,
    pos: usize,
}

impl Chunk {
    pub fn new(size: usize) -> Chunk {
        Chunk{
            data: vec![0; size],
            pos: 0,
        }
    }
/*
    pub fn split_pos(pos: usize) -> (usize, usize) {
        (pos >> 3, pos & 0b111)
    }

    pub fn write_bit_at(&mut self, pos: usize, hi: bool) {
        let (byte_pos, bit_pos) = Chunk::split_pos(pos);
        let byte = self.data[byte_pos];
        let byte = if hi {
            byte | (1 << bit_pos)
        } else {
            byte & !(1 << bit_pos)
        };
        self.data[byte_pos] = byte;
    }

    pub fn read_bit_at(&self, pos: usize) -> bool {
        let (byte_pos, bit_pos) = Chunk::split_pos(pos);
        let byte = self.data[byte_pos];
        (byte >> bit_pos & 1) == 1
    }

    pub fn write_bits(&mut self, v: i64, n: u8) {

    }
    */
}

#[cfg(test)]
mod tests {
    use crate::chunk::Chunk;

    #[test]
    fn new() {
        let ch = Chunk::new(3);
        assert_eq!(ch.pos, 0);
        assert_eq!(ch.data.capacity(), 3);
        assert_eq!(ch.data.len(), 3);
        for n in 0..ch.data.len() {
            assert_eq!(ch.data[n], 0)
        }
    }

    /*
    #[test]
    fn write_bit() {
        let mut ch = Chunk::new(10);
        assert_eq!(ch.data[0], 0);
        ch.write_bit_at(0, true);
        assert_eq!(ch.data[0], 1);
        ch.write_bit_at(6, true);
        assert_eq!(ch.data[0], 0b1000001);
        ch.write_bit_at(1, false);
        assert_eq!(ch.data[0], 0b1000001);
        ch.write_bit_at(0, false);
        assert_eq!(ch.data[0], 0b1000000);
    }
    */
}
