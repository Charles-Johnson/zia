# `zia`: Interpreter for the Zia programming language.

The Zia project aims to develop a programming language that can be used to program itself. Each 
expression of Zia syntax either represents a unit of knowledge that the user provides to the system
or a request for knowledge that the system may have.

Expressions represent a binary tree where parentheses group a pair of expressions and a space 
separates a pair of expressions.

e.g.
```
(ll lr) (rl rr)
```    
represents the following binary tree:
```
    / \
   /   \
  /     \
 / \   / \
ll lr rl rr
```

The leaves of the tree can be any unicode string without a space or parentheses. These symbols may 
be recognised by the intepreter as concepts or if not used to label new concepts.

Currently, 4 types of low-level operations have been implemented using 2 of the built-in symbols.

Reduction symbol: `->`

`->` can be used to specify reduction rules for concepts given by expressions. For example
`a (-> b)` represents the command to specify the rule that the concept labelled by `a` reduces to 
the concept labelled by `b`.

`->` is also used to print the symbol of the normal form (recursive reduction) of a concept. For 
example `a ->` represents the command to print `b` in the above case of `a (-> b)` but `c ->` 
prints `c` because no reduction rule exists for `c`.

Reduction rules chain together. For example if `d (-> e)` and `e (-> f)` are executed then
executing `d ->` will print `f`.

You can modify existing reduction rules. For example you can change the reduction rule for `e` by 
`e (-> g)`; `e ->` will now print `g` and `d ->` also prints `g`. You could also execute `a (-> a)`
and so `a ->` now prints `a`.

The intepreter will let you know if reduction rule commands are redundant. For example `h (-> h)`
is redundant because all new concepts are by default their own normal form. Also `e (-> g)` is
redundant because it's already been explicitly specified. However `d (-> g)` would not be redundant 
because this changes the rule from "The normal form of `d` is the normal form of `e`" to "The 
normal form of `d` is the normal form of `g`" even though `d` already reduces to `g`.

Definition symbol: `:=`

`:=` can be used to label a binary tree of concepts or relabel a concept. For example 
`c (:= (a b))` means graphically:
```
 c
/ \
a b
```
The command `c :=` then prints `a b`. The command `a :=` prints `a`. We can change the symbol of
`b` to `h` using `b (:= h)`. `c :=` would then print `a h`.

To prevent infinite recursion, commands like `i (:= (i j))` are not accepted by the interpreter nor
are commands like `i (-> (i j))`.

API  

The current implementation exposes the `Context` and `AbstractSyntaxTree` type that can be used in an interface such as 
[IZia](https://github.com/Charles-Johnson/izia). Importing the following traits allows the corresponding methods to be called with `Context` (use `execute<AbstractSyntaxTree>`).

```
trait ContextMaker<T> {
	fn new() -> Self { 
		// Constructs a new Context with 3 built-in concepts: one to encode the labels of concepts
    	// (id=LABEL), one to encode commands to define or print the definitions of concepts (id = 
    	// DEFINE) and one to encode commands to define reduction rules or print the normal forms 
		// of concepts (id = REDUCTION).
    }
}

trait Execute<T> {
    fn execute<U>(&mut self, command: &str) -> String { 
		// Executes the commands given by the user. The command is converted into an abstract 
		// syntax tree using the labels of built-in concepts. This abstract syntax tree is then 
		// parsed and appropriate operations are performed.
	}
}
```
