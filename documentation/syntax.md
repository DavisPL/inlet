# Syntax

```
file ::= item { item }

item ::= item-mod | item-fn

item-mod ::= "mod" ident "{" file "}"

item-fn ::= "fn" ident ["[" ("*"|path) ]"] "(" param-list ")" "{" block "}"

path ::= ident {"::" ident}

block ::= { stmt ";" }

stmt ::= "let" ident "=" expr

expr ::= term { "+" expr }

term ::= "(" expr ")" | ident | num-lit | fn-call

fn-call ::= path "(" [arg-list] ")"

arg-list ::= expr { "," expr }
```