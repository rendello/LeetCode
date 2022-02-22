-spec valid_utf8(Data :: [integer()]) -> boolean().
valid_utf8(Data) ->
    f(list_to_binary(Data)).

f(<<0:1, _:7, T/bitstring>>) ->f(T);
f(<<2#110:3, _:5, 2#10:2, _:6, T/bitstring>>) -> f(T);
f(<<2#1110:4, _:4, 2#10:2, _:6, 2#10:2, _:6, T/bitstring>>) -> f(T);
f(<<2#11110:5, _:3, 2#10:2, _:6, 2#10:2, _:6, 2#10:2, _:6, T/bitstring>>) -> f(T);
f(<<>>) -> true;
f(_) -> false.
