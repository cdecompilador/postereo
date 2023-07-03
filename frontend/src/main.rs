use leptos::*;
use anyhow::*;

use frontend::App;

pub fn main() -> Result<()> {
    // Panics report to console.error
    console_error_panic_hook::set_once();

    // Logging goes to console.log
    console_log::init()
        .map_err(|e| anyhow!(e))?;

    log::info!("Hello");

    // Leptos entry
    mount_to_body(|cx| {
        view! { cx,
            <App />
        }
    });

    Ok(())
}
