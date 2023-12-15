# Packet generation
**Date**: 2021-08-16

# Context
To support packet serialization and deserialization I would like to use proper **struct** in order to have a maintainable code, easy to read and debug, instead of hand-crafting packet each time we need to emit one to client.

I also want parsing to be easy and don't want to maintain parsing code.

There are tone of packet to support and don't want to manually maintain those structure. 
In addition, packets can come in different versions. A same packet can be serialized/deserialized differently depending on packet version of the server.

In sum, packets structures should:
- Support multiple packet version
- Provide serialization and deserialization methods
- Implement `Debug` and `Display` to ease debugging

This would be a lot of repetitive work to achieve this.

# Options evaluation
The decision was made to generate packet structure and trait implementation.

## Using rust macro
Use rust macro: with rust we can use proc-macro in order to generate code at compilation time. As of today (2021) I am not mastering rust macro, but it seems achievable.

Proc-macro can increase build time, and I am a bit afraid that regenerating the same code again and again at compilation time would be too long. Sure there is the incremental compilation but still, the first build would be too long. 

In addition packet structure are not supposed to change once generated.

### Pro
- Seems to be the "rust way" to generate code

### Cons
- Slowing compilation time
- More complex to read generated code as the only way would be using macro expand

## Using hand-craft code generation
Another option would be to generate packet struct using manually written code generator. As of today (2021) I am not aware of a similar tool as `javapoet` in java, so code generation would be plain string writing.

### Pro
- Seems easier to control generated code
- This option would allow to use a "packet database file" instead of a rust file to maintain packet definition. There I can use a more concise syntax than using struct and derivation in rust

### Cons
- Not the rust way
- Code generator could be difficult to read

# Decision
I will use the `hand-craft code generation` option, as it seems easier for me and having a "packet database file" a big plus.