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
$ cd merkle_root
$ cargo run 
```

Default input file is input.txt

## Output

The MerkleTree Root and the elapsed time to compute it. 


```shell
$ cargo run 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/merkle_root`
      MerkleRoot Hash: e7394afb40d7bf8debb3171f212b87e8a9ab2437bec55bf5d320cef4d6bfbe6b
```


