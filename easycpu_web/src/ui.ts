import { App } from './app'

class ProgramInput {
    el: HTMLTextAreaElement
    app: App

    constructor(app: App) {
        this.app = app

        this.el = document.createElement("textarea")
        this.el.className = "asmInput"
        this.el.value = this.app.program

        this.el.addEventListener('input', () => this.update())
        this.el.addEventListener('change', () => this.update())
        
        this.app.addEventListener('programUpdate', () => {
            this.el.value = this.app.program
        })
    }

    update() {
        this.app.setProgram(this.el.value)
    }
}

class DisassemblyOutput {
    el: HTMLPreElement
    app: App

    
    constructor(app: App) {
        this.app = app

        this.el = document.createElement("pre")
        this.el.className = "disasmOutput"

        this.app.addEventListener('disassembly', () => this.update())
        this.update()
    }

    update() {
        this.el.classList.remove('disassemblyError')

        if(this.app.disassemblyError) {
            this.el.classList.add('disassemblyError')
            this.el.innerText = this.app.disassemblyError
        } else {
            this.el.innerText =  this.app.disassembled
        }
    }
}

export class AppUI {
    app: App
    
    root: HTMLDivElement
    mainArea: HTMLDivElement

    programInput: ProgramInput
    disassemblyOutput: DisassemblyOutput

    constructor(app: App, root: HTMLDivElement) {
        this.app = app;
        this.root = root;

        this.programInput = new ProgramInput(app)
        this.disassemblyOutput = new DisassemblyOutput(app)

        this.mainArea = document.createElement('div')
        this.mainArea.className = "mainArea"
        this.mainArea.append(this.programInput.el, this.disassemblyOutput.el)

        this.root.append(this.mainArea)
    }
}