defmodule MerkleRoot do
  @moduledoc """
  Documentation for `MerkleRoot`.
  """

  @path "../input.txt"

  @doc """
  MerkleRoot
  Finding the Merkle root from a provided
  hashs array,this is using merkle_tree hex package.
  ## Examples

      iex> MerkleRoot.run()
      {:ok, "94ded0d9e91293268958fd43ff0e817dbb81651f0fc51a4aa7e6c51d2405431b"}
  """
  def run do
    start = Time.utc_now()

    tree =
      @path
      |> File.stream!()
      |> Enum.map(&clean_row/1)
      |> MerkleTree.new()

    finished = Time.utc_now()

    time_to_process = Time.diff(finished, start, :millisecond)

    IO.puts("""
    Elapsed time to merkle root using Elixir and merkle_tree implementation:
    #{time_to_process} ms 
    for root: 
    #{tree.root.value}

    """)

    tree
  end

  defp clean_row(row) do
    String.trim_trailing(row, "\n")
  end
end
