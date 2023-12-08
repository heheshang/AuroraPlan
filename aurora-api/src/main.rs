mod ctx;
mod cypt;
mod model;
mod utils;
mod web;
use anyhow::Result;
use aurora_config::api_config::Settings;
use fern::colors::Color;
use fern::colors::ColoredLevelConfig;
use log::info;
use std::time::SystemTime;
use std::{env, net::SocketAddr};

fn main() -> Result<()> {
    let settings = Settings::new()?;
    let host = settings.server.host;
    let port = settings.server.port;
    let _ = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("aurora-api")
        // .thread_stack_size(32 * 100 * 1024 * 1024)
        .build()
        .unwrap()
        .block_on(start(host, port));
    Ok(())
}

async fn start(host: String, port: u32) -> Result<()> {
    // let backtrace = backtrace::Backtrace::new();
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "full");
    setup_logger()?;

    let route_all = web::route_all().await;
    let tcp_listener = tokio::net::TcpListener::bind(addr).await?;
    let make_service = route_all.into_make_service_with_connect_info::<SocketAddr>();
    axum::serve(tcp_listener, make_service).await?;

    info!("log init success!");
    info!("{:<12}->{}", "listen", addr);
    // println!("log init success! {:#?}", backtrace);
    Ok(())
}
// fn using_serve_dir() -> Router {
//     // serve the file in the "assets" directory under `/assets`
//     let path = get_ui_source_path();
//     match path {
//         Ok(p) => Router::new().route_service(
//             "/aurora/ui/*rest",
//             ServeDir::new(p.to_str().unwrap()).append_index_html_on_directories(true),
//         ),

//         Err(_) => Router::new(),
//     }
// }
fn setup_logger() -> Result<(), fern::InitError> {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(Color::White)
        .debug(Color::White)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(Color::BrightBlack);
    let colors_level = colors_line.info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date} {level} {target} {color_line}] {message}\x1B[0m",
                color_line = format_args!("\x1B[{}m", colors_line.get_color(&record.level()).to_fg_str()),
                date = humantime::format_rfc3339_seconds(SystemTime::now()),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        // set the default log level. to filter out verbose log messages from dependencies, set
        // this to Warn and overwrite the log level for your crate.
        .level(log::LevelFilter::Info)
        // change log levels for individual modules. Note: This looks for the record's target
        // field which defaults to the module path but can be overwritten with the `target`
        // parameter:
        // `info!(target="special_target", "This log message is about special_target");`
        .level_for("pretty_colored", log::LevelFilter::Trace)
        // output to stdout
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
