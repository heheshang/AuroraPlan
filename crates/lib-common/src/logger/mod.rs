// #[cfg(feature = "log_rs")]
// use std::time::SystemTime;

// #[cfg(feature = "log_rs")]
// use fern::colors::{Color, ColoredLevelConfig};

// #[cfg(feature = "log_rs")]
// pub fn setup_logger() -> Result<(), fern::InitError> {
//     let colors_line = ColoredLevelConfig::new()
//         .error(Color::Red)
//         .warn(Color::Yellow)
//         // we actually don't need to specify the color for debug and info, they are white by default
//         .info(Color::White)
//         .debug(Color::White)
//         // depending on the terminals color scheme, this is the same as the background color
//         .trace(Color::BrightBlack);
//     let colors_level = colors_line.info(Color::Green);

//     fern::Dispatch::new()
//         .format(move |out, message, record| {
//             out.finish(format_args!(
//                 "{color_line}[{date} {level} {target} {color_line}:{line}] {message}\x1B[0m",
//                 color_line = format_args!("\x1B[{}m", colors_line.get_color(&record.level()).to_fg_str()),
//                 date = humantime::format_rfc3339_seconds(SystemTime::now()),
//                 target = record.target(),
//                 line = record.line().unwrap_or(0),
//                 level = colors_level.color(record.level()),
//                 message = message,
//             ));
//         })
//         // set the default log level. to filter out verbose log messages from dependencies, set
//         // this to Warn and overwrite the log level for your crate.
//         .level(log::LevelFilter::Info)
//         // change log levels for individual modules. Note: This looks for the record's target
//         // field which defaults to the module path but can be overwritten with the `target`
//         // parameter:
//         // `info!(target="special_target", "This log message is about special_target");`
//         .level_for("pretty_colored", log::LevelFilter::Trace)
//         // output to stdout
//         .chain(std::io::stdout())
//         .apply()?;
//     Ok(())
// }
#[cfg(feature = "otel")]
// To be able to set the XrayPropagator
use opentelemetry::global;
#[cfg(feature = "otel")]
// To configure certain options such as sampling rate
use opentelemetry::sdk::trace as sdktrace;
#[cfg(feature = "otel")]
// For passing along the same XrayId across services
use opentelemetry_aws::trace::XrayPropagator;
#[cfg(feature = "otel")]
// The `Ext` traits are to allow the Registry to accept the
// OpenTelemetry-specific types (such as `OpenTelemetryLayer`)
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, util::TryInitError, EnvFilter};
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
#[cfg(not(feature = "otel"))]
pub fn setup_logger() -> Result<()> {
    // See https://docs.rs/tracing for more info
    tracing_subscriber::fmt::try_init()
}

#[cfg(feature = "otel")]
pub fn setup_logger() -> std::result::Result<(), TryInitError> {
    println!("Setting up logger");
    // Set the global propagator to X-Ray propagator
    // Note: If you need to pass the x-amzn-trace-id across services in the same trace,
    // you will need this line. However, this requires additional code not pictured here.
    // For a full example using hyper, see:
    // https://github.com/open-telemetry/opentelemetry-rust/blob/main/examples/aws-xray/src/server.rs#L14-L26
    global::set_text_map_propagator(XrayPropagator::default());

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(
            sdktrace::config()
                .with_sampler(sdktrace::Sampler::AlwaysOn)
                // Needed in order to convert the trace IDs into an Xray-compatible format
                .with_id_generator(sdktrace::XrayIdGenerator::default()),
        )
        .install_simple()
        .expect("Unable to initialize OtlpPipeline");

    // Create a tracing layer with the configured tracer
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Parse an `EnvFilter` configuration from the `RUST_LOG`
    // environment variable.
    let filter = EnvFilter::from_default_env();

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(filter)
        .with(fmt::Layer::default())
        .try_init()
}
