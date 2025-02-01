# Cross Language Prop Types

A simple type system designed to allow simple functions to be compiled to
different programming languages.

Given the input:

```
struct Person {
  birthYear: u32,
  name: string,
}
```

It will generate the typescript:

```typescript
export interface Person {
  birthYear: number;
  name: string;
}
```

the java:

```java
interface PersonInterface {
  // not int, because although it's 32 bits it is signed
  long getBirthYear();
  String getName();
}
```

the rust:

```rust
trait Person {
  fn birthYear(&self) -> u32;
  fn name(&self) -> &str;
}

// Also generate a derive macro

#[derive(Person)]
struct Employee {
  birthYear: u32,
  name: String,
}
```

The following observations can be made from the examples:

1. Props are intended to be read only data. In languages where it is relevant,
   only getters are provided.
2. It is imperfect- the `birthYear` field could only be accurately produced in 1
   of the 3 outputs.

## Read more

[development priorities](docs/development_priorities.md)
