-spec rearrange_array(Nums :: [integer()]) -> [integer()].
rearrange_array(Nums) ->
    {Pos, Neg} = lists:partition(fun(N) ->  N > 0 end, Nums),
    merge(Pos, Neg).

merge(L1, L2) ->
    merge(L1, L2, []).

merge([H1|T1], [H2|T2], Acc) ->
    merge(T1, T2, [H2, H1] ++ Acc);
merge([], [], Acc) ->
    lists:reverse(Acc).
