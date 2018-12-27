#!/usr/bin/env swipl

:- set_prolog_flag(double_quotes, chars).
:- set_prolog_stack(global, limit(100 000 000 000)).


%% making an exec ...

:- initialization(main, main).

main(Argv) :-
    reduce(Argv,Rs),
    length(Rs,Len),
    format('~w', Len).

%% solution impl ...

reducible(X,Y) :- upcase_atom(X,U), upcase_atom(Y,U), X \= Y.

reduce(Xs,Xs) :- reduce_step(Xs,Xs).
reduce(Xs,Rs) :- reduce_step(Xs,Ys), reduce(Ys,Rs).

reduce_step([],[]).
reduce_step([X],[X]).
reduce_step([X,Y|Xs],Rs) :- reducible(X,Y), reduce_step(Xs,Rs).
reduce_step([X,Y|Xs],[X|Rs]) :- \+ reducible(X,Y), reduce_step([Y|Xs],Rs).

