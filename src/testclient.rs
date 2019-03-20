use std::{
    io::{
        Read,
        Write,
    },
    os::unix::net::UnixStream,
};
fn main()
{
    let mut conn = UnixStream::connect("/tmp/tombala.sock").unwrap();
    for i in 1..91
    {
        write!(conn, r#"{{"cmd":"new_ball","args":[{}]}}"#, i).unwrap();
        let mut data = [0; 100];
        let n = conn.read(&mut data);
        println!("{}", String::from_utf8_lossy(&data[..n.unwrap()]));
        #[allow(deprecated)]
        std::thread::sleep_ms(100);
    }
}
