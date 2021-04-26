pub const HEIGHT: u32 = 1080;
pub const WIDTH: u32 = 1440;
pub const SIZE: u32 = HEIGHT * WIDTH;
pub const SAMPLES: u64 = 100000000;
//thread count
pub const TCOUNT: u64 = 8;
//threads per sample
pub const SAMPLESPT: u64 = SAMPLES / TCOUNT;