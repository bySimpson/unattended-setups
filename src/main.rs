mod setup_manager;
mod setup;

fn main() {
    let mut manager = setup_manager::SetupManager::new(String::from("https://unattended-setups.thmr.at"));
    print!("{:#?}", manager.get_setups());
}
