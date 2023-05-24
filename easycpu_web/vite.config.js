import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
    base: '',
    plugins: [wasmPack('../easycpu_wasm')]
});