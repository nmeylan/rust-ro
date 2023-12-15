# Script lang
**Date**: 2022-01-16

# Context
In order to support `NPC` or scriptable items, we would need a language "lighter" than rust.

Rathena and Hercules use a dedicated language, I am not sure it has a name but lets call it `rathena script lang`.

# Option evaluation

- Reuse `rathena script lang` for script in rust-ro
- Use `lua`

## Reuse `rathena script lang`
In order to support `rathena script lang` we need to build an interpreter, we can't reuse what is done in hercule or rathena because the interpreter code is tightly couple to the server, it is not possible to extract it as a native library.
### Pros
- This option would allow to use existing scripts written for more than 20 years by the community.
- Someone could be interested to reuse the interpreter in its server implementation
### Cons
- `rathena script lang` syntax is far from perfect, sometimes lack of consistency, some function behave totally differently: argument are always passed by value, unless for some function, were they are passed by referenced, there is nothing in the syntax to define that, so it should be "hardcoded" somewhere that the function argument is reference and not a value
- Building an interpreter could be long

## Use `lua`
### Pros
- lua is a famous language use for scripting, well known and easy to use
### Cons
- Unless we can convert existing script into lua, we would not be able to reuse the work done by the community over 20 years

# Decision
I prefer the idea to be able to reuse existing script, so I take the decision to build an interpreter for `rathena script lang`.
The lexer, parser, compiler part could open the door on a `rathena script lang` to `lua` transpiler.