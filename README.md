# Subliminal Nightfall (Zed, Ghostty, Cursor)


This repo packages the Subliminal color scheme for three editors/terminals:
- Zed (native theme JSON)
- Ghostty (terminal scheme)
- Cursor/VS Code (theme extension)

## Install

### Zed
- Copy themes/subliminal-nightfall.json to ~/.config/zed/themes/subliminal-nightfall.json
- In Zed: Command Palette → "Theme: Select Theme" → Subliminal Nightfall

### Ghostty
- Copy ghostty/subliminal-nightfall to ~/.config/ghostty/themes/subliminal-nightfall
- In ~/.config/ghostty/config add:
  ```
  theme = subliminal-nightfall
  ```

### Cursor (VS Code)
- From repo root: npm i -g @vscode/vsce && vsce package cursor
- In Cursor: Extensions → "Install from VSIX" → select the generated .vsix

## Development
- Zed theme source: themes/subliminal-nightfall.json (follows https://zed.dev/schema/themes/v0.2.0.json)
- Ghostty theme: ghostty/subliminal-nightfall (palette + background/foreground/cursor)
- Cursor theme: cursor/themes/Subliminal-color-theme.json + cursor/package.json

## Color Palette

Base colors with properly calibrated bright and dim variants:

| Color   | Base    | Bright  | Dim     | Usage                           |
|---------|---------|---------|---------|----------------------------------|
| Red     | #bf616a | #e2848d | #85434a | Errors, deletions               |
| Green   | #a9cfa4 | #ccf2c7 | #769072 | Success, additions              |
| Yellow  | #ffe2a9 | #ffffcc | #b29e76 | Warnings, modifications         |
| Blue    | #6699cc | #89bcef | #476b8e | Info, titles, predictive        |
| Magenta | #f1a5ab | #ffc8ce | #a87377 | Attributes, emphasis            |
| Cyan    | #5fb3b3 | #82d6d6 | #427d7d | Focus borders, operators        |

Background: #191724 (deep purple-black)  
Foreground: #e0def4 (soft white)

## Notes
- Red uses Nord red (#bf616a) for consistency with Nord color palette
- All bright colors are visibly brighter than base (not duplicates)
- Dim colors are 30% darker for proper terminal contrast
- Swift/Rust accents: cyan functions, blue‑green types/variants, pink attributes/macros, sand strings, lavender numbers/inline code
