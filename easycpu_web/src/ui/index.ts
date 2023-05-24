import './style.css'
import { App } from '../app'
import { CpuExecOutput } from './exec'
import { DisassemblyOutput, ProgramInput } from './program'
import { GoldenLayout } from 'golden-layout'
import 'golden-layout/dist/css/goldenlayout-base.css'
import 'golden-layout/dist/css/themes/goldenlayout-dark-theme.css'

export class AppUI {
  app: App

  root: HTMLDivElement
  mainArea: HTMLDivElement

  programInput: ProgramInput
  disassemblyOutput: DisassemblyOutput

  cpuExec: CpuExecOutput

  layout: GoldenLayout

  constructor(app: App, root: HTMLDivElement) {
    this.app = app
    this.root = root

    this.programInput = new ProgramInput(app)
    this.disassemblyOutput = new DisassemblyOutput(app)
    this.cpuExec = new CpuExecOutput(app)

    this.mainArea = document.createElement('div')
    this.root.append(this.mainArea)

    this.layout = new GoldenLayout(this.mainArea)
    this.layout.registerComponentFactoryFunction(
      'program',
      () => this.programInput,
      true
    )
    this.layout.registerComponentFactoryFunction(
      'disassembly',
      () => this.disassemblyOutput,
      true
    )
    this.layout.registerComponentFactoryFunction(
      'cpuExec',
      () => this.cpuExec,
      true
    )

    this.layout.loadLayout({
      root: {
        type: 'row',
        content: [
          { type: 'component', componentType: 'program' },
          { type: 'component', componentType: 'disassembly' },
          { type: 'component', componentType: 'cpuExec' },
        ],
      },
      settings: {
        responsiveMode: 'always',
      },
      header: {
        popout: false,
        maximise: false,
        close: '',
      },
    })

    window.addEventListener('resize', () => this.layout.updateRootSize())
  }
}
