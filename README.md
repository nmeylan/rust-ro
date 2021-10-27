# rust-ro
An attempt to learn rust while toying with creation of a mmorpg server. Inspired by [herculesWS](https://github.com/HerculesWS/Hercules) and [rathena](https://github.com/rathena/rathena)

# Pre-requisite
- mysql
- RO db (loaded via hercules or rathena scripts)

# Usage
- All packets for account 2000000 are handle by this project.
- All packets for any other account are proxied (and display in console) to hercules or rathena.


- clientinfo.xml to be changed to target port 6901

In proxy mode:
- login, char, map server to be running using default ports (6900, 6121, 6122)

# What has been done?
- proxy login, char and map request to hercules login, char and map servers
- packet structure generator from [packet db](https://github.com/nmeylan/rust-ro-hercules/blob/master/server/tools/packets/packets_db)
- packet parser
- packet debug
- login
- char server features(create char, delete char, join game)
- move character in a loaded map, synchronized with client side movement (no lag, or teleportation, movement is smooth)
- character position calculation (implementation of client side path finding)
- debug log in in-game chat 
- parse scripts (only warps and mobs at the moment)
- warp (change map, reset states)
- display scripts client side (only warps and mobs at the moment)

# Wip
- Map instances + mob spawn
- mob move

## warp
![packets](img/warp_spawn.PNG)
## mob
![packets](img/mob_spawn.PNG)

## Proxied packets
![packets](img/packet_analyzer.PNG)

## In game debug
![packets](img/in_game_debug.PNG)
