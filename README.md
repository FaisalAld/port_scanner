```text
                                                                                                                               
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
                                                                                                                               
```

# port_scanner
A fast, lightweight port scanner written in **Rust**. This tool allows users to scan a range of TCP ports on a target IP address to identify open services.

## Prerequisites
* [Rust and Cargo](https://rustup.rs/) installed on your machine.

## Usage
* cargo run -- scan <IP_ADDRESS> -s <START_PORT> -e <END_PORT>

## Dependencies
* clap - For robust command-line argument parsing.
* colored - For terminal string styling.

## Roadmap

### Phase 1:
- (done) Basic TCP Connect Scanning logic 
- (done) CLI implementation               

### Phase 2:
- (WIP) Implementing **Asynchronous scanning** using "Tokio"
- (WIP) Adding a "Semaphore" to manage rate-limiting and prevent network congestion.
