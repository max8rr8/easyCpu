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
        this.app.addEventListener('exec', () => this.updateExec())

        this.update()
        this.updateExec()
    }

    update() {
        this.el.classList.remove('disassemblyError')
        this.el.innerHTML = ''

        if(this.app.disassemblyError) {
            this.el.classList.add('disassemblyError')
            this.el.innerText = this.app.disassemblyError
        } else {
            this.el.append(...this.app.disassembled.map(dis=>{
                let line = document.createElement('pre')
                line.className = 'codeLine'
                line.innerText = dis
                return line
            }))
        }
    }

    updateExec() {
        console.log("exec", this.app.execPC);

        [...this.el.querySelectorAll('.curExec')].forEach(e=>e.classList.remove('curExec'))
        if(this.app.execPC > this.el.childNodes.length) return;
        let curExecLine = this.el.childNodes[this.app.execPC] as HTMLPreElement
        curExecLine.classList.add('curExec')
    }
}

const REGISTER_NAMES = ['ZX', 'PC', 'R2', 'R3', 'R4', 'R5', 'LP', 'SP']


class CpuExecOutput {
    el: HTMLDivElement
    regsiterTable: HTMLTableElement
    app: App

    regCells: (HTMLInputElement | null)[]

    stepBtn: HTMLButtonElement
    resetBtn: HTMLButtonElement
    
    constructor(app: App) {
        this.app = app

        this.el = document.createElement("div")
        this.el.className = "cpuExec"

        this.regsiterTable = document.createElement('table')
        this.regsiterTable.className = 'registerTable'
        this.regCells = Array.from({ length: 8 }, () => null)
        for(let i = 0; i < 4; i++) {
            const row = document.createElement('tr')

            row.append(...this.initRegInTable(i), ...this.initRegInTable(i+4))
            this.regsiterTable.append(row)
        }

        this.stepBtn = document.createElement('button')
        this.stepBtn.innerText = 'Step'
        this.stepBtn.onclick = () => this.app.stepCpu()

        
        this.resetBtn = document.createElement('button')
        this.resetBtn.innerText = 'Reset'
        this.resetBtn.onclick = () => this.app.resetCpu()

        this.el.append(this.stepBtn, this.resetBtn, this.regsiterTable)

        this.app.addEventListener('exec', () => this.update())
        this.update()
    }

    initRegInTable(i: number): HTMLElement[] {
        const regCellName = document.createElement('td')
        regCellName.innerText = REGISTER_NAMES[i]

        const regCellInput = document.createElement('input')
        regCellInput.value = '0x0000'
        regCellInput.addEventListener('change', () => {
            this.app.setCpuRegister(i, parseInt(regCellInput.value))
        })

        this.regCells[i] = regCellInput

        const regCellVal = document.createElement('td')
        regCellVal.append(regCellInput)
        
        return [regCellName, regCellVal]
    }

    update() {
        const registers = this.app.exec.get_registers()
        this.regCells[1]!.value = '0x' + registers.pc.toString(16).padStart(4, '0');
        this.regCells[2]!.value = '0x' + registers.r2.toString(16).padStart(4, '0');
        this.regCells[3]!.value = '0x' + registers.r3.toString(16).padStart(4, '0');
        this.regCells[4]!.value = '0x' + registers.r4.toString(16).padStart(4, '0');
        this.regCells[5]!.value = '0x' + registers.r5.toString(16).padStart(4, '0');
        this.regCells[6]!.value = '0x' + registers.sp.toString(16).padStart(4, '0');
        this.regCells[7]!.value = '0x' + registers.lp.toString(16).padStart(4, '0');

        this.stepBtn.disabled = !this.app.exec.keep_running();
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