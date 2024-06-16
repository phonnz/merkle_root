defmodule MerkleRootTest do
  use ExUnit.Case
  doctest MerkleRoot

  test "Find the merkle root" do
    assert MerkleRoot.run() ==
             {:ok, "94ded0d9e91293268958fd43ff0e817dbb81651f0fc51a4aa7e6c51d2405431b"}
  "77d519a56a3bb197bca02ed25f880a122487914556d587588e633c8368d13053"
"915961583d426ff5d6726ee59ff7e1ad234d8343f60c57ab023b21741fdba723end"
end
