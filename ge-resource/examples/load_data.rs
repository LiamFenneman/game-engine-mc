#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Example {
    boolean: bool,
    float: f32,
}

fn main() {
    let rm = ge_resource::ResourceManager::default();
    let example: Example = rm.load_data("example.ron").unwrap();
    println!("RON: {}", ron::to_string(&example).unwrap());
    println!("DBG: {:?}", example);
}
