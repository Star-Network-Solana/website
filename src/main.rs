// STAR Network is a trust-first institutional project.
// All features must prioritize transparency, neutrality,
// and auditability over growth or automation.

use leptos::*;

mod app;
pub mod components;
pub mod pages;
use app::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
