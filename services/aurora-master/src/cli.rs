use anyhow::Result;
use futures_util::SinkExt;
use lib_common::logger::setup_logger;
use lib_conifg::master_config::Settings;
use lib_remote::cmd::{
    message::{Message, MessageCodec},
    message_type,
};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, FramedWrite};
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;
    let settings = Settings::new()?;
    let listen_port = settings.master.listen_port;
    let _ = run(listen_port).await;
    // let mut stream = TcpStream::connect(format!("127.0.0.1:{}", listen_port)).await?;
    // let (reader, writer) = stream.split();
    // let mut f_reader = FramedRead::new(reader, Message::default());
    // let mut f_writer = FramedWrite::new(writer, Message::default());
    // f_writer.send(Message::default()).await;
    // let res  = f_reader.next().await;
    // info!("res: {:?}", res);
    // while let Some(v) = f_reader.next().await {
    //     let msg = v?;
    //     info!("msg: {:?}", msg);
    //     f_writer.send(msg).await?;
    //     f_writer.flush().await?;
    // }

    // let mut stream = Framed::new(stream, Message::default());
    // let msg = Message::builder().id(1).data(b"ping".to_vec()).opaque(2).build();
    // let buf = stream.write_buffer_mut();
    // let _ = Message::default().encode(msg.clone(), buf);

    // let mut count = 0;

    // loop {
    //     let n = stream.read_buffer_mut().len();
    //     if n == 0 {
    //         break;
    //     }
    //     count += n;

    // }

    Ok(())
}

async fn run(listen_port: u32) -> Result<()> {
    let mut count = 0;
    loop {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", listen_port)).await?;
        let (reader, writer) = stream.split();
        let mut f_reader = FramedRead::new(reader, MessageCodec);
        let mut f_writer = FramedWrite::new(writer, MessageCodec);
        let _ = f_writer
            .send(
                Message::builder()
                    .message_type(message_type::MessageType::CACHE_EXPIRE)
                    .id(count)
                    .opaque(2)
                    .data(b"ping".to_vec())
                    .build(),
            )
            .await;
        let res = f_reader.next().await;
        count += 1;
        info!("res: {:?}", res);
    }
}
