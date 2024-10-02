use std::str::FromStr;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Layer};

pub(crate) fn init(level: tracing::Level) -> anyhow::Result<()> {
    let layer_stderr = fmt::Layer::new()
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_file(false)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_filter(EnvFilter::from_str(level.as_str())?);

    tracing::subscriber::set_global_default(tracing_subscriber::registry().with(layer_stderr))?;
    Ok(())
}
