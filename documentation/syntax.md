# Syntax

```
file ::= item { item }

item ::= item-mod | item-fn

item-mod ::= "mod" ident "{" file "}"

item-fn ::= "fn" ident "(" [param-list] ")" "->" origin "{" block "}"

origin ::= "{" ("*"|path) "}"

path ::= ident {"::" ident}

block ::= { stmt ";" }

stmt ::= "let" ident "=" expr | "return" expr

expr ::= term { "+" expr }

term ::= "(" expr ")" | ident | num-lit | fn-call

fn-call ::= path "(" [arg-list] ")"

arg-list ::= expr { "," expr }

param-list ::= param { "," param }

param ::= ident ":" origin
```