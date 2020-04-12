# levenshtein.js

Library for Node.js that uses the simple Levenshtein algorithm.

## How to import

Generated files targeted to ESNext as commonjs:

```js
require('levenshteinjs').default;
```

or

```js
require('levenshteinjs/js').default;
```

Original TypeScript lib file:

```ts
import levenshteinJs from 'levenshteinjs/ts';
```

## Signature

```ts
/**
   * Filter function returns filted collection by Levenshtein Algorithm.
   * @input string.
   * @distance number - can't be bigger than 255 and lower than 0.
   * @collection string[] - collection to search over.
   * @returns string[] - filtered collection.
  */
levenshteinJs.filter(input: string, distance: number, collection: string[]): string[];
```

## Used libraries

https://github.com/wooorm/levenshtein-rs
https://github.com/neon-bindings/neon