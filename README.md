# rust-dis-logger
Distributed Interactive Simulation (DIS) logger for terminal.  
It creates a UDP socket, receives DIS packages and decodes them with the dis-rs crate.  
Usage:

```bash
dis-logger 127.0.0.1 3000
```

# Supported DIS Packet Data Units (PDU)

- [x] Start Resume
- [x] Stop Freeze
- [x] Fire
- [x] Detonation
- [x] Electromagnetic Emission
- [x] IFF

# Dependency
Crate clap for CLI https://crates.io/crates/clap  
Crate dis-rs https://crates.io/crates/dis-rs

# References
Distributed Interactive Simulation https://en.wikipedia.org/wiki/Distributed_Interactive_Simulation  
DIS Standard https://www.sisostds.org
