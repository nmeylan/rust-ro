# rust-ro-hercules

An attempt to learn rust while toying with [herculesWS](https://github.com/HerculesWS/Hercules)

The goal is to rewrite some part of hercules on rust while forwarding client request to actual hercules if feature are not implemented in this project.

# What has been done?
- proxy login, char and map request to hercules login, char and map servers
- packet structure generator
- packet parser
- packet debug

![packets](img/packet_analyzer.PNG)
