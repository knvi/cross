use std::thread::sleep_ms;

use crate::prompt::prompts;
use colored::Colorize;
use indicatif::ProgressStyle;

pub fn handle_create_project() {
    // Get the project name
    let input = prompts::Input {
        message: " project name".into(),
        default: None,
        allow_empty: false,
    };

    // Must be unique, cross.sh/[name]
    let project_name = input.run().unwrap();

    // Check validity here.

    // (Optional) Get the project description
    let input = prompts::Input {
        message: format!(" {} project description", "(optional)".bright_black()).into(),
        default: None,
        allow_empty: true,
    };

    let project_description = input.run().unwrap();

    // (Optional) Get the admin's information
    // This will be used for team management in the future
    let input = prompts::Input {
        message: format!(" your username").into(),
        default: None,
        allow_empty: false,
    };

    let username = input.run().unwrap();

    // Get the admin's password
    let password = prompts::Secret {
        message: " your password".into(),
        error: None,
        allow_empty: false,
        confirm: None,
    };

    let password = password.run().unwrap();

    // 2 factor-authentication code
    let input = prompts::Input {
        message: format!(" 2FA code {}", "(6 digits)".bright_black()).into(),
        default: None,
        allow_empty: false,
    };

    let two_factor_code = input.run().unwrap();

    let progress_bar = indicatif::ProgressBar::new_spinner();

    progress_bar
        .set_style(ProgressStyle::default_spinner().template("[{elapsed}] {spinner} {msg}"));
    progress_bar.enable_steady_tick(10);

    progress_bar.set_message("🔒 authenticating");

    sleep_ms(1000);

    progress_bar.set_message("🚀 creating project");
    sleep_ms(500);

    progress_bar.finish_and_clear();

    println!(
        "🎉 successfully created {}",
        project_name.to_lowercase().bright_green()
    );

    println!(
        "🔗 {}/{} {}",
        "https://cross.sh",
        project_name.to_lowercase().bright_green(),
        "[copied to clipboard]".bright_black()
    );
}
