use cgmath::Vector3;

#[must_use]
pub const fn pos_to_idx(pos: Vector3<u32>, size: u32) -> usize {
    return (pos.y * size.pow(2) + pos.z * size + pos.x) as usize;
}

#[must_use]
pub const fn idx_to_pos(idx: usize, size: u32) -> Vector3<u32> {
    return Vector3::new(
        idx as u32 % size,
        idx as u32 / (size * size),
        (idx as u32 / size) % size,
    );
}
