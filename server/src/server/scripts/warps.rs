use accessor::Setters;

#[derive(Setters)]
struct Warp {
    #[set]
    map_name: String,
    x: u16,
    y: u16,
    x_size: u16,
    y_size: u16,
    dest_map_name: String,
    to_x: u16,
    to_y: u16,
}