# 2048 AI

## Acknowledgements

Thanks to Gabriele Cirulli for creating the game 2048 and [open sourcing the code](https://github.com/gabrielecirulli/2048), it was used and adapted for the front end web code.

Thanks to Robert Xiao (nneonneo) for his work on developing an [expectimax to play 2048](https://github.com/nneonneo/2048-ai). The algorithms and data structures he developed were key for developing a fast implementation of the game engine.

## Usage
Rust with cargo needs to be installed. Build with:
```shell
cargo build --release
```

From the root directory the executable can be run:
```shell
./target/release/msc-2048-ai
```

By default this executable will run the iterated local search to find a strategy with 1 ban rule and 4 try rules.

It is possible to watch the best strategy found using this search by passing the --play argument.

```shell
./target/release/msc-2048-ai --play
```

You can change the time taken between moves by entering another argument after --play with the desired time in milliseconds.

```shell
./target/release/msc-2048-ai --play 100
```

## Structure

### benches/  
This folder contains benchmarking code used to help optimise the engine.

### pkg/
The pkg directory contains wasm code and js code created by wasm-pack. The code is used by the front end web code.

### src/
The src folder contains the majority of the code base. Most notably it includes the code for the engine and the strategies.

**engine.rs**  
This is the code for the game engine that is used by the rest of the library.

**engine_unsafe.rs**  
This is engine code that is used by wasm.

**main.rs**  
Defines what the executable does.

**wasm.rs**  
Defines the interface to use with js that compiling to wasm will produce.

#### ai/
A variety of different agents were implemented. The most important being the strategy agent.

**mod.rs**
Contains functions that can be used to run any agent and defines what interface and agent must have.

##### strategy/
**mod.rs**  
The mod file contains the definition for a strategy.

**attributes.rs**  
The attributes file contains all of the attribute functions used to convert the game state to a boolean.

**ban_rules.rs**  
Contains the definition for all of the possible ban rules.

**try_rules.rs**  
Contains the definition for all of the possible try rules.

**evaluate_strategies**  
Includes functions that are used to evaluate and compare strategies.

**generate_strategies**  
Has iterators that are used by brute force methods to generate all possible strategies, will not work if search space is too large.

**mann_whitney.rs**  
The code to execute the Mann-Whitney test. The [rustats](https://docs.rs/rustats/0.1.0/rustats/hypothesis_testings/index.html) library was inspiration for the implementation.

###### search/
This directory contains the code for all of the search methods.
