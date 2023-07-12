use dicom::ul::association::server::ServerAssociationOptions;
use std::net::TcpListener;

fn main() {
    run_scp();
}

fn run_scp() {
    let scp_options = ServerAssociationOptions::new()
        .with_abstract_syntax("1.2.840.10008.5.1.4.1.1.4")
        .ae_title("BrainKey_Server")
        .accept_any();
    let tcp_listener = TcpListener::bind("192.168.86.23:80").unwrap();
    let (stream, _address) = tcp_listener.accept().unwrap();
    scp_options.establish(stream).unwrap();

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client!");
            }
            Err(e) => { /* connection failed */ }
        }
    }
}

// set up tcp listener to receive dicom file from remote node.  need to look into receiving compressed dicoms and more abstract syntaxes.
// But this will work for now.