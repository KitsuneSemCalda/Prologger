% Defining facts
father(joseph, mary).  % Joseph is Mary's father
father(joseph, john).  % Joseph is John's father
mother(anne, mary).    % Anne is Mary's mother
mother(anne, john).    % Anne is John's mother

% Defining rules
sibling(X, Y) :- father(F, X), father(F, Y), mother(M, X), mother(M, Y), X \= Y.
sister(X, Y) :- sibling(X, Y), female(X).
female(mary).
male(john).
