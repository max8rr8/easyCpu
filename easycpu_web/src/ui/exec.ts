import { App } from '../app'

const REGISTER_NAMES = ['ZX', 'PC', 'R2', 'R3', 'R4', 'R5', 'LP', 'SP']

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
    this.regCells[1]!.value = '0x' + registers.pc.toString(16).padStart(4, '0')
    this.regCells[2]!.value = '0x' + registers.r2.toString(16).padStart(4, '0')
    this.regCells[3]!.value = '0x' + registers.r3.toString(16).padStart(4, '0')
    this.regCells[4]!.value = '0x' + registers.r4.toString(16).padStart(4, '0')
    this.regCells[5]!.value = '0x' + registers.r5.toString(16).padStart(4, '0')
    this.regCells[6]!.value = '0x' + registers.sp.toString(16).padStart(4, '0')
    this.regCells[7]!.value = '0x' + registers.lp.toString(16).padStart(4, '0')

    this.stepBtn.disabled = !this.app.exec.keep_running()

    this.state.innerText = this.app.exec.keep_running() ? 'Idle' : 'Halted'
  }

  get rootHtmlElement() {
    return this.el
  }
}
