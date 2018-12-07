# `zia`: Interpreter for the Zia programming language.

The Zia project aims to allow programs to be easily adaptable. In contrast to functional 
programming, in which program data is immutable wherever possible, Zia program data is mutable 
wherever possible. In contrast to traditional interpreted languages, Zia source code plays a role 
similar to database query languages. In contrast to traditional databases, Zia further abstracts 
away data representation details (such as tables and columns) and allows programs to be stored. 

The Zia syntax represents a binary tree where parentheses group a pair of expressions and 
a space separates a pair of expressions.

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
`a (-> b)` represents the command to specify the rule that the concept labelled by `a` reduces 
to the concept labelled by `b`.

`->` is also used to print the symbol of the normal form of a concept. For example `a ->`
represents the command to print `b` in the above case of `a (-> b)` but `c ->` prints `c` because
no reduction rule exists for `c`.

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

`:=` can be used to label a binary tree of concepts or relabel of a concept. For example `c (:= (a b))` means graphically:
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

The current implementation exposes the `Context` type that can be used in an interface such as 
[IZia](https://github.com/Charles-Johnson/izia). 

```
impl Context {
	pub fn new() -> Context { 
		// Constructs a new Context with 3 built-in concepts: one to encode the labels of concepts
    	// (id=LABEL), one to encode commands to define or print the definitions of concepts (id = 
    	// DEFINE) and one to encode commands to define reduction rules or print the normal forms 
		// of concepts (id = REDUCTION).
    }
    pub fn execute(&mut self, command: &str) -> String { 
		// Executes the commands given by the user. The command is converted into an abstract 
		// syntax tree using the labels of built-in concepts. This abstract syntax tree is then 
		// parsed and appropriate operations are performed.
	}
}
```
