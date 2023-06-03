use crate::WINDOW_DIMENSIONS;

pub(crate) const VRAM_BEGIN: usize = 0x8000;
pub(crate) const VRAM_END: usize = 0x9FFF;
pub(crate) const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy,Clone)]
pub(crate) enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
pub(crate) fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

pub(crate) struct GPU{
    pub(crate) vram: [u8; VRAM_SIZE],
    pub(crate) tile_set: [Tile; 384],
    pub canvas_buffer: [u32; WINDOW_DIMENSIONS[0] * WINDOW_DIMENSIONS[1] * 4],
}