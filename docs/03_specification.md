---
title: Specification
layout: home
nav_order: 3
---

# Specification

## Grammar

This is a [PEG grammar][PEG Paper]. This is for documentation. All parsers are
hand-written, but informed by this grammar. Having this written explicitly makes
it easier to implement parsers for all the languages libraries.

```
grammar
  = package? import* declaration*

package
  = PACKAGE (COLON PACKAGE)* SEMI
  
declaration
  = constructor_decl
  | schema_decl
  | annotation_decl

constructor_decl =
  annotation_def* CONSTRUCTOR block_decl 

schema_decl =
  annotation_def* SCHEMA block_decl 

block_decl =
  OPEN_BRACE (block_attr_decl* | map_decl | union_decl | repatable_decl) CLOSE_BRACE

map_decl =
  MAP OPEN_CHEVRON type COMMA type CLOSE_CHEVRON

union_decl =
  UNION OPEN_PAREN block_attr_decl* CLOSE_PAREN

repeatable_decl =
  REPEATABLE block_decl

annotation_decl =
  annotation_def* ANNOTATION OPEN_PAREN annotation_attr_decl* CLOSE_PAREN

AT = "@"
CONSTRUCTOR = "constructor"
SCHEMA = "schema"
ANNOTATION = "constructor"
COLON = ":"
OPEN_BRACE = "{"
CLOSE_BRACE = "}"
OPEN_PAREN = "("
CLOSE_PAREN = ")"
```

----

[PEG Paper]: https://pdos.csail.mit.edu/papers/parsing:popl04.pdf