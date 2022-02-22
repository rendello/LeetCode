-module(prop_test_submission).
-compile(export_all).

-include("../proper/include/proper.hrl").

%%% ===========================================================================

prop_is_valid_fuzz() ->
    ?FORALL(String, string(), begin
        _ = submission:is_valid(String)
        true.
    end).

% c(prop_test_submission).
% proper:quickcheck(prop_test_submission:prop_is_valid_fuzz(), {numtests, 1000}).
