import { compile, disassemble, DebugCpu } from 'easycpu_wasm'
import { RegistersState } from 'easycpu_wasm'


const DEFAULT_PROGRAM = `
LCONST r2 0b1101	#	Operand	1
LCONST r3 0b1011	#	Operand	2

LCONST r4 0 # Operation 0 and 1 add

JEQ r4 DO_AND

ADD r2 r2 r3
JMP FIN

DO_AND:
AND r2 r2 r3

FIN:
STORE r2 pc 2
HALT
0
`

export class App extends EventTarget {
    program: string

    compileError: string | null
    compiled: Uint16Array

    disassemblyError: string | null
    disassembled: string[]

    exec: DebugCpu
    execRegisters: RegistersState
    execPC: number

    constructor() {
        super();

        this.program = ''

        this.compileError = null
        this.compiled = new Uint16Array()

        this.disassemblyError = null
        this.disassembled = []

        this.exec = new DebugCpu(this.compiled)
        this.execRegisters = this.exec.get_registers()
        this.execPC = 0

        this.addEventListener('programUpdate', () => this.recompile())
        this.addEventListener('compile', () => this.resetCpu())
        this.addEventListener('compile', () => this.disassemble())

        this.setProgram(DEFAULT_PROGRAM)
    }

    setProgram(newProgram: string) {
        this.program = newProgram
        this.dispatchEvent(new Event('programUpdate'))
    }

    recompile() {
        this.compileError = "Failed to compile"
        this.compiled = new Uint16Array()
        try {
            this.compiled = compile(this.program)
            this.compileError = null;
        } catch(e) {
            console.log("Failed compilation", e)
            if(typeof e == 'string') {
                this.compileError = "Failed to compile: \n" + e
            } else {
                console.error("Critical compile error: ", e)
            }
        }

        this.dispatchEvent(new Event('compile'))
    }

    disassemble() {
        this.disassembled = []
        if(this.compileError) {
            this.disassemblyError = "Failed to disassemble: \n" + this.compileError;
            this.dispatchEvent(new Event('disassembly'))
            return;
        }
        try {
            this.disassembled = disassemble(this.compiled) as string[]
            this.disassemblyError = null;
        } catch(e) {
            if(typeof e == 'string') {
                this.disassemblyError = "Failed to disassemble: \n" + e
            } else {
                console.error("Critical disassembly error: ", e)
            }
        }

        this.dispatchEvent(new Event('disassembly'))
    }

    resetCpu() {
        if(!this.compileError) {
            this.exec.reset(this.compiled)
            this.execRegisters = this.exec.get_registers()
            this.execPC = this.execRegisters.pc
            this.dispatchEvent(new Event('exec'))   
        }
    }

    stepCpu() {
        if(this.exec.keep_running()) {
            this.exec.step()
            this.execRegisters = this.exec.get_registers()
            this.execPC = this.execRegisters.pc
        }
        
        this.dispatchEvent(new Event('exec'))
    }

    setCpuRegister(reg: number, val: number) {
        this.exec.set_register(reg, val)
        this.execRegisters = this.exec.get_registers()
        this.execPC = this.execRegisters.pc

        this.dispatchEvent(new Event('exec'))
    }
}