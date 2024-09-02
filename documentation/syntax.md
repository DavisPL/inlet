# Syntax

```
file ::= item { item }

item ::= item-mod | item-fn

item-mod ::= "mod" ident "{" file "}"

item-fn ::= "fn" ident ["[" ("*"|path) ]"] "(" param-list ")" "{" block "}"

path ::= ident {"::" ident}

block ::= { call; }

call ::= path "(" ")"
```