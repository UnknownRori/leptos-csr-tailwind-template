mod composable;
mod pages;
mod shared;

use leptos::*;
use leptos_router::*;

use pages::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Welcome/>
                <Route path="/*any" view=NotFound/>
            </Routes>
        </Router>
    }
}

// INFO : This is just for adding more version to the welcome page
// Remove it if you want to
mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));

    pub fn get_leptos_version() -> &'static str {
        for (name, version) in DIRECT_DEPENDENCIES {
            if name == "leptos" {
                return version;
            }
        }

        return "Failed to fetch version";
    }

    pub fn get_rustc_version() -> &'static str {
        RUSTC_VERSION
    }
}
