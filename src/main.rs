use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use clap::{Parser, Subcommand}; 
use colored::*;
#[derive(Parser)]
#[command(name="PortScanner", author="Faisal Aldawsari", version="1.0", about = "a basic port scanner")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]enum Commands {
    /// it Scans an IP to check for any open Ports it has 3 args ip ,starting port,ending port
    Scan { 

        #[arg(default_value="127.0.0.1")]
        ip: IpAddr,
        #[arg(long,short,default_value_t=1)]
        start_port: u16,
        #[arg(long,short,default_value_t=100)]
        ending_port: u16,
    }
}

fn main() {
 
let logo = r#"
                                                                                                                               
,-.----.                                                                                                                       
\    /  \                         ___              .--.--.                                                                     
|   :    \                      ,--.'|_           /  /    '.                                                                   
|   |  .\ :   ,---.    __  ,-.  |  | :,'         |  :  /`. /                              ,---,      ,---,             __  ,-. 
.   :  |: |  '   ,'\ ,' ,'/ /|  :  : ' :         ;  |  |--`                           ,-+-. /  | ,-+-. /  |          ,' ,'/ /| 
|   |   \ : /   /   |'  | |' |.;__,'  /          |  :  ;_       ,---.     ,--.--.    ,--.'|'   |,--.'|'   |   ,---.  '  | |' | 
|   : .   /.   ; ,. :|  |   ,'|  |   |            \  \    `.   /     \   /       \  |   |  ,"' |   |  ,"' |  /     \ |  |   ,' 
;   | |`-' '   | |: :'  :  /  :__,'| :             `----.   \ /    / '  .--.  .-. | |   | /  | |   | /  | | /    /  |'  :  /   
|   | ;    '   | .; :|  | '     '  : |__           __ \  \  |.    ' /    \__\/: . . |   | |  | |   | |  | |.    ' / ||  | '    
:   ' |    |   :    |;  : |     |  | '.'|         /  /`--'  /'   ; :__   ," .--.; | |   | |  |/|   | |  |/ '   ;   /|;  : |    
:   : :     \   \  / |  , ;     ;  :    ;        '--'.     / '   | '.'| /  /  ,.  | |   | |--' |   | |--'  '   |  / ||  , ;    
|   | :      `----'   ---'      |  ,   /           `--'---'  |   :    :;  :   .'   \|   |/     |   |/      |   :    | ---'     
`---'.|                          ---`-'                       \   \  / |  ,     .-./'---'      '---'        \   \  /           
  `---`                                                        `----'   `--`---'                             `----'            
                                                                                                                               "#;
    println!("{}",logo.cyan());
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { ip, start_port, ending_port } => {
            let port_range = start_port..=ending_port; 
            let timeout = Duration::from_millis(200);
            
            println!("Scan on {} ..", ip);
            
            for port in port_range {
                let s_address = SocketAddr::new(ip, port);
                
                match TcpStream::connect_timeout(&s_address, timeout) {
                    Ok(_) => {
                        println!("Port {} is OPEN", port.to_string().green());
                    }
                    Err(_) => {
                        // Port is closed or timed out
                    }
                }
            }
            println!("Scan complete.");
        }
        
    }   
}