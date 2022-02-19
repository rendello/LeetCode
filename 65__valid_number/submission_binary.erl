-module(submission_binary). %% Omit on Leetcode.

-compile({no_auto_import,[is_number/1]}).
-compile(export_all).

-spec is_number(S :: unicode:unicode_binary()) -> boolean().
is_number(S) ->
    case number(S) of
        {match, <<>>} -> true;
        {match, _}  -> false;
        nomatch     -> false
    end.

sign(S)  -> parse_char(S, <<"+-">>).
e(S)     -> parse_char(S, <<"Ee">>).
dot(S)   -> parse_char(S, <<".">>).
digit(S) -> parse_char(S, <<"0123456789">>).

digits(S) -> one_or_more(S, digit).

integer(S) -> all(S, [optional_sign, digits]).

decimal_without_sign(S) ->
    union_all(S,
        [
            [digits, dot, digits],
            [digits, dot        ],
            [        dot, digits]]).
decimal(S) -> all(S, [optional_sign, decimal_without_sign]).

exponent(S) -> all(S, [e, integer]).

number_without_exponent(S) -> union(S, [decimal, integer]).
number(S) -> all(S, [number_without_exponent, optional_exponent]).

optional_sign(S) -> optional(S, sign).
optional_exponent(S) -> optional(S, exponent).


parse_char(_, <<>>) ->
    nomatch;
parse_char(<<>>, _) ->
    nomatch;
parse_char(S, <<CH,CT/binary>>) ->
    <<SH,ST/binary>> = S,
    case SH == CH of
        true  -> {match, ST};
        false -> parse_char(S, CT)
    end.

union(_, []) ->
    nomatch;
union([], _) ->
    nomatch;
union(S, [P|PT]) ->
    case fun ?MODULE:P/1(S) of
        {match, S2} -> {match, S2};
        nomatch     -> union(S, PT)
    end.

optional([], _) ->
    {match, []};
optional(S, P) ->
    case fun ?MODULE:P/1(S) of
        {match, S2} -> {match, S2};
        nomatch     -> {match, S}
    end.
    
all(S, []) ->
    {match, S};
all(S, [P|PT]) ->
    case fun ?MODULE:P/1(S) of
        {match, S2} -> all(S2, PT);
        nomatch     -> nomatch
    end.

one_or_more([], _) ->
    nomatch;
one_or_more(S, P) ->
    case fun ?MODULE:P/1(S) of
        {match, S2} -> one_or_more_continued(S2, P);
        nomatch     -> nomatch
    end.

one_or_more_continued([], _) ->
    {match, []};
one_or_more_continued(S, P) ->
    case fun ?MODULE:P/1(S) of
        {match, S2} -> one_or_more_continued(S2, P);
        nomatch     -> {match, S}
    end.


union_all(_, []) ->
    nomatch;
union_all([], _) ->
    {match, []};
union_all(S, [PList|PListT]) ->
    case all(S, PList) of
        {match, S2} -> {match, S2};
        nomatch     -> union_all(S, PListT)
    end.
