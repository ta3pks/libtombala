use std::io::Read;
use std::net::TcpListener;
use std::sync::mpsc::{
    Receiver,
    SyncSender,
};
use std::sync::Arc;
pub fn start_server(receiver: SyncSender<String>, server_sent_command: Receiver<String>)
{
    let receiver = Arc::new(receiver);
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    loop
    {
        let mut s = match listener.accept()
        {
            Ok((s, _)) => s,
            Err(e) =>
            {
                println!("err {}", e);
                continue;
            }
        };
        let receiver = receiver.clone();
        let mut data: [u8; 10] = [0; 10];
        loop
        {
            let num = s.read(&mut data).unwrap();
            if num == 0
            {
                break;
            }
            receiver
                .send(String::from_utf8_lossy(&data[..num]).into())
                .expect("error");
        }
    }
}
