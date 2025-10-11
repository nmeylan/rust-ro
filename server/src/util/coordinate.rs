#[inline]
pub fn get_cell_index_of(x: u16, y: u16, x_size: u16) -> usize {
    (x as u32 + y as u32 * x_size as u32) as usize
}

#[inline]
pub fn get_pos_of(index: u32, x_size: u16) -> (u16, u16) {
    let y: u16 = (index / x_size as u32) as u16;
    let x: u16 = (index - (y as u32 * (x_size) as u32)) as u16;
    (x, y)
}
