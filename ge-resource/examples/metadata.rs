use ge_resource::block::BlockMeta;

fn main() {
    let rm = ge_resource::ResourceManager::default();

    let meta = BlockMeta::new([
        "grass_top".to_owned(),
        "dirt".to_owned(),
        "grass_side".to_owned(),
        "grass_side".to_owned(),
        "grass_side".to_owned(),
        "grass_side".to_owned(),
    ]);

    println!("DBG: {:?}", meta);
    rm.save_data("grass.ron", &meta).unwrap();
}
