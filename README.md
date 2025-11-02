# Subliminal Nightfall (Zed, Ghostty, Cursor)

Last updated: 2025-11-02T02:38:59.253Z

This repo packages the Subliminal color scheme for three editors/terminals:
- Zed (native theme JSON)
- Ghostty (terminal scheme)
- Cursor/VS Code (theme extension)

## Install

### Zed
- Copy zed/themes/Subliminal.json to ~/.config/zed/themes/Subliminal.json
- In Zed: Command Palette → "Theme: Select Theme" → Subliminal Nightfall

### Ghostty
- Copy ghostty/Subliminal.conf to ~/.config/ghostty/themes/Subliminal.conf
- In ~/.config/ghostty/config add:
  ```
  theme = ~/.config/ghostty/themes/Subliminal.conf
  ```

### Cursor (VS Code)
- cd cursor && npm i -g @vscode/vsce && vsce package
- In Cursor: Extensions → "Install from VSIX" → select the generated .vsix

## Development
- Zed theme source: zed/themes/Subliminal.json (follows https://zed.dev/schema/themes/v0.2.0.json)
- Ghostty theme: ghostty/Subliminal.conf (palette + background/foreground/cursor)
- Cursor theme: cursor/themes/Subliminal-color-theme.json + cursor/package.json

## Notes
- Palette from iTerm2 Subliminal: #e15a60 #a9cfa4 #ffe2a9 #6699cc #f1a5ab #5fb3b3 with dark base #191724.
- Swift/Rust accents: cyan functions, blue‑green types/variants, pink attributes/macros, sand strings, lavender numbers/inline code.
