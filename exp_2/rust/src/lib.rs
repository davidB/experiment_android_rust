use log::{error, info};

/// Expose the JNI interface for android below
//#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android;

#[cfg(target_os = "android")]
fn init_logger() {
    use android_logger::{Config, FilterBuilder};
    use log::Level;

    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Trace) // limit log level
            .with_tag("exp_2") // logs will show under mytag tag
            // .with_filter(
            //     // configure messages for specific crate
            //     FilterBuilder::new()
            //         .parse("debug,hello::crate=error")
            //         .build(),
            // ),
    );
}

#[cfg(not(target_os = "android"))]
fn init_logger() {
    pretty_env_logger::init();
}

fn main() {
    init_logger();
    info!("this is a verbose {}", "message");
    error!("this is printed by default");
}
