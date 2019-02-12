#![allow(unused_must_use)]
use crate::command_parser::{
    Command,
    COMMAND,
};
use std::error::Error;
use std::io::{
    Read,
    Write,
};
use std::os::unix::net::{
    UnixListener,
    UnixStream,
};
use std::rc::Rc;
use std::sync::mpsc::{
    Receiver,
    SyncSender,
};
pub fn start_server(
    cmd: SyncSender<COMMAND>,
    msg: Receiver<(
        COMMAND,
        bool,
        Result<Option<crate::core::types::Winning>, String>,
    )>,
)
{
    std::fs::remove_file("/tmp/tombala.sock");
    let listener = UnixListener::bind("/tmp/tombala.sock").unwrap();
    let cmd = Rc::new(cmd);
    let msg = Rc::new(msg);
    loop
    {
        let s = match listener.accept()
        {
            Ok((s, _)) => s,
            Err(e) =>
            {
                println!("err {}", e);
                continue;
            }
        };
        //handle only on econnection at a given time
        handle_connection(s, cmd.clone(), msg.clone());
    }
}

fn handle_connection(
    c: UnixStream,
    command: Rc<SyncSender<COMMAND>>,
    msg: Rc<
        Receiver<(
            COMMAND,
            bool,
            Result<Option<crate::core::types::Winning>, String>,
        )>,
    >,
)
{
    const MAX_DATA_LEN: usize = 1024; //max data size in bytes
    let mut c = c;
    loop
    // for every command
    {
        let mut buffer = [0; MAX_DATA_LEN];
        let read = c.read(&mut buffer);
        let num_read = if let Ok(n) = read { n } else { break };
        if num_read == 0
        {
            break;
        }
        // let cmd: Result<Command, serde_json::Error> = serde_json::from_reader(c);
        let str_val = String::from_utf8_lossy(&buffer[..num_read]).into_owned();
        let val = str_val.trim_end().to_string();
        let cmd = serde_json::from_str(val.as_str());
        // check if command is valid
        let cmd: Command = if let Ok(cmd) = cmd
        {
            cmd
        }
        else
        {
            serde_json::to_writer(
                c,
                &Command {
                    error: Some(String::from("invalid cmd json")),
                    ..Default::default()
                },
            );
            break;
        };

        if let Err(e) = command.send(cmd.into())
        // send command to the actual game
        {
            serde_json::to_writer(
                c,
                &Command {
                    error: Some(format!("something went wrong: {}", e.description())),
                    ..Default::default()
                },
            );
            break;
        };

        match msg.recv() // wait reply from the game
        {
            Ok((cmd, terminate,winning)) =>
            {
                let mut cmd:Command =cmd.into() ;
                let winning = if winning.is_ok(){
                    winning.ok().unwrap()
                }else{

                    println!("err here"); 
                    serde_json::to_string(&Command{
                        error:Some(winning.err().unwrap()),
                        ..Default::default()
                    }).and_then(|val|{
                        c.write_all(val.as_bytes());
                        Ok(())
                    });
                    if !terminate{
                        continue
                    }
                    break
                };
                cmd.winning=winning;
                serde_json::to_string(&cmd).and_then(|val|{
                    c.write_all(val.as_bytes());
                    Ok(())
                });
                if terminate
                {
                    break;
                }
            }
            Err(e) =>
            {
                send_error(c, e);
                break;
            }
        }
    }
}

fn send_error<T, E>(writer: T, err: E)
where
    T: std::io::Write,
    E: Error,
{
    serde_json::to_writer(
        writer,
        &Command {
            error: Some(err.description().to_string()),
            ..Default::default()
        },
    );
}
