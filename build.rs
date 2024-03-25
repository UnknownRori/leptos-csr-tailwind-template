// INFO : This is just for adding more version to the welcome page
// Remove it if you want to
fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");
}
