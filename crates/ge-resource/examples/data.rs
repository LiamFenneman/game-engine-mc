#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Example {
    boolean: bool,
    float: f32,
}

fn main() {
    let rm = ge_resource::ResourceManager::default();

    let mut example = Example {
        boolean: true,
        float: 1.5,
    };

    println!("DBG: {:?}", example);

    rm.save_data("example.ron", &example).unwrap();
    example = rm.load_data("example.ron").unwrap();

    println!("RON: {}", ron::to_string(&example).unwrap());
}
