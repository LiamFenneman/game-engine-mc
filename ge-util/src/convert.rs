#[must_use]
pub fn three_to_one(x: u32, y: u32, z: u32, size: cgmath::Vector3<u32>) -> usize {
    return (x + y * size.x + z * size.x * size.y) as usize;
}

#[must_use]
pub fn one_to_three(index: usize, size: cgmath::Vector3<u32>) -> cgmath::Vector3<u32> {
    let x = index % size.x as usize;
    let y = (index / size.x as usize) % size.y as usize;
    let z = index / (size.x * size.y) as usize;
    return cgmath::vec3(
        u32::try_from(x).expect("index is out of bounds"),
        u32::try_from(y).expect("index is out of bounds"),
        u32::try_from(z).expect("index is out of bounds"),
    );
}
