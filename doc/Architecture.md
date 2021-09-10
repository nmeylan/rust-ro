# Architecture

Hercules is composed on 3 servers:
- Login: to handle everything related to user authentication
- Char: to handle everything related to characters selection, but also party, guild and pet
- Map: to handle the rest of the game logic. It is the core of the server.

I guess that the split of these servers was done for scaling reason. However, it seems that having more than 1 map server is considered dangerous, certainly due to state synchronisation.
Client is expecting to interact with these 3 interfaces.

To simplify my implementation I won't split the 3 servers. 
Even if I don't expect my server to be used by anyone, I will try to design and implement my server to be able to load balance clients requests in different server instances.