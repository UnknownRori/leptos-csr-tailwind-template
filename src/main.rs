use leptos::*;

// INFO : Make sure this thing correspond with project name in Cargo.toml
use leptos_csr_tailwind_template::App;

fn main() {
    // For better console log
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
