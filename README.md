# lindera-wasm

WebAssembly of Lindera

![Screenshot](https://github.com/user-attachments/assets/0cccbc5b-78b3-462c-acb6-18136129cf04)

## Demo Application

- <https://lindera.github.io/lindera-wasm/>

## npm

- <https://www.npmjs.com/package/lindera-wasm>  
Lindera WASM with CJK dictionaries (IPADIC, ko-dic, CC-CEDICT)

- <https://www.npmjs.com/package/lindera-wasm-ipadic>  
Lindera WASM with Japanese dictionary (IPADIC)

- <https://www.npmjs.com/package/lindera-wasm-unidic>  
Lindera WASM with Japanese dictionary (UniDic)

- <https://www.npmjs.com/package/lindera-wasm-ko-dic>  
Lindera WASM with Korean dictionary (ko-dic)

- <https://www.npmjs.com/package/lindera-wasm-cc-cedict>  
Lindera WASM with Chinese dictionary (CC-CEDICT)

## Usage
init the wasm module before construct `TokenizerBuilder`:
```ts
import __wbg_init, { TokenizerBuilder } from 'lindera-wasm'

__wbg_init.then(() => {
    const builder = new TokenizerBuilder()
    //...
})
```

### for [Vite](https://vite.dev/) base project
You should exclude this package in the `optimizeDeps`:
```ts
// vite.config.js
import { defineConfig } from 'vite'

export default defineConfig({
  optimizeDeps: {
    exclude: [
      "lindera-wasm"
    ]
  },
})
```

### for Browser extension development
Set the `cors` config in vite.config.js
```ts
// vite.config.js
import { defineConfig } from 'vite'

export default defineConfig({
  server: {
    cors: {
      origin: [
        /chrome-extension:\/\//,
      ],
    },
  },
})
```

and set the `content_security_policy` to contains `wasm-unsafe-eval` in manifest.json:
```json
// manifest.json
"content_security_policy": {
	"extension_pages": "script-src 'self' 'wasm-unsafe-eval';"
}
```


## Development

### Install project dependencies

- wasm-pack : <https://rustwasm.github.io/wasm-pack/installer/>

### Setup repository

```shell
# Clone lindera-py project repository
% git clone git@github.com:lindera/lindera-wasm.git
% cd lindera-wasm
```

### Build project

```shell
% wasm-pack build --release --features=cjk --target=bundler
```

### Build example web application

```shell
% cd lindera-wasm && npm install && npm run build && cp index.html dist/index.html
```

### Run example web application

```shell
% cd lindera-wasm && npm run start
```
