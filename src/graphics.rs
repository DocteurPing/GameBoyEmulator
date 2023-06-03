pub(crate) const VRAM_BEGIN: usize = 0x8000;
pub(crate) const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy,Clone)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

pub(crate) struct GPU{
    pub(crate) vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}