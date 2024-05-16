import { App } from '../app'

const REGISTER_NAMES = ['ZX', 'PC', 'R2', 'R3', 'R4', 'R5', 'LP', 'SP']

const formatVal = (a: number) =>
    '0x' + (a & 0xffff).toString(16).padStart(4, '0')

class CpuMemoryView {
    el: HTMLTableElement
    app: App
    currentAddress: number
    selectedAddress: number

    rowTitleCells: HTMLTableCellElement[]
    memoryInputs: HTMLInputElement[]
    posInput: HTMLInputElement

    minRow = -2
    maxRow = 14

    scrollShift = 0

    constructor(app: App) {
        this.app = app
        this.el = document.createElement('table')
        this.el.className = 'memoryView'
        this.currentAddress = 0x4000
        this.selectedAddress = 0x4000
        this.rowTitleCells = []
        this.memoryInputs = []

        const headerRow = document.createElement('tr')
        {
            const addrCell = document.createElement('td')
            addrCell.innerText = 'Address'
            headerRow.append(addrCell)
        }

        {
            const addrCell = document.createElement('td')
            addrCell.innerText = ''
            addrCell.colSpan = 1000
            this.posInput = document.createElement('input')
            this.posInput.addEventListener('change', () => {
                this.selectedAddress = parseInt(this.posInput.value)
                this.update()
            })
            addrCell.append(this.posInput)
            headerRow.append(addrCell)
        }

        this.el.append(headerRow)

        for (let row = this.minRow; row < this.maxRow; row++) {
            const tableRow = document.createElement('tr')

            const titleCell = document.createElement('td')
            this.rowTitleCells.push(titleCell)
            tableRow.append(titleCell)

            for (let col = 0; col < 4; col++) {
                const relPos = row * 4 + col
                const tableCell = document.createElement('td')
                this.initMemCell(relPos, tableCell)
                tableRow.append(tableCell)
            }
            this.el.append(tableRow)
        }

        this.el.addEventListener('wheel', (e) => {
            this.scrollShift += e.deltaY
            const step = 80

            while (this.scrollShift < -step) {
                this.currentAddress -= 4
                if (!this.selectedVisible()) {
                    this.selectedAddress -= 4
                }
                this.scrollShift += step
            }

            while (this.scrollShift > step) {
                this.currentAddress += 4
                if (!this.selectedVisible()) {
                    this.selectedAddress += 4
                }
                this.scrollShift -= step
            }

            this.update()
        })

        this.update()
        this.app.addEventListener('exec', () => this.update())
    }

    selectedVisible() {
        const diff = this.selectedAddress - this.currentAddress
        return diff >= this.minRow * 4 && diff < this.maxRow * 4
    }

    initMemCell(relPos: number, cell: HTMLTableCellElement) {
        const inp = document.createElement('input')
        inp.addEventListener('change', () => {
            this.app.exec.write_memory(
                this.currentAddress + relPos,
                parseInt(inp.value)
            )
            this.update()
        })

        inp.addEventListener('focus', () => {
            const newAddr = (this.currentAddress + relPos) & 0xffff
            if (newAddr != this.selectedAddress) {
                this.selectedAddress = newAddr
                this.update()
            }
        })

        this.memoryInputs.push(inp)

        cell.append(inp)
    }

    update() {
        if (!this.selectedVisible())
            this.currentAddress = this.selectedAddress & 0xfffc

        this.posInput.value = formatVal(this.selectedAddress)
        let from = this.currentAddress
        from += this.minRow * 4
        from &= 0xffff

        const to = from + (this.maxRow - this.minRow) * 4
        const memory = this.app.exec.read_memory(from, to)

        this.rowTitleCells.forEach((e, i) => {
            e.innerText = formatVal(from + i * 4)
        })

        let regs = this.app.exec.get_registers()

        this.memoryInputs.forEach((e, i) => {
            let pos = (from + i) & 0xffff
            if (pos == this.selectedAddress && this.el.contains(document.activeElement)) {
                e.focus()
            }

            e.value = formatVal(memory[i])
            let color = ''
            if (pos == regs.pc) {
                color = `rgba(255, 128, 0, 0.2)`
            } else if (pos == regs.sp) {
                color = `rgba(255, 0, 128, 0.2)`
            } else if (pos == regs.lp) {
                color = `rgba(0, 192, 192, 0.2)`
            }

            e.style.backgroundColor = color
            if (e.parentElement) e.parentElement.style.backgroundColor = color
        })
    }

    get rootHtmlElement() {
        return this.el
    }
}

export class CpuExecOutput {
    el: HTMLDivElement
    app: App

    registersEl: HTMLDivElement
    regsiterTable: HTMLTableElement
    regCells: (HTMLInputElement | null)[]

    header: HTMLDivElement
    state: HTMLSpanElement
    stepBtn: HTMLButtonElement
    resetBtn: HTMLButtonElement

    memoryView: CpuMemoryView

    constructor(app: App) {
        this.app = app

        this.el = document.createElement('div')
        this.el.className = 'cpuExec'

        this.registersEl = document.createElement('div')
        this.registersEl.className = 'registers'
        const registersTitle = document.createElement('h1')
        registersTitle.innerText = 'Registers'
        this.registersEl.appendChild(registersTitle)

        this.regsiterTable = document.createElement('table')
        this.regsiterTable.className = 'registerTable'
        this.regCells = Array.from({ length: 8 }, () => null)
        for (let i = 0; i < 4; i++) {
            const row = document.createElement('tr')

            row.append(...this.initRegInTable(i), ...this.initRegInTable(i + 4))
            this.regsiterTable.append(row)
        }
        this.registersEl.appendChild(this.regsiterTable)

        this.header = document.createElement('div')
        this.header.className = 'header'
        this.header.innerText = 'CPU:'

        this.state = document.createElement('span')
        this.state.innerText = 'Idle'

        this.stepBtn = document.createElement('button')
        this.stepBtn.innerText = 'Step'
        this.stepBtn.onclick = () => this.app.stepCpu()

        this.resetBtn = document.createElement('button')
        this.resetBtn.innerText = 'Reset'
        this.resetBtn.onclick = () => this.app.resetCpu()
        this.header.append(this.state, this.stepBtn, this.resetBtn)

        this.el.append(this.header, this.registersEl)

        this.memoryView = new CpuMemoryView(this.app)
        const memoryViewTitle = document.createElement('h1')
        memoryViewTitle.innerText = 'Memory View'
        this.el.append(memoryViewTitle, this.memoryView.rootHtmlElement)

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
        if (i == 0) regCellInput.disabled = true

        const regCellVal = document.createElement('td')
        regCellVal.append(regCellInput)

        return [regCellName, regCellVal]
    }

    update() {
        const registers = this.app.exec.get_registers()
        this.regCells[1]!.value = formatVal(registers.pc)
        this.regCells[2]!.value = formatVal(registers.r2)
        this.regCells[3]!.value = formatVal(registers.r3)
        this.regCells[4]!.value = formatVal(registers.r4)
        this.regCells[5]!.value = formatVal(registers.r5)
        this.regCells[6]!.value = formatVal(registers.lp)
        this.regCells[7]!.value = formatVal(registers.sp)

        this.stepBtn.disabled = !this.app.exec.keep_running()

        this.state.innerText = this.app.exec.keep_running() ? 'Idle' : 'Halted'
    }

    get rootHtmlElement() {
        return this.el
    }
}
