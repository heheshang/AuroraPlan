use anyhow::Result;
use lib_common::logger::setup_logger;
use lib_conifg::master_config::Settings;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;
    let settings = Settings::new()?;
    let listen_port = settings.master.listen_port;
    for _ in 0..100 {
        let addr = format!("{}:{}", "0.0.0.0", listen_port);
        let mut stream = TcpStream::connect(addr).await?;
        let result = stream.write_all(b"hello world\n").await;
        info!("wrote to stream; success={:?}", result.is_ok());
    }
    Ok(())
}
