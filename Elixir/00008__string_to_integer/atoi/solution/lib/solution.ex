defmodule Solution do
  def scan(<<" " <> rest>>) do
    scan(rest)
  end

  def scan(<<"+" <> rest>>) do
    {:positive, scan_num(rest)}
  end

  def scan(<<"-" <> rest>>) do
    {:negative, scan_num(rest)}
  end

  def scan(s) do
    {:positive, scan_num(s)}
  end

  def is_numeral(<<c>>) when c >= ?0 and c <= ?9, do: true
  def is_numeral(_), do: false

  def scan_num(s), do: scan_num(s, [])

  def scan_num(<<>>, acc), do: acc

  def scan_num(<<char::binary-size(1), rest::binary>>, acc) do
    if is_numeral(char) do
      scan_num(rest, [char | acc])
    else
      acc
    end
  end

  def to_digit(<<"0">>), do: 0
  def to_digit(<<"1">>), do: 1
  def to_digit(<<"2">>), do: 2
  def to_digit(<<"3">>), do: 3
  def to_digit(<<"4">>), do: 4
  def to_digit(<<"5">>), do: 5
  def to_digit(<<"6">>), do: 6
  def to_digit(<<"7">>), do: 7
  def to_digit(<<"8">>), do: 8
  def to_digit(<<"9">>), do: 9

  def num_build(:positive, powers), do: num_build_powers(powers)
  def num_build(:negative, powers), do: -num_build_powers(powers)

  def num_build_powers(l), do: num_build_powers(l, 0, 0)
  def num_build_powers([], _, number), do: number

  def num_build_powers([n_char | rest], power_of_ten, number) do
    n = to_digit(n_char)
    num_build_powers(rest, power_of_ten + 1, number + n * 10 ** power_of_ten)
  end

  def clamp(n, min, _max) when n < min, do: min
  def clamp(n, _min, max) when n > max, do: max
  def clamp(n, _min, _max), do: n


  def my_atoi_noclamp(s) do
    {sign, powers} = scan(s)
    num_build(sign, powers)
  end

  @spec my_atoi(s :: String.t()) :: integer
  def my_atoi(s) do
    my_atoi_noclamp(s)
    |> clamp(-2**31, (2**31)-1)
  end
end
