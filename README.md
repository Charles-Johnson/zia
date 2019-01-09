# `zia`: Interpreter for the Zia programming language.

The Zia project aims to develop a programming language that can be used to program the language 
itself. Instead of storing the source code as plain text and editing the raw text (which can 
easily break the program), the runtime environment of the interpreter (the `Context`) can be saved
to disk and used in other programs. All the programming is done using an interactive shell such as
[IZia](https://github.com/Charles-Johnson/izia). The commands sent are interpreted based on the `Context`. They are used to incrementally modify, test and debug the `Context`.  

Expressions for Zia commands represent a binary tree where parentheses group a pair of expressions and a space separates a pair of expressions.

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

Currently, 4 types of low-level operations have been implemented using 3 of the built-in symbols.

Let symbol: `let`

`let` is used to set a relation as true. It can be further explained in its use below.

Reduction symbol: `->`

`->` can be used to specify reduction rules for concepts given by expressions. `let (a (-> b))`
 represents the command to specify the rule that the concept labelled by `a` reduces to the 
concept labelled by `b`.

`->` is also used to print the symbol of the reduction of a concept. `a ->` represents 
the command to print `b` because of the previous command but `b ->` prints `b` because no 
reduction rule exists for `b`.

You can modify existing reduction rules. For example you can change the reduction rule for `a` by 
`let (a (-> c))`; `a ->` will now print `c`. You could also execute `let (a (-> a))` and so `a ->` 
now prints `a`.

The intepreter will let you know if reduction rule commands are redundant. `let (a (-> a))` is 
redundant because `a` already reduces to itself. Executing `let (a (-> b))` twice will print an 
error for the second time.

Definition symbol: `:=`

`:=` can be used to label a binary tree of concepts or relabel a concept. For example 
`let (c (:= (a b)))` means graphically:
```
 c
/ \
a b
```
The command `c :=` then prints `a b`. The command `a :=` prints `a`. We can change the symbol of
`b` to `e` using `let (e (:= b))`. `c :=` would then print `a e` and `a ->` would print `e`. 
Because `a` reduces to `e`, `c ->` prints `e e`. If `let (c (-> f))` is executed, an error message 
is printed explaining some of the components of `c` (`a` and `e`) reduce. This would break the 
coherence between the reduction of a concept and its composition's reductions. Should `c` reduce 
to `f` or `e e`? Maintaining this coherence is important for a consistent lazy reduction of syntax 
trees.

To make sure concepts can be fully reduced, commands like `let (i (:= (i j)))` are not 
accepted by the interpreter nor are commands like `let (i (-> (i j)))`. More subtley 
`let (e (:= (a d)))` is not accepted because the reduction of `a d` is `e d` and so `e ->` prints
`e d`, `(e d) ->` prints `(e d) d` etc. Successive reductions can always be applied. 

API  

The current implementation exposes the `Context` type that can be used in an interface such as 
[IZia](https://github.com/Charles-Johnson/izia). Importing the following traits allows the 
corresponding methods to be called with `Context`.

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
    fn execute(&mut self, command: &str) -> String { 
		// Executes the commands given by the user. The command is converted into an abstract 
		// syntax tree using the labels of built-in concepts. This abstract syntax tree is then 
		// parsed and appropriate operations are performed.
	}
}
```
