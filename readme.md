# Convex TypeGen

[Convex](https://www.convex.dev) is an amazing real-time database that allows developers to write their backend app logic in typescript. Convex does support an [offical Rust SDK](https://docs.rs/convex/latest/convex/), and this crate aims to expand on the rust developer experience using Convex.

## Features

- Blazing fast AST parsing and type generation
- Generate Rust types from your ConvexDB schema.ts file
- Automatically re-generate the types when the schema.ts file changes

## Supported Types

- [ ] `null`
- [ ] `boolean`
- [ ] `int64`
- [ ] `float64`
- [ ] `string`
- [ ] `bytes`
- [ ] `array`
- [ ] `object`
- [ ] `record`

## Limitations

- Nested types are not supported (e.g. `array` of `object` of `any`) yet.

## Example

todo

## Installation

```sh
cargo add convex-typegen
```
