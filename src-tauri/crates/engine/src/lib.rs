pub fn greet() -> String {
    let name = db::get_username();
    format!("Hello, {}! You've been greeted from Rust!", name)
}
