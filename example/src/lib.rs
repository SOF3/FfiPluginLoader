pocketmine::plugin!("example-plugin" version "0.2.0": Main);

struct Main;

impl pocketmine::Plugin for Main {
    fn init(api: pocketmine::Api) -> anyhow::Result<Self> {
        println!("Plugin initialized");
        Ok(Self)
    }
}
