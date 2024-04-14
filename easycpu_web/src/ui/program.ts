import { App } from '../app'
import { editor, languages, MarkerSeverity, Range } from 'monaco-editor'

languages.register({ id: 'easm' })

// Register a tokens provider for the language
languages.setMonarchTokensProvider('easm', {
  defaultToken: '',
  ignoreCase: true,
  tokenPostfix: '.easm',

  brackets: [
    { token: 'delimiter.bracket', open: '{', close: '}' },
    { token: 'delimiter.parenthesis', open: '(', close: ')' },
    { token: 'delimiter.square', open: '[', close: ']' },
  ],

  registers: /r[0-8]|zx|pc|sp|lp/,

  // The main tokenizer for our languages
  tokenizer: {
    root: [
      [/[ \t\r\n]+/, ''],
      [/(#(?:\s.*|))$/, 'comment'],
      [/\w+:/, 'metatag'],
      [/@registers/, 'variable'],
      [/[{}()\[\]]/, '@brackets'],
      [/-?0[xX][0-9a-fA-F_]*[0-9a-fA-F]/, 'number'],
      [/-?0[bB][01]*[01]/, 'number'],
      [/-?\d+/, 'number'],
      [/[;,.]/, 'delimiter'],
      [/"/, 'string', '@string."'],
    ],

    string: [
      [/[^\\"]+$/, 'string', '@popall'],
      [/[^\\"]+/, 'string'],
      [/\\./, 'string'],
      [/"/, 'string.escape', '@popall'],
      [/\\$/, 'string'],
    ],
  },
})

export class ProgramInput {
  el: HTMLDivElement
  editor: editor.ICodeEditor
  app: App

  constructor(app: App) {
    this.app = app

    this.el = document.createElement('div')
    this.el.className = 'asmInput'

    this.editor = editor.create(this.el, {
      value: this.app.program,
      bracketPairColorization: {
        enabled: true,
      },
      autoClosingBrackets: 'always',
      language: 'easm',
      automaticLayout: true,

      theme: 'vs-dark',
    })

    this.editor.onDidChangeModelContent(() => {
      this.app.setProgram(this.editor.getValue())
    })

    this.app.addEventListener('programUpdate', () => {
      if (this.editor.getValue() != this.app.program) {
        let oldPos = this.editor.getPosition()
        this.editor.setValue(this.app.program)
        if (oldPos) {
          this.editor.setPosition(oldPos)
        }
      }
    })

    this.app.addEventListener('compile', () => this.updateMarkers());
  }

  get rootHtmlElement() {
    return this.el
  }

  updateMarkers() {
    const model = this.editor.getModel();
    if(!model) return;
    
    let markers: editor.IMarkerData[] = [];
    const errors = this.app.compileError;
    if(errors) {
      for(let err of errors) {
        markers.push({
          message: err.get_message(),
          severity: MarkerSeverity.Error,
          startLineNumber: err.start.line,
          startColumn: err.start.column,
          endLineNumber: err.end.line,
          endColumn: err.end.column,
        })
      }
    }
    editor.setModelMarkers(model, "owner", markers);
  }
}

export class DisassemblyOutput {
  el: HTMLDivElement
  editor: editor.ICodeEditor
  errorEl: HTMLPreElement

  app: App

  execDecor: editor.IEditorDecorationsCollection

  constructor(app: App) {
    this.app = app

    this.el = document.createElement('div')
    this.el.className = 'asmInput'

    this.editor = editor.create(this.el, {
      value: this.app.disassembled.join('\n'),
      language: 'easm',
      automaticLayout: true,
      theme: 'vs-dark',
      readOnly: true,
    })

    this.execDecor = this.editor.createDecorationsCollection()

    this.errorEl = document.createElement('pre')
    this.errorEl.className = 'disassemblyError'
    this.el.appendChild(this.errorEl)

    this.app.addEventListener('programUpdate', () => this.update())
    this.app.addEventListener('exec', () => this.updateExec())
    this.update()
  }

  update() {
    if (this.app.disassemblyError) {
      this.errorEl.innerText = this.app.disassemblyError
      this.errorEl.style.visibility = 'visible'
    } else {
      this.errorEl.style.visibility = 'hidden'

      let oldPos = this.editor.getPosition()
      this.editor.setValue(this.app.disassembled.join('\n'))
      if (oldPos) {
        this.editor.setPosition(oldPos)
      }

      this.updateExec()
    }
  }

  updateExec() {
    console.log('Update')
    this.execDecor.set([
      {
        range: new Range(this.app.execPC + 1, 1, this.app.execPC + 1, 1),
        options: {
          isWholeLine: true,
          className: 'curExec',
        },
      },
    ])
  }

  get rootHtmlElement() {
    return this.el
  }
}
