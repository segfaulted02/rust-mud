# Rust-Independent_Study

## Description

This game is a simple, multi-user dungeon, entirely written in Rust, for the purpose of learning and developing skills in the Rust language. By creating a functional and somewhat engaging video game, I argue it is the best way to learn Rust.

## How do I play?

Start the server by running this command in the rust_server directory:
```
cargo run
```

Connect to the server using netcat:
```
nc 127.0.0.1 5555
```

## In-game commands
```
interact <entity/object> #interact with entities/players or items
attack <entity> #attack an entity/player
move <direction> #moves into a different room
inventory #opens inventory
equip <item> #equips weapon
logout #exits the game
```
