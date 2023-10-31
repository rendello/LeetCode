defmodule SolutionTest do
  use ExUnit.Case
  use PropCheck

  doctest Solution

  defp space, do: " "

  defp sign do
    oneof(["+", "-", ""])
  end

  defp whitespace do
    let l <- list(space()) do
      to_string(l)
    end
  end

  defp numeral do
    oneof(~c"0123456789")
  end

  defp non_numeral do
    oneof(
      ~c"\\\"!#$%&'()*+,-./:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]^_`abcdefghijklmnopqrstuvwxyz{|}~ \t\n"
    )
  end

  defp number_str do
    let l <- non_empty(list(numeral())) do
      to_string(l)
    end
  end

  defp garbage() do
    let l <- list(non_numeral()) do
      to_string(l)
    end
  end

  defp test_str() do
    let l <- [whitespace(), sign(), number_str(), garbage()] do
      Enum.join(l, "")
    end
  end

  property "my_atoi_noclamp() matches standard library function behaviour", numtests: 100_000 do
    forall t <- test_str() do
      {correct_n, _} = Integer.parse(String.trim(t))
      my_n = Solution.my_atoi_noclamp(t)
      correct_n == my_n
    end
  end

  test "my_atoi() clamps" do
    assert Solution.my_atoi("2147483647") == 2147483647
    assert Solution.my_atoi("2147483648") == 2147483647
    assert Solution.my_atoi("2147483649") == 2147483647
    assert Solution.my_atoi("-2147483648") == -2147483648
    assert Solution.my_atoi("-2147483649") == -2147483648
  end
end
