# Merkle Tree Root

Spec v1 (2021-12-17)

The task is to compute the merkle tree root corresponding to a set of transactions as fast and efficiently as possible.

## Requirements

- The program must output the merkle root.
- Sha-256 must be used to perform the hashing at each step, you may use a library for this.
- The code must be tested and documented.

## Input

The supplied transactions are in their hashed form and are ordered. The input file has one hex-encoded transaction per line.


## Usage

```shell
$ cargo run <input_file.txt>
```

Default input file is input.txt

## Output

The MerkleTree Root and the elapsed time to compute it. 


```shell
$ cargo run 
    2024-06-10T15:14:47.318925Z  INFO  Failed  = 13
```

## Testing

```shell
$ cargo test 
```

## Method
This approach was thought to use a simple solution with Elixir in order to set the baseline of time to process. As the second step, iterations over Rust implementation were tested to a better result. Finally will be tested to distribute the Rust processes under an Erlang Beam Node distribution and see if that can decrease the time to process.

The theory of the implementation came from [wikipedia](https://en.wikipedia.org/wiki/Merkle_tree) page

[imh](https://en.wikipedia.org/wiki/Merkle_tree#/media/File:Hash_Tree.svg)

All the iterations and time to response where measured under an AMD Ryzen9 with 16 cores and 48GB RAM. 

|  Implementation | Description | Time to process |
|---|---|---|
| [Elixir](./merkle_root_ex)| Simple Mix application with MerkleTree lib   |  ~1476ms
| [Rust](./merkle_root) | Simple Cargo application using  |
| Distributed Rust | | 
