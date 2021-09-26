# rust-ro-hercules

An attempt to learn rust while toying with [herculesWS](https://github.com/HerculesWS/Hercules)

The goal is to rewrite some part of hercules on rust while forwarding client request to actual hercules if feature are not implemented in this project.

# What has been done?
- proxy login, char and map request to hercules login, char and map servers
- packet structure generator from [packet db](https://github.com/nmeylan/rust-ro-hercules/blob/master/server/tools/packets/packets_db)
- packet parser
- packet debug
- login
- display char list
- create char
- delete char
- join game
- move in loaded map
- debug log in in-game chat 


![packets](img/packet_analyzer.PNG)

![packets](img/in_game_debug.PNG)