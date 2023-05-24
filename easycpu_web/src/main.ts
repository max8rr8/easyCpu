import { App } from './app'
import { AppUI } from './ui'
import init from 'easycpu_wasm'

init().then(() => {
  const app = new App()

  const rootEl =
    document.querySelector<HTMLDivElement>('#app') ??
    document.createElement('div')
  const ui = new AppUI(app, rootEl)

  console.log('EasyCPU IDE running app:', app, 'ui:', ui)
})
