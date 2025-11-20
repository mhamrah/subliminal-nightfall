use crate::config::{Config, Variant};
use anyhow::{anyhow, Result};
use serde_json::json;
use std::{fs, path::PathBuf};

fn strip_alpha(hex: &str) -> String {
    let h = hex.trim_start_matches('#');
    if h.len() >= 6 { format!("#{}", &h[0..6]) } else { format!("#{}", h) }
}

fn apply_alpha(hex: &str, a: f32) -> String {
    let a = a.clamp(0.0, 1.0);
    let alpha = ((a * 255.0).round() as u8) as u8;
    let h = hex.trim_start_matches('#');
    let base = if h.len() >= 6 { &h[0..6] } else { h };
    format!("#{}{:02X}", base, alpha)
}

fn ui_with_variant(cfg: &Config, variant: &Variant) -> crate::config::UiPalette {
    let mut ui = cfg.palette.ui.clone();
    if let Some(alpha) = variant.alpha {
        // Apply alpha to key backgrounds by default
        ui.background = apply_alpha(&ui.background, alpha);
        ui.background_alt = apply_alpha(&ui.background_alt, alpha);
        ui.background_elevated = apply_alpha(&ui.background_elevated, alpha);
        ui.selection = apply_alpha(&ui.selection, alpha);
        ui.line_highlight = apply_alpha(&ui.line_highlight, alpha);
    }
    if let Some(ov) = &variant.overrides {
        if let Some(uo) = &ov.ui {
            if let Some(v) = &uo.background { ui.background = v.clone(); }
            if let Some(v) = &uo.background_alt { ui.background_alt = v.clone(); }
            if let Some(v) = &uo.background_elevated { ui.background_elevated = v.clone(); }
            if let Some(v) = &uo.selection { ui.selection = v.clone(); }
            if let Some(v) = &uo.cursor { ui.cursor = v.clone(); }
            if let Some(v) = &uo.line_highlight { ui.line_highlight = v.clone(); }
            if let Some(v) = &uo.foreground { ui.foreground = v.clone(); }
            if let Some(v) = &uo.foreground_muted { ui.foreground_muted = v.clone(); }
            if let Some(v) = &uo.foreground_dim { ui.foreground_dim = v.clone(); }
        }
    }
    ui
}

pub fn generate_target(cfg: &Config, target: &crate::config::Target, root: &PathBuf) -> Result<()> {
    match target.id.as_str() {
        "ghostty" => gen_ghostty(cfg, target, root),
        "zed" => gen_zed(cfg, target, root),
        "cursor" => gen_cursor(cfg, target, root),
        "neovim" => gen_neovim(cfg, target, root),
        "website" => gen_website(cfg, target, root),
        other => Err(anyhow!("Unknown target id: {}", other)),
    }
}

fn gen_ghostty(cfg: &Config, target: &crate::config::Target, root: &PathBuf) -> Result<()> {
    let dir = root.join(&target.path);
    fs::create_dir_all(&dir)?;
    for v in &cfg.variants {
        let ui = ui_with_variant(cfg, v);
        let name = target
            .out_names
            .as_ref()
            .and_then(|m| m.get(&v.name))
            .cloned()
            .unwrap_or_else(|| format!("{}-{}", cfg.meta.name.to_lowercase().replace(' ', "-"), v.name));
        let mut out = String::new();
        out.push_str(&format!("# {} for Ghostty\n", cfg.meta.name));
        out.push_str("palette = 0=#7f7f7f\n");
        out.push_str(&format!("palette = 1={}\n", cfg.palette.base.ansi.red.base));
        out.push_str(&format!("palette = 2={}\n", cfg.palette.base.ansi.green.base));
        out.push_str(&format!("palette = 3={}\n", cfg.palette.base.ansi.yellow.base));
        out.push_str(&format!("palette = 4={}\n", cfg.palette.base.ansi.blue.base));
        out.push_str(&format!("palette = 5={}\n", cfg.palette.base.ansi.magenta.base));
        out.push_str(&format!("palette = 6={}\n", cfg.palette.base.ansi.cyan.base));
        out.push_str("palette = 7=#d4d4d4\n");
        out.push_str("palette = 8=#7f7f7f\n");
        out.push_str(&format!("palette = 9={}\n", cfg.palette.base.ansi.red.bright));
        out.push_str(&format!("palette = 10={}\n", cfg.palette.base.ansi.green.bright));
        out.push_str(&format!("palette = 11={}\n", cfg.palette.base.ansi.yellow.bright));
        out.push_str(&format!("palette = 12={}\n", cfg.palette.base.ansi.blue.bright));
        out.push_str(&format!("palette = 13={}\n", cfg.palette.base.ansi.magenta.bright));
        out.push_str(&format!("palette = 14={}\n", cfg.palette.base.ansi.cyan.bright));
        out.push_str("palette = 15=#ffffff\n");
        out.push_str(&format!("background = {}\n", strip_alpha(&ui.background)));
        out.push_str(&format!("foreground = {}\n", strip_alpha(&ui.foreground)));
        out.push_str(&format!("cursor-color = {}\n", strip_alpha(&ui.cursor)));
        out.push_str("cursor-text = #ffffff\n");
        out.push_str(&format!("selection-background = {}\n", strip_alpha(&ui.selection)));
        out.push_str("selection-foreground = #ffffff\n");
        fs::write(dir.join(name), out)?;
    }
    Ok(())
}

fn gen_zed(cfg: &Config, target: &crate::config::Target, root: &PathBuf) -> Result<()> {
    let dir = root.join(&target.path);
    fs::create_dir_all(&dir)?;
    let mut themes = vec![];
    for v in &cfg.variants {
        let ui = ui_with_variant(cfg, v);
        let title = match v.name.as_str() {
            "base" => cfg.meta.name.clone(),
            other => format!("{} {}", cfg.meta.name, capitalize(other)),
        };
        let theme = json!({
            "name": title,
            "appearance": "dark",
            "style": {
                "border": cfg.palette.border.border,
                "border.variant": cfg.palette.border.border_variant,
                "border.focused": cfg.palette.border.border_focused,
                "border.selected": cfg.palette.border.border_selected,
                "text": ui.foreground,
                "text.muted": cfg.palette.ui.foreground_muted,
                "background": ui.background,
                "surface.background": ui.background_alt,
                "editor.background": ui.background,
                "editor.foreground": ui.foreground,
                "editor.selection.background": ui.selection,
                "editor.selection.foreground": "#ffffff",
                "editor.active_line.background": ui.line_highlight,
                "terminal.background": ui.background,
                "terminal.foreground": ui.foreground,
                "terminal.ansi.black": cfg.palette.ui.foreground_dim,
                "terminal.ansi.red": cfg.palette.base.ansi.red.base,
                "terminal.ansi.green": cfg.palette.base.ansi.green.base,
                "terminal.ansi.yellow": cfg.palette.base.ansi.yellow.base,
                "terminal.ansi.blue": cfg.palette.base.ansi.blue.base,
                "terminal.ansi.magenta": cfg.palette.base.ansi.magenta.base,
                "terminal.ansi.cyan": cfg.palette.base.ansi.cyan.base,
                "terminal.ansi.white": "#d4d4d4",
                "terminal.ansi.bright_black": cfg.palette.ui.foreground_dim,
                "terminal.ansi.bright_red": cfg.palette.base.ansi.red.bright,
                "terminal.ansi.bright_green": cfg.palette.base.ansi.green.bright,
                "terminal.ansi.bright_yellow": cfg.palette.base.ansi.yellow.bright,
                "terminal.ansi.bright_blue": cfg.palette.base.ansi.blue.bright,
                "terminal.ansi.bright_magenta": cfg.palette.base.ansi.magenta.bright,
                "terminal.ansi.bright_cyan": cfg.palette.base.ansi.cyan.bright,
                "terminal.ansi.bright_white": "#ffffff"
            },
            "syntax": {
                "comment": {"color": cfg.palette.syntax.gray, "font_style": "italic"},
                "keyword": {"color": cfg.palette.syntax.blue_green},
                "function": {"color": cfg.palette.syntax.teal},
                "string": {"color": cfg.palette.syntax.teal},
                "number": {"color": cfg.palette.syntax.lavender},
                "operator": {"color": cfg.palette.base.ansi.cyan.base},
                "attribute": {"color": cfg.palette.base.ansi.magenta.base, "font_style": "italic"}
            }
        });
        themes.push(theme);
    }
    let root_obj = json!({
        "$schema": "https://zed.dev/schema/themes/v0.2.0.json",
        "name": cfg.meta.name,
        "author": cfg.meta.author.clone().unwrap_or_default(),
        "themes": themes
    });
    let file = target.out_file.clone().unwrap_or_else(|| format!("{}.json", cfg.meta.name.to_lowercase().replace(' ', "-")));
    fs::write(dir.join(file), serde_json::to_string_pretty(&root_obj)?)?;
    Ok(())
}

fn gen_cursor(cfg: &Config, target: &crate::config::Target, root: &PathBuf) -> Result<()> {
    // Generate base variant as primary file; others if out_names provided
    let dir = root.join(&target.path);
    fs::create_dir_all(&dir)?;
    for v in &cfg.variants {
        let ui = ui_with_variant(cfg, v);
        let name = target
            .out_names
            .as_ref()
            .and_then(|m| m.get(&v.name))
            .cloned()
            .unwrap_or_else(|| format!("{}-{}.json", cfg.meta.name.to_lowercase().replace(' ', "-"), v.name));
        // Minimal VS Code theme JSON
        let variant_suffix = if v.name == "base" { String::new() } else { format!(" ({})", capitalize(&v.name)) };
        let theme = json!({
            "name": format!("{}{}", cfg.meta.name, variant_suffix),
            "type": "dark",
            "colors": {
                "editor.background": ui.background,
                "editor.foreground": ui.foreground,
                "editor.lineHighlightBackground": ui.line_highlight,
                "editor.selectionBackground": ui.selection,
                "editorCursor.foreground": ui.cursor,
                "sideBar.background": cfg.palette.ui.background_alt,
                "sideBar.foreground": cfg.palette.ui.foreground,
                "sideBar.border": cfg.palette.border.border,
                "terminal.background": ui.background,
                "terminal.foreground": ui.foreground,
                "terminal.ansiBlack": cfg.palette.ui.foreground_dim,
                "terminal.ansiRed": cfg.palette.base.ansi.red.base,
                "terminal.ansiGreen": cfg.palette.base.ansi.green.base,
                "terminal.ansiYellow": cfg.palette.base.ansi.yellow.base,
                "terminal.ansiBlue": cfg.palette.base.ansi.blue.base,
                "terminal.ansiMagenta": cfg.palette.base.ansi.magenta.base,
                "terminal.ansiCyan": cfg.palette.base.ansi.cyan.base,
                "terminal.ansiWhite": "#d4d4d4",
                "terminal.ansiBrightBlack": cfg.palette.ui.foreground_dim,
                "terminal.ansiBrightRed": cfg.palette.base.ansi.red.bright,
                "terminal.ansiBrightGreen": cfg.palette.base.ansi.green.bright,
                "terminal.ansiBrightYellow": cfg.palette.base.ansi.yellow.bright,
                "terminal.ansiBrightBlue": cfg.palette.base.ansi.blue.bright,
                "terminal.ansiBrightMagenta": cfg.palette.base.ansi.magenta.bright,
                "terminal.ansiBrightCyan": cfg.palette.base.ansi.cyan.bright,
                "terminal.ansiBrightWhite": "#ffffff"
            },
            "tokenColors": [
                {"scope": ["keyword", "storage.type", "storage.modifier"], "settings": {"foreground": cfg.palette.syntax.blue_green}},
                {"scope": ["entity.name.function", "support.function"], "settings": {"foreground": cfg.palette.syntax.teal}},
                {"scope": ["string", "string.quoted"], "settings": {"foreground": cfg.palette.syntax.teal}},
                {"scope": ["constant.numeric", "constant.language"], "settings": {"foreground": cfg.palette.syntax.lavender}},
                {"scope": ["comment", "punctuation.definition.comment"], "settings": {"foreground": cfg.palette.syntax.gray, "fontStyle": "italic"}}
            ]
        });
        fs::write(dir.join(name), serde_json::to_string_pretty(&theme)?)?;
    }
    Ok(())
}

fn gen_neovim(cfg: &Config, target: &crate::config::Target, root: &PathBuf) -> Result<()> {
    let dir = root.join(&target.path);
    fs::create_dir_all(&dir)?;
    for v in &cfg.variants {
        let ui = ui_with_variant(cfg, v);
        let name = target
            .out_names
            .as_ref()
            .and_then(|m| m.get(&v.name))
            .cloned()
            .unwrap_or_else(|| format!("{}-{}.lua", cfg.meta.name.to_lowercase().replace(' ', "-"), v.name));
        let variant_suffix = if v.name == "base" { String::new() } else { format!(" ({})", capitalize(&v.name)) };
        let lua = format!(r#"-- Generated by colorloom
vim.cmd('highlight clear')
if vim.fn.exists('syntax_on') then vim.cmd('syntax reset') end
vim.g.colors_name = '{}{}'
vim.o.background = 'dark'
local c = {{
  bg = '{}', bg_alt = '{}', fg = '{}', fg_muted = '{}', fg_dim = '{}',
  selection = '{}', cursor = '{}', line = '{}',
  red = '{}', green = '{}', yellow = '{}', blue = '{}', magenta = '{}', cyan = '{}', purple = '{}', teal = '{}', dark_blue = '{}'
}}
local function hl(g, o) vim.api.nvim_set_hl(0, g, o) end
hl('Normal', {{ fg = c.fg, bg = c.bg }})
hl('CursorLine', {{ bg = c.line }})
hl('Visual', {{ bg = c.selection, fg = '#ffffff' }})
hl('Comment', {{ fg = c.fg_dim, italic = true }})
hl('String', {{ fg = c.teal }})
hl('Number', {{ fg = c.purple }})
hl('Function', {{ fg = c.teal }})
hl('Keyword', {{ fg = c.dark_blue }})
"#,
            cfg.meta.name,
            variant_suffix,
            ui.background,
            cfg.palette.ui.background_alt,
            ui.foreground,
            cfg.palette.ui.foreground_muted,
            cfg.palette.ui.foreground_dim,
            ui.selection,
            ui.cursor,
            ui.line_highlight,
            cfg.palette.base.ansi.red.base,
            cfg.palette.base.ansi.green.base,
            cfg.palette.base.ansi.yellow.base,
            cfg.palette.base.ansi.blue.base,
            cfg.palette.base.ansi.magenta.base,
            cfg.palette.base.ansi.cyan.base,
            cfg.palette.syntax.lavender,
            cfg.palette.syntax.teal,
            cfg.palette.syntax.blue_green,
        );
        fs::write(dir.join(name), lua)?;
    }
    Ok(())
}

fn gen_website(cfg: &Config, target: &crate::config::Target, root: &PathBuf) -> Result<()> {
    let dir = root.join(&target.path);
    std::fs::create_dir_all(&dir)?;
    // Build arrays matching the website component expectations
    let colors = vec![
        json!({"name":"Red","base":cfg.palette.base.ansi.red.base,"bright":cfg.palette.base.ansi.red.bright,"dim":cfg.palette.base.ansi.red.dim,"usage":"Errors, deletions, keywords"}),
        json!({"name":"Green","base":cfg.palette.base.ansi.green.base,"bright":cfg.palette.base.ansi.green.bright,"dim":cfg.palette.base.ansi.green.dim,"usage":"Success, additions"}),
        json!({"name":"Yellow","base":cfg.palette.base.ansi.yellow.base,"bright":cfg.palette.base.ansi.yellow.bright,"dim":cfg.palette.base.ansi.yellow.dim,"usage":"Warnings, modifications"}),
        json!({"name":"Blue","base":cfg.palette.base.ansi.blue.base,"bright":cfg.palette.base.ansi.blue.bright,"dim":cfg.palette.base.ansi.blue.dim,"usage":"Info, titles, headings"}),
        json!({"name":"Magenta","base":cfg.palette.base.ansi.magenta.base,"bright":cfg.palette.base.ansi.magenta.bright,"dim":cfg.palette.base.ansi.magenta.dim,"usage":"Attributes, emphasis, operators"}),
        json!({"name":"Cyan","base":cfg.palette.base.ansi.cyan.base,"bright":cfg.palette.base.ansi.cyan.bright,"dim":cfg.palette.base.ansi.cyan.dim,"usage":"Focus borders"}),
    ];
    let syntax_colors = vec![
        json!({"name":"Cyan Teal","hex":cfg.palette.syntax.teal,"usage":"Functions, methods, strings"}),
        json!({"name":"Blue Green","hex":cfg.palette.syntax.blue_green,"usage":"Keywords, types, constructors"}),
        json!({"name":"Lavender","hex":cfg.palette.syntax.lavender,"usage":"Numbers, constants, inline code"}),
        json!({"name":"Gray","hex":cfg.palette.syntax.gray,"usage":"Comments"}),
    ];
    let background_colors = vec![
        json!({"name":"Background","hex":cfg.palette.ui.background,"usage":"Deep purple-black editor background"}),
        json!({"name":"Background Alt","hex":cfg.palette.ui.background_alt,"usage":"Sidebar, panels, inactive tabs"}),
        json!({"name":"Foreground","hex":cfg.palette.ui.foreground,"usage":"Soft white text","textColor":"#000"}),
    ];

    let obj = json!({
        "meta": {"name": cfg.meta.name, "author": cfg.meta.author},
        "colors": colors,
        "syntaxColors": syntax_colors,
        "backgroundColors": background_colors
    });
    let file = target.out_file.clone().unwrap_or_else(|| "palette.json".to_string());
    std::fs::write(dir.join(file), serde_json::to_string_pretty(&obj)?)?;
    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut ch = s.chars();
    match ch.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + ch.as_str(),
    }
}
