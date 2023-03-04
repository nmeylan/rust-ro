![build](https://github.com/nmeylan/rust-ro/actions/workflows/rust.yml/badge.svg)
# What is rust-ro?
rust-ro is a Ragnarok mmo server implementation written in rust. While it is a from scratch implemention it is heavily inspired by [herculesWS](https://github.com/HerculesWS/Hercules) and [rathena](https://github.com/rathena/rathena), for example this implementation support same scripting language for NPC meaning that existing scripts should work on this implementation and use the same database structure than rathena and some naming convention were kept.

## Ambition
The project started on August 2021 with the ambition to being able to connect on the server and navigate across maps to see monsters.

Today February 2023 a lot of features have been added, see below. My ultimate goal would be to have a fully playable implementation for **PRE-RE**, supporting packet version **20120307**.

I am working on this project for fun and also to provide a more accessible way than existing implementation to understand how Ragnarok game works for educational purpose. Each feature is covered with tests to provide internal documentation.

## Packet version
The packet parser, builder and serializer are all taking packet version as parameter. The [packet db](https://github.com/nmeylan/rust-ro/blob/master/tools/packets/packets_db) also support condition around packet [attributes](https://github.com/nmeylan/rust-ro/blob/master/tools/packets/packets_db#L112) and [packet id](https://github.com/nmeylan/rust-ro/blob/master/tools/packets/packets_db#L423)

Although I mentioned above wanting to fully support packet version **20120307**, this implementation can support any packet version, it is just I am testing exclusively with a [robrowser client](https://github.com/MrAntares/roBrowserLegacy) using this packet version and thus i am implementing only packet in this version.


# Implementation details

Checkout [architectures notes](doc/Architecture.md)

# TODO
See [Meta issue](https://github.com/nmeylan/rust-ro/issues/19)

# Usage
*This section will be rewritten*

**Pre-requisite:**
- postgresql (rathena db was converted using pgloader and then adapted)
- RO db `db/pg.sql`

**Notes:**
- All packets for account 2000000 are handle by this project.
- All packets for any other account are proxied (and display in console) to hercules or rathena.


- clientinfo.xml to be changed to target port 6901

In proxy mode:
- login, char, map server to be running using default ports (6900, 6121, 6122)

Database:
- This project use postgresql instead of mysql to leverage some feature not fully provided by mysql (`ON CONFLICT (column) .. DO UPDATE`, `UNNEST(array)`, ...).
- Structure was copied from rathena database, some modification where done (like adding constraints)

# What has been done? ✔️
## Tools
- packet structure generator from [packet db](https://github.com/nmeylan/rust-ro/blob/master/tools/packets/packets_db)
- packet parser generator
- map cache generator
## Server
- proxy login, char and map request to hercules/rathena login, char and map servers
- packet debug
- login
- char server features(create char, delete char, join game)
- move character in a loaded map, synchronized with client side movement (no lag, or teleportation, movement is smooth)
- character position calculation (implementation of client side path finding)
- debug log in in-game chat 
- parse scripts (only warps and mobs at the moment)
- warp (change map, reset states)
- display scripts client side (only warps and mobs at the moment)
- visual debugger
- map instances (map are lazily loaded, an instance is created when a player join an non initialized map)
- mob spawn
- atcommand: @go, @warp
- mob move
- NPC scripts (partially: see https://github.com/nmeylan/rust-ro/issues/3) via [rathena script lang interpreter](https://github.com/nmeylan/rathena-script-lang-interpreter)
- basis for inventory management
- basis for skills
- basis for consumable item usage
- basis for player attacking mob
- mob drops item on death


## Compilation
A compilation of progress made so far

https://user-images.githubusercontent.com/1909074/210180140-0bb7e034-4209-4fb9-8601-a29cbb90cea5.mp4


## Integration of the VM (showing instance and class(npc) variable)

https://user-images.githubusercontent.com/1909074/178155321-d3eeb4b8-32ed-4901-bbfe-b101b1a5a56d.mp4

## Visual debugger
![visual-debugger](doc/img/visual_debugger.PNG)
Debug server state with a UI


## warp
![warps](doc/img/warp_spawn.PNG)
![warps](doc/img/warp.PNG)

## mob
![mobs](doc/img/mob_spawn.PNG)

## Proxied packets
![packets](doc/img/packet_analyzer.PNG)
