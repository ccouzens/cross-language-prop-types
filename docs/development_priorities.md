# Development Priorities

This tool is a building block within a larger system. The larger system is
tooling to facilitate server side rendering web pages in one language, and
hydrating them and updating them in Javascript. Within this system, it is used
to type parameters to simple functions that produce: HTML/vDOM, CSS styles and
CSS classes.

Whilst I hope this tool is useful to further use, the above will take priority.
This may mean that certain decisions are made:

- Typescript output, compatible with popular frontend frameworks such as React
  or Svelte will be a priority
- Callbacks may initially only be supported in Typescript, as in web development
  only the client is interactive
- Numeric types may be a bit fuzzy - as Javascript supports double precision 64
  bit floats and BigInts only.
