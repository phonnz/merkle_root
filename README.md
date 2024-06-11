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
