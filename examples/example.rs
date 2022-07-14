// Helpful source: https://towardsdatascience.com/understand-your-comp&uter-system-using-logs-98139d0b5de1
use bitcoind_log_parser;
use std::fs;
use bitcoind_log_parser::types::*;

fn main() {
    let file_contents =
        fs::read_to_string("../../../Desktop/bitcoind-log.txt").expect("failed to open file");
    let lines: Vec<&str> = file_contents.lines().collect();

    for line in lines {
        let log_line = bitcoind_log_parser::parse_log_line(line);
        // Creating another copy just so I can print it out later without the compiler yelling at
        // me :(
        let log_line2 = bitcoind_log_parser::parse_log_line(line);
        match log_line.message_container.message {
            BitcoindLogMessage::NewOutboundPeerConnected(_) => {
                println!("{:#?}", log_line2);
            }
            BitcoindLogMessage::Unknown { raw } => {
                println!("UNKNOW_TYPE: [{}]", raw);
            }
            _ => {
                panic!("DONT KNOW THIS KIND")
            }
        }

    }
}
