mod setup;
mod setup_manager;

fn main() {
    let mut manager =
        setup_manager::SetupManager::new(String::from("https://unattended-setups.thmr.at"));
    print!("{:#?}", manager.get_setups());
}
