# neon-starter

Neon starter template for building **Node.js library** in `Rust`.

[![npm version][npm-version-src]][npm-version-href]
[![npm downloads][npm-downloads-src]][npm-downloads-href]
[![bundle][bundle-src]][bundle-href]
[![JSDocs][jsdocs-src]][jsdocs-href]
[![License][license-src]][license-href]

## Usage

> [!NOTE]
> Replace neon-starter, \_description\_ and Vincent-the-gamer globally to use this template.

Both `Node.js` and `Rust` environments required to use this template, please install them first.

then `clone this repo to local` or `use it as a template`.

Clone to local without `.git` folder:
```shell
npx degit https://github.com/Vincent-the-gamer/neon-starter.git your-project-name

# pnpm
pnpx degit https://github.com/Vincent-the-gamer/neon-starter.git your-project-name
```

## Dev
```shell
pnpm i
cd native && cargo check
cd .. && pnpm run debug
```

## Test

Test your module by:

1. Run `pnpm run debug` to generate `index.node` module.
2. Run `pnpm run tsx test/{/path/to/your/file}` to test your functions. you can run example: 
   ```shell
    pnpm run tsx test/string.test.ts 
   ```


## Build
```shell
pnpm run build
```

# License
[MIT License @Vincent-the-gamer 2024-PRESENT](./LICENSE)

<!-- Badges -->

[npm-version-src]: https://img.shields.io/npm/v/neon-starter?style=flat&colorA=080f12&colorB=1fa669
[npm-version-href]: https://npmjs.com/package/neon-starter
[npm-downloads-src]: https://img.shields.io/npm/dm/neon-starter?style=flat&colorA=080f12&colorB=1fa669
[npm-downloads-href]: https://npmjs.com/package/neon-starter
[bundle-src]: https://img.shields.io/bundlephobia/minzip/neon-starter?style=flat&colorA=080f12&colorB=1fa669&label=minzip
[bundle-href]: https://bundlephobia.com/result?p=neon-starter
[license-src]: https://img.shields.io/github/license/Vincent-the-gamer/neon-starter.svg?style=flat&colorA=080f12&colorB=1fa669
[license-href]: https://github.com/Vincent-the-gamer/neon-starter/blob/main/LICENSE
[jsdocs-src]: https://img.shields.io/badge/jsdocs-reference-080f12?style=flat&colorA=080f12&colorB=1fa669
[jsdocs-href]: https://www.jsdocs.io/package/neon-starter
