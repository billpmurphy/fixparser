/// Append-only memory pool.
pub trait MemPool {
    /// Allocate some bytes to the pool.
    fn push(&mut self, bytes: &[u8]) -> Option<&[u8]>;

    /// Free the entire pool.
    fn free(&mut self);

    /// Return the current number of bytes stored in the pool.
    fn len(&self) -> usize;

    /// Return the maximum size of the pool, if applicable.
    fn max_len(&self) -> Option<usize>;
}

/// Flexible size heap-pushated memory pool that uses Vec<u8> for storage.
pub struct VecMemPool {
    raw: Vec<u8>,
}

impl VecMemPool {
    /// Initialize the memory pool with a starting capacity.
    fn new(cap: usize) -> VecMemPool {
        VecMemPool { raw: Vec::with_capacity(cap) }
    }
}

impl MemPool for VecMemPool {
    fn push(&mut self, bytes: &[u8]) -> Option<&[u8]> {
        let index = self.raw.len();
        for &byte in bytes {
            self.raw.push(byte);
        }
        Some(&self.raw[index..index+bytes.len()])
    }

    fn free(&mut self) {
        self.raw = Vec::new();
    }

    fn len(&self) -> usize {
        self.raw.len()
    }

    fn max_len(&self) -> Option<usize> {
        None
    }
}


/// Fixed-size 4096-byte stack pushated memory pool.
pub struct StackMemPool4096 {
    raw: [u8; 4096],
    ptr: usize,
}

impl StackMemPool4096 {
    fn new() -> StackMemPool4096 {
        StackMemPool4096 { raw: [0; 4096], ptr: 0 }
    }
}

impl MemPool for StackMemPool4096 {
    fn push(&mut self, bytes: &[u8]) -> Option<&[u8]> {
        if self.ptr + bytes.len() > self.raw.len() {
            None
        }
        else {
            for (i, &byte) in bytes.iter().enumerate() {
                self.raw[self.ptr+i] = byte;
            }
            self.ptr += bytes.len();
            Some(&self.raw[self.ptr-bytes.len()..self.ptr])
        }
    }

    fn free(&mut self) {
        self.raw = [0; 4096];
        self.ptr = 0;
    }

    fn len(&self) -> usize {
        self.ptr
    }

    fn max_len(&self) -> Option<usize> {
        Some(4096)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vecmempool() {
        // We should be able to push to this forever, since it's on the heap
        let mut pool = VecMemPool::new(20);
        for i in 0..4000 {
            assert!(pool.push(&[(i % 256) as u8]).is_some());
        }
        assert_eq!(pool.len(), 4000);

        // We can free as long as there are no living references to the contents of the pool
        {
            let ptr = pool.push(&[1u8]).unwrap();
        }
        pool.free();
    }

    #[test]
    fn test_stackmempool4096() {
        let mut pool = StackMemPool4096::new();

        // We can push 4096 bytes no problem
        for i in 0..4096 {
            assert!(pool.push(&[(i % 256) as u8]).is_some());
        }

        // allocator is filled, we can no longer push
        assert!(pool.push(&[0u8]).is_none());

        // After free, we can push again
        pool.free();
        assert_eq!(pool.len(), 0);

        assert!(pool.push(&[0u8, 1u8, 2u8, 3u8]).is_some());
        assert_eq!(pool.len(), 4);

        // We can free as long as there are no living references to the contents of the pool
        {
            let ptr = pool.push(&[1u8]).unwrap();
        }
        pool.free();

        // We cannot push a slice that would overflow the buffer
        assert!(pool.push(&[1; 4000]).is_some());
        assert!(pool.push(&[1; 97]).is_none());

        // We cannot push a slice that is too big
        pool.free();
        assert!(pool.push(&[1; 5000]).is_none());
    }
}
