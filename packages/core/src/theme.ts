/**
 * Theme generation utilities
 * Creates editor-specific theme objects from core color definitions
 */

import {
  baseColors,
  syntaxColors,
  backgroundColors,
  borderColors,
  uiColors,
  palette
} from './colors.js';

export interface ThemeColors {
  // Base ANSI colors
  ansiBlack: string;
  ansiRed: string;
  ansiGreen: string;
  ansiYellow: string;
  ansiBlue: string;
  ansiMagenta: string;
  ansiCyan: string;
  ansiWhite: string;

  // Bright ANSI colors
  ansiBrightBlack: string;
  ansiBrightRed: string;
  ansiBrightGreen: string;
  ansiBrightYellow: string;
  ansiBrightBlue: string;
  ansiBrightMagenta: string;
  ansiBrightCyan: string;
  ansiBrightWhite: string;

  // Dim ANSI colors
  ansiDimBlack: string;
  ansiDimRed: string;
  ansiDimGreen: string;
  ansiDimYellow: string;
  ansiDimBlue: string;
  ansiDimMagenta: string;
  ansiDimCyan: string;
  ansiDimWhite: string;

  // UI colors
  background: string;
  foreground: string;
  cursor: string;
  selection: string;
  lineHighlight: string;

  // Syntax colors
  keyword: string;
  function: string;
  string: string;
  number: string;
  comment: string;
  type: string;
  variable: string;
  constant: string;
  operator: string;
  attribute: string;
}

export interface TerminalTheme {
  name: string;
  background: string;
  foreground: string;
  cursor: string;
  selection: string;

  // ANSI colors (16 colors)
  black: string;
  red: string;
  green: string;
  yellow: string;
  blue: string;
  magenta: string;
  cyan: string;
  white: string;

  brightBlack: string;
  brightRed: string;
  brightGreen: string;
  brightYellow: string;
  brightBlue: string;
  brightMagenta: string;
  brightCyan: string;
  brightWhite: string;
}

export interface EditorTheme {
  name: string;
  type: 'dark' | 'light';
  colors: {
    // Editor
    'editor.background': string;
    'editor.foreground': string;
    'editor.lineHighlightBackground': string;
    'editor.selectionBackground': string;
    'editorCursor.foreground': string;

    // Sidebar
    'sideBar.background': string;
    'sideBar.foreground': string;
    'sideBar.border': string;

    // Activity Bar
    'activityBar.background': string;
    'activityBar.foreground': string;
    'activityBar.border': string;

    // Status Bar
    'statusBar.background': string;
    'statusBar.foreground': string;
    'statusBar.border': string;

    // Title Bar
    'titleBar.activeBackground': string;
    'titleBar.activeForeground': string;
    'titleBar.inactiveBackground': string;
    'titleBar.border': string;

    // Tabs
    'tab.activeBackground': string;
    'tab.activeForeground': string;
    'tab.inactiveBackground': string;
    'tab.inactiveForeground': string;
    'tab.border': string;

    // Panel
    'panel.background': string;
    'panel.border': string;

    // Terminal
    'terminal.background': string;
    'terminal.foreground': string;
    'terminal.ansiBlack': string;
    'terminal.ansiRed': string;
    'terminal.ansiGreen': string;
    'terminal.ansiYellow': string;
    'terminal.ansiBlue': string;
    'terminal.ansiMagenta': string;
    'terminal.ansiCyan': string;
    'terminal.ansiWhite': string;
    'terminal.ansiBrightBlack': string;
    'terminal.ansiBrightRed': string;
    'terminal.ansiBrightGreen': string;
    'terminal.ansiBrightYellow': string;
    'terminal.ansiBrightBlue': string;
    'terminal.ansiBrightMagenta': string;
    'terminal.ansiBrightCyan': string;
    'terminal.ansiBrightWhite': string;
  };
  tokenColors: Array<{
    scope: string | string[];
    settings: {
      foreground?: string;
      fontStyle?: string;
    };
  }>;
}

export interface Theme {
  colors: ThemeColors;
  terminal: TerminalTheme;
  editor: EditorTheme;
}

/**
 * Generate a complete terminal theme
 */
export function createTerminalTheme(): TerminalTheme {
  return {
    name: 'Subliminal Nightfall',
    background: palette.bg,
    foreground: palette.fg,
    cursor: palette.cursor,
    selection: palette.selection,

    // Base ANSI
    black: palette.fgDim,
    red: palette.red,
    green: palette.green,
    yellow: palette.yellow,
    blue: palette.blue,
    magenta: palette.magenta,
    cyan: palette.cyan,
    white: '#d4d4d4',

    // Bright ANSI
    brightBlack: palette.fgDim,
    brightRed: palette.redBright,
    brightGreen: palette.greenBright,
    brightYellow: palette.yellowBright,
    brightBlue: palette.blueBright,
    brightMagenta: palette.magentaBright,
    brightCyan: palette.cyanBright,
    brightWhite: '#ffffff'
  };
}

/**
 * Generate a complete editor theme (VSCode format)
 */
export function createEditorTheme(): EditorTheme {
  return {
    name: 'Subliminal Nightfall',
    type: 'dark',
    colors: {
      // Editor
      'editor.background': palette.bg,
      'editor.foreground': palette.fg,
      'editor.lineHighlightBackground': palette.lineHighlight,
      'editor.selectionBackground': palette.selection,
      'editorCursor.foreground': palette.cursor,

      // Sidebar
      'sideBar.background': palette.bgAlt,
      'sideBar.foreground': palette.fg,
      'sideBar.border': palette.border,

      // Activity Bar
      'activityBar.background': palette.bg,
      'activityBar.foreground': palette.fg,
      'activityBar.border': palette.border,

      // Status Bar
      'statusBar.background': palette.bg,
      'statusBar.foreground': palette.fg,
      'statusBar.border': palette.border,

      // Title Bar
      'titleBar.activeBackground': palette.bg,
      'titleBar.activeForeground': palette.fg,
      'titleBar.inactiveBackground': palette.bgAlt,
      'titleBar.border': palette.border,

      // Tabs
      'tab.activeBackground': palette.bg,
      'tab.activeForeground': palette.fg,
      'tab.inactiveBackground': palette.bgAlt,
      'tab.inactiveForeground': palette.fgMuted,
      'tab.border': palette.border,

      // Panel
      'panel.background': palette.bgAlt,
      'panel.border': palette.border,

      // Terminal
      'terminal.background': palette.bg,
      'terminal.foreground': palette.fg,
      'terminal.ansiBlack': palette.fgDim,
      'terminal.ansiRed': palette.red,
      'terminal.ansiGreen': palette.green,
      'terminal.ansiYellow': palette.yellow,
      'terminal.ansiBlue': palette.blue,
      'terminal.ansiMagenta': palette.magenta,
      'terminal.ansiCyan': palette.cyan,
      'terminal.ansiWhite': '#d4d4d4',
      'terminal.ansiBrightBlack': palette.fgDim,
      'terminal.ansiBrightRed': palette.redBright,
      'terminal.ansiBrightGreen': palette.greenBright,
      'terminal.ansiBrightYellow': palette.yellowBright,
      'terminal.ansiBrightBlue': palette.blueBright,
      'terminal.ansiBrightMagenta': palette.magentaBright,
      'terminal.ansiBrightCyan': palette.cyanBright,
      'terminal.ansiBrightWhite': '#ffffff'
    },
    tokenColors: [
      {
        scope: ['keyword', 'storage.type', 'storage.modifier'],
        settings: { foreground: palette.blueGreen }
      },
      {
        scope: ['entity.name.function', 'support.function'],
        settings: { foreground: palette.teal }
      },
      {
        scope: ['string', 'string.quoted'],
        settings: { foreground: palette.teal }
      },
      {
        scope: ['constant.numeric', 'constant.language'],
        settings: { foreground: palette.lavender }
      },
      {
        scope: ['comment', 'punctuation.definition.comment'],
        settings: { foreground: palette.gray, fontStyle: 'italic' }
      },
      {
        scope: ['entity.name.type', 'support.type', 'support.class'],
        settings: { foreground: palette.blueGreen }
      },
      {
        scope: ['variable', 'variable.other'],
        settings: { foreground: palette.fg }
      },
      {
        scope: ['constant', 'constant.other'],
        settings: { foreground: palette.magenta }
      },
      {
        scope: ['keyword.operator', 'punctuation'],
        settings: { foreground: palette.cyan }
      },
      {
        scope: ['entity.other.attribute-name', 'meta.attribute'],
        settings: { foreground: palette.magenta, fontStyle: 'italic' }
      },
      {
        scope: ['markup.heading'],
        settings: { foreground: palette.blue }
      },
      {
        scope: ['markup.bold'],
        settings: { foreground: palette.magenta, fontStyle: 'bold' }
      },
      {
        scope: ['markup.italic'],
        settings: { foreground: palette.magenta, fontStyle: 'italic' }
      },
      {
        scope: ['markup.inline.raw'],
        settings: { foreground: palette.lavender }
      }
    ]
  };
}

/**
 * Create the complete theme object
 */
export function createTheme(): Theme {
  return {
    colors: {
      // ANSI colors
      ansiBlack: palette.fgDim,
      ansiRed: palette.red,
      ansiGreen: palette.green,
      ansiYellow: palette.yellow,
      ansiBlue: palette.blue,
      ansiMagenta: palette.magenta,
      ansiCyan: palette.cyan,
      ansiWhite: '#d4d4d4',

      // Bright ANSI
      ansiBrightBlack: palette.fgDim,
      ansiBrightRed: palette.redBright,
      ansiBrightGreen: palette.greenBright,
      ansiBrightYellow: palette.yellowBright,
      ansiBrightBlue: palette.blueBright,
      ansiBrightMagenta: palette.magentaBright,
      ansiBrightCyan: palette.cyanBright,
      ansiBrightWhite: '#ffffff',

      // Dim ANSI
      ansiDimBlack: '#5a5a5a',
      ansiDimRed: palette.redDim,
      ansiDimGreen: palette.greenDim,
      ansiDimYellow: palette.yellowDim,
      ansiDimBlue: palette.blueDim,
      ansiDimMagenta: palette.magentaDim,
      ansiDimCyan: palette.cyanDim,
      ansiDimWhite: palette.fgMuted,

      // UI
      background: palette.bg,
      foreground: palette.fg,
      cursor: palette.cursor,
      selection: palette.selection,
      lineHighlight: palette.lineHighlight,

      // Syntax
      keyword: palette.blueGreen,
      function: palette.teal,
      string: palette.teal,
      number: palette.lavender,
      comment: palette.gray,
      type: palette.blueGreen,
      variable: palette.fg,
      constant: palette.magenta,
      operator: palette.cyan,
      attribute: palette.magenta
    },
    terminal: createTerminalTheme(),
    editor: createEditorTheme()
  };
}

/**
 * Default theme export
 */
export const theme = createTheme();

/**
 * Export individual theme parts
 */
export const terminal = theme.terminal;
export const editor = theme.editor;
