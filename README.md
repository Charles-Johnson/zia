# zia
Library for the Zia programming language.

Zia is a symbolic programming language that aims to have a minimal set of built-in features yet maximal expressive power by having the most extensible and flexible syntax.

The current implementation exposes two functions that can be used in an interface:

pub fn memory_database() -> SqliteConnection {...}
pub fn oracle(buffer: &str, conn: &SqliteConnection) -> String {...}

memory_database initialises a database representing the knowledge of the program and returns a handle to that database.
oracle accepts an expression as input (buffer references this string) and processes the command given the knowledge in the database (conn references this database).

An expression consists of an applicand and an argument, separated by spaces. Either of these could be another expression nested in parentheses.

e.g.

(aplicand1 argument1) (applicand2 argument2)

If an applicand or argument is not an expression, it is an atom: labelled by a string containing no spaces or parentheses.

We can change the label of an atom from "a" to "b" by the expression:

(:= a) b

Ideally, we would write:

b := a

but that would require Zia to know the precedence of atoms to order the applications without superfluous parentheses. This feature hasn't been implemented yet.

We can also define "c" as the application of "a" and "b" by:

(:= (a b)) c

If you want to know what "c" consists of, you can expand it by:

c :=

Reduction rules for expressions can be defined. If "a" reduces to "b" then we can write:

(-> b) a

If we want to find the normal form of "a" then we can write:

a ->
