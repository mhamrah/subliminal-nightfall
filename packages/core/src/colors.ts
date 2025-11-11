/**
 * Subliminal Nightfall - Core Color Definitions
 *
 * Single source of truth for all color values used across
 * VSCode, Neovim, Zed, Ghostty, and the showcase website.
 */

export interface ColorVariant {
  base: string;
  bright: string;
  dim: string;
}

export interface SyntaxColors {
  /** Cyan teal - Functions, methods, strings */
  teal: string;
  /** Blue green - Keywords, types, constructors */
  blueGreen: string;
  /** Lavender - Numbers, constants, inline code */
  lavender: string;
  /** Gray - Comments */
  gray: string;
}

export interface BackgroundColors {
  /** Deep purple-black editor background */
  background: string;
  /** Sidebar, panels, inactive tabs */
  backgroundAlt: string;
  /** Elevated elements, hover states */
  backgroundElevated: string;
  /** Soft white text */
  foreground: string;
  /** Muted text for less important content */
  foregroundMuted: string;
  /** Dimmed text for disabled states */
  foregroundDim: string;
}

export interface BorderColors {
  border: string;
  borderVariant: string;
  borderFocused: string;
  borderSelected: string;
}

export interface UIColors {
  selection: string;
  cursor: string;
  lineHighlight: string;
}

/**
 * Base ANSI colors with bright and dim variants
 * Calibrated for optimal terminal and editor contrast
 */
export const baseColors = {
  red: {
    base: '#bf616a',    // Nord red - errors, deletions
    bright: '#e2848d',
    dim: '#85434a'
  } as ColorVariant,

  green: {
    base: '#a9cfa4',    // Success, additions
    bright: '#ccf2c7',
    dim: '#769072'
  } as ColorVariant,

  yellow: {
    base: '#ffe2a9',    // Warnings, modifications
    bright: '#ffffcc',
    dim: '#b29e76'
  } as ColorVariant,

  blue: {
    base: '#6699cc',    // Info, titles, headings
    bright: '#89bcef',
    dim: '#476b8e'
  } as ColorVariant,

  magenta: {
    base: '#f1a5ab',    // Attributes, emphasis, booleans, operators
    bright: '#ffc8ce',
    dim: '#a87377'
  } as ColorVariant,

  cyan: {
    base: '#5fb3b3',    // Focus borders
    bright: '#82d6d6',
    dim: '#427d7d'
  } as ColorVariant
} as const;

/**
 * Syntax highlighting colors
 * Used for code tokens across all editors
 */
export const syntaxColors: SyntaxColors = {
  teal: '#9ccfd8',      // Functions, methods, strings
  blueGreen: '#31748f', // Keywords, types, constructors
  lavender: '#c4a7e7',  // Numbers, constants, inline code
  gray: '#7f7f7f'       // Comments
} as const;

/**
 * Background and foreground colors
 * Base layer for all UI elements
 */
export const backgroundColors: BackgroundColors = {
  background: '#191724',        // Deep purple-black
  backgroundAlt: '#1f1d2e',     // Sidebar, panels
  backgroundElevated: '#26233a', // Hover, elevated elements
  foreground: '#e0def4',        // Soft white text
  foregroundMuted: '#a0a0a0',   // Muted content
  foregroundDim: '#7f7f7f'      // Disabled states
} as const;

/**
 * Border colors for UI elements
 */
export const borderColors: BorderColors = {
  border: '#484e5b',
  borderVariant: '#363b45',
  borderFocused: '#6699cc',
  borderSelected: '#5fb3b3'
} as const;

/**
 * UI interaction colors
 */
export const uiColors: UIColors = {
  selection: '#484e5b',
  cursor: '#5fb3b3',
  lineHighlight: '#2e3239bf'
} as const;

/**
 * All colors exported as a single object
 */
export const colors = {
  base: baseColors,
  syntax: syntaxColors,
  background: backgroundColors,
  border: borderColors,
  ui: uiColors
} as const;

/**
 * Flatten all colors for easy access
 */
export const palette = {
  // Base colors
  red: baseColors.red.base,
  redBright: baseColors.red.bright,
  redDim: baseColors.red.dim,

  green: baseColors.green.base,
  greenBright: baseColors.green.bright,
  greenDim: baseColors.green.dim,

  yellow: baseColors.yellow.base,
  yellowBright: baseColors.yellow.bright,
  yellowDim: baseColors.yellow.dim,

  blue: baseColors.blue.base,
  blueBright: baseColors.blue.bright,
  blueDim: baseColors.blue.dim,

  magenta: baseColors.magenta.base,
  magentaBright: baseColors.magenta.bright,
  magentaDim: baseColors.magenta.dim,

  cyan: baseColors.cyan.base,
  cyanBright: baseColors.cyan.bright,
  cyanDim: baseColors.cyan.dim,

  // Syntax
  teal: syntaxColors.teal,
  blueGreen: syntaxColors.blueGreen,
  lavender: syntaxColors.lavender,
  gray: syntaxColors.gray,

  // Background
  bg: backgroundColors.background,
  bgAlt: backgroundColors.backgroundAlt,
  bgElevated: backgroundColors.backgroundElevated,
  fg: backgroundColors.foreground,
  fgMuted: backgroundColors.foregroundMuted,
  fgDim: backgroundColors.foregroundDim,

  // Borders
  border: borderColors.border,
  borderVariant: borderColors.borderVariant,
  borderFocused: borderColors.borderFocused,
  borderSelected: borderColors.borderSelected,

  // UI
  selection: uiColors.selection,
  cursor: uiColors.cursor,
  lineHighlight: uiColors.lineHighlight
} as const;

export default colors;
