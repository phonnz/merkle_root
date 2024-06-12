# MerkleRoot Elixir implementation
This is a very simple usage from Elixir tools to find a root of a Merkle Tree
usgins Sha-256 and reading the base transaction hashes from an input.txt file

The main purpose to this implementation is to have a simple reference of time to 
process in order to write Rust code to improve the time.

## Installation

```elixir
def deps do
  [
    {:merkle_root, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). 

