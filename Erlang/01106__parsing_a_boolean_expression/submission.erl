-spec parse_bool_expr(Expression :: unicode:unicode_binary()) -> boolean().
parse_bool_expr(Expression) ->
    p(Expression, [], []).

p(<<>>, [[Value]], []) ->
    Value;
p(Text, [], []) ->
    %% Push initial frame onto the stack, if needed.
    p(Text, [[]], []);
p(<<Opcode,$(,Rest/binary>>, Stack, Ops) ->
    %% Push a new frame onto the stack.
    p(Rest,[[]|Stack],[Opcode|Ops]);
p(<<$t,Rest/binary>>, [Frame|StackRest], Ops) ->
    p(Rest, [[true|Frame]|StackRest], Ops);
p(<<$f,Rest/binary>>, [Frame|StackRest], Ops) ->
    p(Rest, [[false|Frame]|StackRest], Ops);
p(<<$,,Rest/binary>>, Stack, Ops) ->
    p(Rest, Stack, Ops);
p(<<$),Rest/binary>>, [Frame|StackRest], Ops) ->
    [Opcode|OpsRest] = Ops,
    Result = case Opcode of
        $! -> [V] = Frame, not V;
        $& -> lists:all(fun(Z) -> Z==true end, Frame);
        $| -> lists:member(true, Frame)
    end,
    [SiblingFrame|NewStackRest] = StackRest,
    NewStack = [[Result|SiblingFrame]|NewStackRest], 
    p(Rest, NewStack, OpsRest).
