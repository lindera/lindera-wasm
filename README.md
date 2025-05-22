# lindera-wasm

WebAssembly of Lindera

![Screenshot 2024-12-25 143434](https://github.com/user-attachments/assets/a463bf2e-70be-492a-b13c-e5aff6581fc7)

## Demo Application

- <https://lindera.github.io/lindera-wasm/>

## npm

- <https://www.npmjs.com/package/lindera-wasm>

## Install project dependencies

- wasm-pack : <https://rustwasm.github.io/wasm-pack/installer/>

## Setup repository

```shell
# Clone lindera-py project repository
% git clone git@github.com:lindera/lindera-wasm.git
% cd lindera-wasm
```

## Build project

```shell
% wasm-pack build --release --features=cjk --target=bundler
```

## Build example web application

```shell
% cd lindera-wasm && npm install && npm run build && cp index.html dist/index.html
```

## Run example web application

```shell
% cd lindera-wasm && npm run start
```
