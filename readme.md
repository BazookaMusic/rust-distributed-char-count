# Distributed Char Count

This is a first rust project done with the generous help of chatgpt-3.5.
It can be tested by cloning, running cargo build and doing:
`cargo run -- -n_clients {the amount of clients you want to spawn}`

It will spawn the specified number of concurrent clients which will progressively count characters in their random text
and then they will send the result to the server which accumulates and prints it.

It is absolutely useless but a nice learning project to understand Websockets, concurrency and rust in general.