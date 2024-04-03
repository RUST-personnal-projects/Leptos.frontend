use leptos::*;

fn main() {
    console_log::init_with_level(log::Level::Trace).expect("error initializing logger");
    console_error_panic_hook::set_once();

    log::trace!("trace");
    log::debug!("debug");
    log::info!("info");
    log::warn!("warn");
    log::error!("error");

    mount_to_body(|| view! { <p>"Hello, world! On githubpage"</p> })
}
