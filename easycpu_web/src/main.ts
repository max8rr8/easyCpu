import './style.css'
import init, { compile, disassemble } from 'easycpu_wasm'

await init()

const appEl = document.querySelector<HTMLDivElement>('#app');

const mainAreaEl = document.createElement("div")
mainAreaEl.className = "mainArea"


const asmInputEl = document.createElement("textarea")
asmInputEl.className = "asmInput"

const disasmOutputEl = document.createElement("pre")
disasmOutputEl.className = "disasmOutput"

mainAreaEl.append(asmInputEl, disasmOutputEl)

const toolbarEl = document.createElement("div")

const compileBtnEl = document.createElement("button")
compileBtnEl.className = "compileBtn"
compileBtnEl.innerText = 'Compile'

compileBtnEl.onclick = () => {
    disasmOutputEl.classList.remove("failed_compile")

    let asmInput  = asmInputEl.value
    try {
        let compiled = compile(asmInput)
        let disassembled = disassemble(compiled)
        disasmOutputEl.innerText = disassembled;
    } catch (e) {
        if(typeof e == 'string') {
            disasmOutputEl.classList.add("failed_compile")
            disasmOutputEl.innerText = e.toString();
        } else {
            throw e
        }
    }
}

toolbarEl.append(compileBtnEl)

appEl?.append(toolbarEl, mainAreaEl)
