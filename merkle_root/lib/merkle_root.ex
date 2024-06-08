defmodule MerkleRoot do
  @moduledoc """
  Documentation for `MerkleRoot`.
  """

  @doc """
  MerkleRoot
  Finding the Merkle root from a provided
  hashs array,this is using merkle_tree hex package.
  ## Examples

      iex> MerkleRoot.root()
      :world

  """
  @path "../input.txt"
  def run do
    start = Time.utc_now()

    tree =
      @path
      |> File.stream!()
      |> Enum.map(&clean_row/1)
      |> MerkleTree.new()

    finished = Time.utc_now()

    time_to_process = Time.diff(finished, start, :millisecond)
    IO.inspect(tree.hash_function())

    IO.puts(
      "Elpased time to merkle root using Elixir and merkle_tree implementation : #{time_to_process}"
    )
  end

  defp clean_row(row) do
    String.trim_trailing(row, "\n")
  end
end
