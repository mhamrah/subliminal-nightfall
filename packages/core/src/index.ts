/**
 * @subliminal/core
 *
 * Core color definitions and theme utilities for Subliminal Nightfall.
 * Single source of truth for all color values used across editors.
 */

export {
  baseColors,
  syntaxColors,
  backgroundColors,
  borderColors,
  uiColors,
  colors,
  palette
} from './colors.js';

export type {
  ColorVariant,
  SyntaxColors,
  BackgroundColors,
  BorderColors,
  UIColors
} from './colors.js';

export { theme, createTheme } from './theme.js';

export type {
  Theme,
  EditorTheme,
  TerminalTheme,
  ThemeColors
} from './theme.js';

/**
 * Package version
 */
export const VERSION = '1.0.0';

/**
 * Theme metadata
 */
export const metadata = {
  name: 'Subliminal Nightfall',
  description: 'A dark color scheme featuring deep purple-black backgrounds with carefully calibrated accent colors',
  author: 'Michael Hamrah',
  license: 'MIT',
  homepage: 'https://hamrah.com/subliminal-nightfall',
  repository: 'https://github.com/mhamrah/subliminal-nightfall',
  version: VERSION
} as const;
