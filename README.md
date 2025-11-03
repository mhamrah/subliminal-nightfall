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
- Copy ghostty/subliminal-nightfall.conf to ~/.config/ghostty/themes/subliminal-nightfall.conf
- In ~/.config/ghostty/config add:
  ```
  theme = subliminal-nightfall.conf
  ```

### Cursor (VS Code)
- From repo root: npm i -g @vscode/vsce && vsce package cursor
- In Cursor: Extensions → "Install from VSIX" → select the generated .vsix

## Development
- Zed theme source: themes/subliminal-nightfall.json (follows https://zed.dev/schema/themes/v0.2.0.json)
- Ghostty theme: ghostty/subliminal-nightfall.conf (palette + background/foreground/cursor)
- Cursor theme: cursor/themes/Subliminal-color-theme.json + cursor/package.json

## Notes
- Palette from iTerm2 Subliminal: #e15a60 #a9cfa4 #ffe2a9 #6699cc #f1a5ab #5fb3b3 with dark base #191724.
- Swift/Rust accents: cyan functions, blue‑green types/variants, pink attributes/macros, sand strings, lavender numbers/inline code.
