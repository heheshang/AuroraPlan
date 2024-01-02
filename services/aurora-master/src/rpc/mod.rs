use anyhow::Result;
use futures_util::SinkExt;
use lib_conifg::master_config::Settings;
use lib_remote::cmd::message::MessageCodec;
use tokio::net::{TcpListener, TcpStream};

use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing::{error, info};

use crate::processor::MasterMessageType;
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
            let (stream, _addr) = listener.accept().await?;
            info!("accept from {}", _addr);
            tokio::spawn(async move {
                if let Err(e) = Self::process(stream).await {
                    error!("process error: {:?}", e);
                }
            });
        }
    }

    async fn process(mut socket: TcpStream) -> Result<()> {
        // let mut framed_socket = Framed::new(socket, LinesCodec::new());
        let (read, write) = socket.split();
        let mut f_read = FramedRead::new(read, MessageCodec);
        let mut f_write = FramedWrite::new(write, MessageCodec);

        info!("process");

        while let Some(msg) = f_read.next().await {
            let mut msg = msg?;
            let message_type = msg.message_type;
            let master_message_type: MasterMessageType = message_type.into();
            master_message_type.process(&msg).await;
            info!("server receive {:?}", master_message_type);

            info!("server receive {:?}", msg);
            msg.id += 1;
            f_write.send(msg).await?;
            f_write.flush().await?;
        }

        // let mut buf = BytesMut::new();
        // while let Ok(n) = socket.read(&mut buf).await {
        //     if n == 0 {
        //         break;
        //     }
        //     info!("server receive {:?}", buf);
        //     let res = Message::default().decode(&mut buf)?.unwrap_or_default();
        //     info!("server receive {:?}", res);
        //     socket.write_all(&buf).await?;
        // }

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
