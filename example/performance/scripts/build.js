import { execSync } from 'child_process';
import fs from 'fs'

const replaceTarget='node_modules/.pnpm/kuromoji@0.1.2/node_modules/kuromoji/src/loader/DictionaryLoader.js'
const dictDir = 'node_modules/.pnpm/kuromoji@0.1.2/node_modules/kuromoji/dict/'
const dictInstallDir = 'static/dict/'


// rm -rf node_modules
const nodeModulesPath = 'node_modules';
if (fs.existsSync(nodeModulesPath))
    fs.rmSync(nodeModulesPath, { recursive: true, force: true });

execSync('pnpm install');

const originalKuromoji= fs.readFileSync(replaceTarget,'utf-8')

const body=`function pathJoin(...parts) {
    return parts
      .map(part => part.replace(/\\/+$/, ''))
      .filter(part => part !== '')
      .join('/')
      .replace(/\\/+/g, '/');
}
${originalKuromoji.replaceAll('path.join','pathJoin')}
`

fs.writeFileSync(replaceTarget,body)
fs.cpSync(dictDir, dictInstallDir, {recursive: true});

execSync('NODE_OPTIONS=--experimental-wasm-modules npx vite build');

fs.rmSync(dictInstallDir, { recursive: true, force: true });

