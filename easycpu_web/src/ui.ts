import { App } from './app'
import { RegistersState } from 'easycpu_wasm'

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

const REGISTER_NAMES = ['ZX', 'PC', 'R2', 'R3', 'R4', 'R5', 'LP', 'SP']


class CpuExecOutput {
    el: HTMLDivElement
    regsiterTable: HTMLTableElement
    app: App

    regCells: {[a in string]: HTMLTableCellElement}

    stepBtn: HTMLButtonElement
    
    constructor(app: App) {
        this.app = app

        this.el = document.createElement("div")
        this.el.className = "cpuExec"

        this.regsiterTable = document.createElement('table')
        this.regsiterTable.className = 'registerTable'
        this.regCells = {}
        for(let i = 0; i < 4; i++) {
            const row = document.createElement('tr')

            const regCellNameA = document.createElement('td')
            regCellNameA.innerText = REGISTER_NAMES[i]
            const regCellValA = document.createElement('td')
            regCellValA.innerText = '0x0000'
            this.regCells[REGISTER_NAMES[i]] = regCellValA;

            
            const regCellNameB = document.createElement('td')
            regCellNameB.innerText = REGISTER_NAMES[4 + i]
            const regCellValB = document.createElement('td')
            regCellValB.innerText = '0x0000'
            this.regCells[REGISTER_NAMES[4 + i]] = regCellValB;

            row.append(regCellNameA, regCellValA, regCellNameB, regCellValB)
            this.regsiterTable.append(row)
        }

        this.stepBtn = document.createElement('button')
        this.stepBtn.innerText = 'Step'
        this.stepBtn.onclick = () => this.app.stepCpu()

        this.el.append(this.stepBtn, this.regsiterTable)

        this.app.addEventListener('exec', () => this.update())
        this.update()
    }

    update() {
        const registers = this.app.exec.get_registers()
        this.regCells["PC"].innerText = '0x' + registers.pc.toString(16).padStart(4, '0');
        this.regCells["R2"].innerText = '0x' + registers.r2.toString(16).padStart(4, '0');
        this.regCells["R3"].innerText = '0x' + registers.r3.toString(16).padStart(4, '0');
        this.regCells["R4"].innerText = '0x' + registers.r4.toString(16).padStart(4, '0');
        this.regCells["R5"].innerText = '0x' + registers.r5.toString(16).padStart(4, '0');
        this.regCells["SP"].innerText = '0x' + registers.sp.toString(16).padStart(4, '0');
        this.regCells["LP"].innerText = '0x' + registers.lp.toString(16).padStart(4, '0');
    }
}

export class AppUI {
    app: App
    
    root: HTMLDivElement
    mainArea: HTMLDivElement

    programInput: ProgramInput
    disassemblyOutput: DisassemblyOutput

    cpuExec: CpuExecOutput

    constructor(app: App, root: HTMLDivElement) {
        this.app = app;
        this.root = root;

        this.programInput = new ProgramInput(app)
        this.disassemblyOutput = new DisassemblyOutput(app)
        this.cpuExec = new CpuExecOutput(app)

        this.mainArea = document.createElement('div')
        this.mainArea.className = "mainArea"
        this.mainArea.append(this.programInput.el, this.disassemblyOutput.el, this.cpuExec.el)

        this.root.append(this.mainArea)
    }
}