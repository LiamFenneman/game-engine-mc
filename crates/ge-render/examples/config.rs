use ge_resource::ResourceManager;
use ge_util::EngineConfig;

fn main() {
    let rm = ResourceManager::default();
    rm.save_config("engine.toml", &EngineConfig::default()).unwrap();
}
