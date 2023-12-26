use anyhow::Result;
use lib_conifg::master_config::Settings;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{error, info};
pub struct MasterRpcServer;
pub struct MasterRpcClient;

impl MasterRpcServer {
    pub async fn start() -> Result<()> {
        info!("start rpc server");
        let setting = Settings::new()?;
        // start rpc server
        let listen_port = setting.master.listen_port;

        info!("master  start  {}", listen_port);
        let listener = TcpListener::bind(format!("0.0.0.0:{}", listen_port)).await?;
        loop {
            let (socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                info!("accept socket: {:?}", socket);
                if let Err(e) = Self::process(socket).await {
                    error!("process error: {:?}", e);
                }
            });
        }
    }

    async fn process(socket: TcpStream) -> Result<()> {
        let mut framed_socket = Framed::new(socket, LinesCodec::new());
        info!("process");
        if let Some(line) = framed_socket.next().await {
            info!("receive line: {:?}", line);
        }

        Ok(())
    }
}

impl MasterRpcClient {
    pub async fn start() -> Result<()> {
        info!("start rpc client");
        // start rpc client
        Ok(())
    }
}
