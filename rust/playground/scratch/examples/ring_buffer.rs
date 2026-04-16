use std::cmp::min;

#[derive(Debug, Default)]
struct RingBuffer {
    buf: Vec<u8>,
    current_size: usize,
    read_idx: usize,
    write_idx: usize,
}

// cases
// [_, a, b, _, _] -> write_idx > read_idx
// [empty, data] -> 
// [data, empty]
// [data]
// [empty]

// RingBuffer Invariants
// current_size is correct
impl RingBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            buf: vec![0; capacity],
            ..Default::default()
        }
    }

    fn capacity_available(&self) -> usize {
        self.buf.len() - self.current_size
    }

    fn capacity_used(&self) -> usize {
        self.current_size
    }

    fn write_iter(&mut self, src: &[u8]) -> usize {
        let capacity_available = self.capacity_available();
        if capacity_available == 0 || src.len() == 0 {
            return 0;
        }
        let to_write = min(capacity_available, src.len());
        let mut src_idx = 0;
        for _ in 0..to_write {
            self.buf[self.write_idx] = src[src_idx];
            src_idx += 1;
            self.write_idx = (self.write_idx + 1) % self.buf.len();
        }
        self.current_size += to_write;
        to_write
    }

    fn read_iter(&mut self, dst: &mut [u8]) -> usize {
        let capacity_used = self.capacity_used();
        if capacity_used == 0 || dst.len() == 0 {
            return 0;
        }
        let to_read = min(capacity_used, dst.len());
        let mut dst_idx = 0;
        for _ in 0..to_read {
            dst[dst_idx] = self.buf[self.read_idx];
            self.buf[self.read_idx] = 0;
            dst_idx += 1;
            self.read_idx = (self.read_idx + 1) % self.buf.len();
        }
        self.current_size -= to_read;
        to_read
    }

    fn write_memcpy(&mut self, src: &[u8]) -> usize {
        let capacity_available = self.capacity_available();
        if capacity_available == 0 || src.len() == 0 {
            return 0;
        }
        let to_write = min(capacity_available, src.len());
        let mut src_idx = 0;
        let left_at_end = min(self.buf.len() - self.write_idx, src.len());
        self.buf[self.write_idx..self.write_idx + left_at_end].copy_from_slice(&src[..left_at_end]);
        src_idx += left_at_end;
        self.write_idx = (self.write_idx + left_at_end) % self.buf.len();

        let left_to_write = to_write - left_at_end;
        // handle [data, empty]
        if left_to_write == 0 {
            self.current_size += left_at_end;
            return to_write;
        }

        // empty [empty, data]
        self.buf[0..left_to_write].copy_from_slice(&src[src_idx..src_idx + left_to_write]);
        self.write_idx = (self.write_idx + left_to_write) % self.buf.len();
        self.current_size += to_write;
        to_write
    }

    fn read_memcpy(&mut self, dst: &mut [u8]) -> usize {
        let capacity_used = self.capacity_used();
        if capacity_used == 0 || dst.len() == 0 {
            return 0;
        }
        let to_read = min(capacity_used, dst.len());
        let mut src_idx = 0;
        let left_at_end = min(self.buf.len() - self.read_idx, to_read);
        // [0, r_1, 2, w_0, 0]
        dst[..src_idx + left_at_end]
            .copy_from_slice(&self.buf[self.read_idx..self.read_idx + left_at_end]);
        src_idx += left_at_end;
        self.read_idx = (self.read_idx + left_at_end) % self.buf.len();

        let left_to_read = to_read - left_at_end;
        // handle [data, empty]
        if left_to_read == 0 {
            self.current_size -= left_at_end;
            return to_read;
        }

        // empty [empty, data]
        dst[src_idx..src_idx + left_to_read].copy_from_slice(&self.buf[0..left_to_read]);
        self.read_idx = (self.read_idx + left_to_read) % self.buf.len();
        self.current_size -= to_read;
        to_read
    }
}

/*
[ _ _ _ ] create(3)
[ a b _ ] write "ab" -> return 2

"As with any queue, we start writing into the beginning of the memory we allocated."
[ a b c ] write "c" -> return 1

"If you try to write more data than there is space in the buffer, it fails. To indicate this, we always return the number of characters we have successfully written."

[ _ b c ] read(1) -> return "a"

[ e b c ] write "e" -> return 1

"Here you can see that the data WRAPS AROUND the end of the buffer to the beginning."

[ e _ c ] read(1) -> return "b"

"However, since this is a FIRST IN, FIRST OUT queue, we always return the OLDEST element in the buffer when reading from it."
*/

fn main() {
    test_1();
    test_2();
}

fn test_1() {
    let mut rb = RingBuffer::new(3);
    assert_eq!(rb.write_iter(b"ABC"), 3);
    assert_eq!(rb.write_iter(b"ABC"), 0);
    let mut out = [0u8; 3];
    assert_eq!(rb.read_iter(&mut out), 3);
    assert_eq!(&out, b"ABC");

    // [a, _, _]
    assert_eq!(rb.write_iter(b"A"), 1);
    // [a, b, _]
    assert_eq!(rb.write_iter(b"B"), 1);
    let mut out = [0u8; 1];
    // [_, b, _]
    assert_eq!(rb.read_iter(&mut out), 1);
    assert_eq!(&out, b"A");

    // [a, b, c], but start at b
    assert_eq!(rb.write_iter(b"CA"), 2);
    let mut out = [0u8; 3];
    assert_eq!(rb.read_iter(&mut out), 3);
    assert_eq!(&out, b"BCA");
}

fn test_2() {
    {
        let mut rb = RingBuffer::new(3);
        assert_eq!(rb.write_memcpy(b"ABC"), 3);
        assert_eq!(rb.write_memcpy(b"ABCDEF"), 0);
        let mut out = [0u8; 3];
        assert_eq!(rb.read_memcpy(&mut out), 3);
        assert_eq!(&out, b"ABC");

        // [a, _, _]
        assert_eq!(rb.write_memcpy(b"A"), 1);
        // [a, b, _]
        assert_eq!(rb.write_memcpy(b"B"), 1);
        let mut out = [0u8; 1];
        // [_, b, _]
        assert_eq!(rb.read_memcpy(&mut out), 1);
        assert_eq!(&out, b"A");

        // [a, b, c], but start at b
        assert_eq!(rb.write_memcpy(b"CA"), 2);
        let mut out = [0u8; 3];
        assert_eq!(rb.read_memcpy(&mut out), 3);
        assert_eq!(&out, b"BCA");
    }
    {
        let mut rb = RingBuffer::new(3);
        assert_eq!(rb.write_memcpy(b"ABC"), 3);
        let mut out = [0u8; 1];
        // [_, b, c]
        assert_eq!(rb.read_memcpy(&mut out), 1);
        assert_eq!(&out, b"A");
        assert_eq!(rb.write_memcpy(b"A"), 1);
        let mut out = [0u8; 3];
        assert_eq!(rb.read_memcpy(&mut out), 3);
        assert_eq!(&out, b"BCA");
    }

    {
        let mut rb = RingBuffer::new(4);
        // [a, b, c, _]
        assert_eq!(rb.write_memcpy(b"ABC"), 3);
        let mut out = [0u8; 1];
        // [_, b, c, _]
        assert_eq!(rb.read_memcpy(&mut out), 1);
        assert_eq!(&out, b"A");
        // [a, b, c, d]
        assert_eq!(rb.write_memcpy(b"DA"), 2);
        let mut out = [0u8; 6];
        assert_eq!(rb.read_memcpy(&mut out), 4);
        assert_eq!(&out, b"BCDA\0\0");
        assert_eq!(rb.write_memcpy(b"ABCDEFG"), 4);
        assert_eq!(rb.write_memcpy(b"ABCDEFG"), 0);
    }
}
