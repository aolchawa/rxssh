use std::{error::Error, io::Read, net::TcpStream};

use ssh2::Session;

// Static values
static SSH_PORT: u8 = 22;

pub fn execute_single(
    hostname: &str,
    username: &str,
    command: &str,
) -> Result<String, Box<dyn Error>> {
    let tcp = match TcpStream::connect(format!("{}:{}", hostname, SSH_PORT)) {
        Ok(tcp) => tcp,
        Err(err) => return Err(err.into()),
    };

    let mut session = match Session::new() {
        Ok(session) => session,
        Err(err) => return Err(err.into()),
    };

    session.set_tcp_stream(tcp);

    if let Err(err) = session.handshake() {
        return Err(err.into());
    }

    if let Err(err) = session.userauth_agent(username) {
        return Err(err.into());
    }

    let mut channel = match session.channel_session() {
        Ok(channel) => channel,
        Err(err) => return Err(err.into()),
    };

    if let Err(err) = channel.exec(command) {
        return Err(err.into());
    }

    let mut output = String::new();
    if let Err(err) = channel.read_to_string(&mut output) {
        return Err(err.into());
    }

    match channel.exit_status() {
        Ok(code) => exit_code_to_string(code, &mut output),
        Err(err) => return Err(err.into()),
    };

    Ok(output)
}

fn exit_code_to_string(code: i32, output: &mut String) {
    if code != 0 {
        output.clear();
        // TODO: Handle this by a switch on different code
        //       values and return a string interpretation
        output.push_str(&code.to_string());
    }
}
