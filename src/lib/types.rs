use circular_buffer::CircularBuffer;

pub type Nonce = u64;
pub type Difficulty = u32;
pub type Hashrate = u64;
pub type Hash = Vec<u8>;

// hashrate_buf is useful for averaging hashrate
pub const HASHRATE_BUF_SIZE: usize = 10;
pub type HashrateBuf = CircularBuffer<HASHRATE_BUF_SIZE, Hashrate>;
pub type HashrateAvg = f32;
pub type WorkerId = u64;
