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
    // Apply overrides after alpha (overrides take precedence)
    if let Some(ov) = &variant.overrides {
        if let Some(uo) = &ov.ui {
            // For overrides, apply alpha if specified and the override doesn't already have alpha
            if let Some(v) = &uo.background { 
                ui.background = if variant.alpha.is_some() && v.len() <= 7 {
                    apply_alpha(v, variant.alpha.unwrap())
                } else { v.clone() };
            }
            if let Some(v) = &uo.background_alt { 
                ui.background_alt = if variant.alpha.is_some() && v.len() <= 7 {
                    apply_alpha(v, variant.alpha.unwrap())
                } else { v.clone() };
            }
            if let Some(v) = &uo.background_elevated { 
                ui.background_elevated = if variant.alpha.is_some() && v.len() <= 7 {
                    apply_alpha(v, variant.alpha.unwrap())
                } else { v.clone() };
            }
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
        if let Some(alpha) = v.alpha {
            if alpha < 1.0 {
                out.push_str(&format!("background-opacity = {}\n", alpha));
                out.push_str("background-blur = true\n");
                out.push_str("window-decoration = true\n");
            }
        }
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
        
        // Determine if this is a blurred/transparent variant
        let alpha = v.alpha.unwrap_or(1.0);
        let is_blurred = alpha < 1.0;
        let appearance_mode = if is_blurred { "blurred" } else { "opaque" };
        
        // Convert alpha to hex suffix
        let alpha_hex = |a: f32| -> String { format!("{:02X}", (a * 255.0).round() as u8) };
        
        let base_color = strip_alpha(&cfg.palette.ui.background);
        let alt_color = strip_alpha(&cfg.palette.ui.background_alt);
        let elevated_color = strip_alpha(&cfg.palette.ui.background_elevated);
        
        // Different transparency levels based on variant alpha:
        // - Hazy (alpha=0.70): more transparent, lower opacity values
        // - Cloudy (alpha=0.85): less transparent, higher opacity values
        // - Base (alpha=1.0): fully opaque
        let (editor_bg, gutter_bg, panel_bg, surface_bg, elevated_bg, tab_active_bg, tab_inactive_bg, tab_bar_bg, toolbar_bg, list_bg, menu_bg, scrollbar_track_bg) = if is_blurred {
            // Calculate proportional opacities based on the variant's alpha
            let container_alpha = alpha_hex(alpha);                    // containers: variant alpha (70% or 85%)
            let subtle_alpha = alpha_hex(alpha * 0.5);                 // subtle elements: half of variant alpha
            let very_subtle_alpha = alpha_hex(alpha * 0.3);            // very subtle: 30% of variant alpha
            
            (
                format!("{}00", base_color),                           // editor: fully transparent
                format!("{}00", base_color),                           // gutter: fully transparent  
                format!("{}00", alt_color),                            // panel: fully transparent (blur shows through)
                format!("{}{}", alt_color, container_alpha),           // surface: variant alpha
                format!("{}{}", elevated_color, container_alpha),      // elevated: variant alpha
                format!("{}{}", base_color, subtle_alpha),             // active tab: 50% of variant alpha
                format!("{}{}", alt_color, very_subtle_alpha),         // inactive tab: 30% of variant alpha
                format!("{}{}", alt_color, very_subtle_alpha),         // tab bar: 30% of variant alpha
                format!("{}{}", alt_color, subtle_alpha),              // toolbar: 50% of variant alpha
                format!("{}{}", alt_color, container_alpha),           // list backgrounds: variant alpha
                format!("{}{}", elevated_color, container_alpha),      // menu/popover: variant alpha
                format!("{}{}", alt_color, very_subtle_alpha),         // scrollbar track: subtle
            )
        } else {
            // Opaque variant: full opacity everywhere
            (
                format!("{}FF", base_color),
                format!("{}FF", base_color),
                format!("{}FF", alt_color),
                format!("{}FF", alt_color),
                format!("{}FF", elevated_color),
                format!("{}80", base_color),
                format!("{}80", alt_color),
                format!("{}80", alt_color),
                format!("{}80", alt_color),
                format!("{}FF", alt_color),
                format!("{}FF", elevated_color),
                format!("{}4D", alt_color),
            )
        };
        
        // Status bar and title bar backgrounds - use container alpha for blurred
        let container_alpha = if is_blurred { alpha_hex(alpha) } else { "FF".to_string() };
        let status_bar_bg = format!("{}{}", alt_color, container_alpha);
        let title_bar_bg = format!("{}{}", alt_color, container_alpha);
        let terminal_bg = if is_blurred { format!("{}00", alt_color) } else { format!("{}FF", alt_color) };
        let border_alpha = if is_blurred { alpha_hex(alpha * 0.5) } else { "CC".to_string() };
        
        let theme = json!({
            "name": title,
            "appearance": "dark",
            "style": {
                "background.appearance": appearance_mode,
                
                // Borders (with appropriate alpha for blurred)
                "border": format!("{}{}", strip_alpha(&cfg.palette.border.border), border_alpha),
                "border.variant": cfg.palette.border.border_variant,
                "border.focused": cfg.palette.border.border_focused,
                "border.selected": cfg.palette.border.border_selected,
                "border.transparent": format!("{}00", base_color),
                "border.disabled": cfg.palette.border.border_variant,
                
                // Text
                "text": ui.foreground,
                "text.muted": cfg.palette.ui.foreground_muted,
                "text.placeholder": cfg.palette.ui.foreground_dim,
                "text.disabled": cfg.palette.ui.foreground_dim,
                "text.accent": cfg.palette.base.ansi.cyan.base,
                
                // Main backgrounds
                "background": if is_blurred { format!("{}CC", base_color) } else { format!("{}FF", base_color) },
                "surface.background": surface_bg,
                "elevated_surface.background": elevated_bg,
                "element.background": format!("{}{}", strip_alpha(&cfg.palette.border.border_variant), if is_blurred { container_alpha.clone() } else { "FF".to_string() }),
                "element.hover": format!("{}{}", strip_alpha(&cfg.palette.border.border), if is_blurred { container_alpha.clone() } else { "FF".to_string() }),
                "element.active": format!("{}{}", strip_alpha(&ui.selection), if is_blurred { container_alpha.clone() } else { "FF".to_string() }),
                "element.selected": format!("{}{}", strip_alpha(&ui.selection), if is_blurred { container_alpha.clone() } else { "FF".to_string() }),
                "element.disabled": format!("{}{}", strip_alpha(&cfg.palette.border.border_variant), if is_blurred { alpha_hex(alpha * 0.3) } else { "80".to_string() }),
                "ghost_element.background": format!("{}00", alt_color),
                "ghost_element.hover": format!("{}{}", strip_alpha(&cfg.palette.border.border_variant), if is_blurred { alpha_hex(alpha * 0.3) } else { "4D".to_string() }),
                "ghost_element.active": format!("{}{}", strip_alpha(&cfg.palette.border.border), if is_blurred { container_alpha.clone() } else { "FF".to_string() }),
                "ghost_element.selected": format!("{}{}", strip_alpha(&ui.selection), if is_blurred { container_alpha.clone() } else { "CC".to_string() }),
                "ghost_element.disabled": format!("{}00", alt_color),
                "drop_target.background": format!("{}{}", strip_alpha(&cfg.palette.border.border_variant), if is_blurred { container_alpha.clone() } else { "FF".to_string() }),
                
                // Editor
                "editor.background": editor_bg,
                "editor.foreground": ui.foreground,
                "editor.gutter.background": gutter_bg,
                "editor.subheader.background": surface_bg,
                "editor.active_line.background": if is_blurred { format!("{}4D", strip_alpha(&ui.line_highlight)) } else { ui.line_highlight.clone() },
                "editor.highlighted_line.background": cfg.palette.border.border_variant,
                "editor.line_number": cfg.palette.ui.foreground_dim,
                "editor.active_line_number": ui.foreground,
                "editor.invisible": cfg.palette.ui.foreground_dim,
                "editor.wrap_guide": cfg.palette.border.border_variant,
                "editor.active_wrap_guide": cfg.palette.border.border,
                "editor.document_highlight.read_background": cfg.palette.border.border_variant,
                "editor.document_highlight.write_background": cfg.palette.border.border,
                
                // Selections
                "editor.selection.background": ui.selection,
                "editor.selection.foreground": "#ffffff",
                
                // Title bar
                "title_bar.background": title_bar_bg,
                "title_bar.inactive_background": title_bar_bg,
                
                // Tab bar
                "tab_bar.background": tab_bar_bg,
                "tab.inactive_background": tab_inactive_bg,
                "tab.active_background": tab_active_bg,
                
                // Toolbar
                "toolbar.background": toolbar_bg,
                
                // Search
                "search.match_background": format!("{}{}", strip_alpha(&cfg.palette.base.ansi.yellow.dim), if is_blurred { "CC" } else { "FF" }),
                
                // Minimap
                "minimap.thumb.background": format!("{}{}", strip_alpha(&cfg.palette.border.border), if is_blurred { container_alpha.clone() } else { "CC".to_string() }),
                "minimap.thumb.hover_background": cfg.palette.border.border_variant,
                "minimap.thumb.active_background": format!("{}80", strip_alpha(&cfg.palette.base.ansi.cyan.base)),
                
                // Accents (used for various UI highlights)
                "accents": [
                    format!("{}80", strip_alpha(&cfg.palette.base.ansi.blue.base)),
                    format!("{}80", strip_alpha(&cfg.palette.base.ansi.cyan.base)),
                    format!("{}80", strip_alpha(&cfg.palette.base.ansi.magenta.base)),
                    format!("{}80", strip_alpha(&cfg.palette.base.ansi.green.base)),
                    format!("{}80", strip_alpha(&cfg.palette.base.ansi.yellow.base)),
                    format!("{}80", strip_alpha(&cfg.palette.base.ansi.red.base)),
                ],
                
                // Panel
                "panel.background": panel_bg,
                "panel.focused_border": cfg.palette.border.border_focused,
                "panel.overlay_background": menu_bg,
                
                // Hidden elements
                "hidden": cfg.palette.ui.foreground_dim,
                "ignored": cfg.palette.ui.foreground_dim,
                
                // Debugger
                "debugger.accent": cfg.palette.base.ansi.yellow.base,
                
                // Pane
                "pane.focused_border": cfg.palette.border.border_focused,
                
                // Scrollbar
                "scrollbar.thumb.background": format!("{}{}", strip_alpha(&cfg.palette.border.border), if is_blurred { container_alpha.clone() } else { "CC".to_string() }),
                "scrollbar.thumb.hover_background": cfg.palette.border.border_variant,
                "scrollbar.thumb.border": cfg.palette.border.border,
                "scrollbar.track.background": scrollbar_track_bg,
                "scrollbar.track.border": null,
                
                // Status bar
                "status_bar.background": status_bar_bg,
                
                // Notifications/popover
                "notification.background": elevated_bg,
                "notification.border": cfg.palette.border.border,
                
                // Version control
                "version_control.added": cfg.palette.base.ansi.green.base,
                "version_control.modified": cfg.palette.base.ansi.yellow.base,
                "version_control.deleted": cfg.palette.base.ansi.red.base,
                "version_control.renamed": cfg.palette.base.ansi.blue.base,
                "version_control.conflict": cfg.palette.base.ansi.magenta.base,
                "version_control.ignored": cfg.palette.ui.foreground_dim,
                
                // Conflict
                "conflict": cfg.palette.base.ansi.magenta.base,
                "conflict.background": "#f1a5ab20",
                "conflict.border": cfg.palette.base.ansi.magenta.dim,
                
                // Created/modified/deleted
                "created": cfg.palette.base.ansi.green.base,
                "created.background": "#a9cfa420",
                "created.border": cfg.palette.base.ansi.green.dim,
                "modified": cfg.palette.base.ansi.yellow.base,
                "modified.background": "#ffe2a920",
                "modified.border": cfg.palette.base.ansi.yellow.dim,
                "deleted": cfg.palette.base.ansi.red.base,
                "deleted.background": "#bf616a20",
                "deleted.border": cfg.palette.base.ansi.red.dim,
                
                // Hints/errors/warnings/info
                "hint": cfg.palette.ui.foreground_dim,
                "hint.background": format!("{}{}", strip_alpha(&cfg.palette.border.border_variant), if is_blurred { container_alpha.clone() } else { "FF".to_string() }),
                "hint.border": cfg.palette.ui.foreground_dim,
                "predictive": cfg.palette.ui.foreground_dim,
                "predictive.background": null,
                "predictive.border": cfg.palette.ui.foreground_dim,
                "renamed": cfg.palette.base.ansi.blue.base,
                "renamed.background": "#6699cc20",
                "renamed.border": cfg.palette.base.ansi.blue.dim,
                "success": cfg.palette.base.ansi.green.base,
                "success.background": "#a9cfa420",
                "success.border": cfg.palette.base.ansi.green.dim,
                "warning": cfg.palette.base.ansi.yellow.base,
                "warning.background": "#ffe2a920",
                "warning.border": cfg.palette.base.ansi.yellow.dim,
                "error": cfg.palette.base.ansi.red.base,
                "error.background": "#bf616a20",
                "error.border": cfg.palette.base.ansi.red.dim,
                "info": cfg.palette.base.ansi.blue.base,
                "info.background": "#6699cc20",
                "info.border": cfg.palette.base.ansi.blue.dim,
                
                // Utility
                "icon": cfg.palette.ui.foreground_muted,
                "icon.muted": cfg.palette.ui.foreground_dim,
                "icon.disabled": cfg.palette.ui.foreground_dim,
                "icon.placeholder": cfg.palette.ui.foreground_dim,
                "icon.accent": cfg.palette.base.ansi.cyan.base,
                
                // Link
                "link_text.hover": cfg.palette.base.ansi.blue.bright,
                
                // Players (for collaborative features)
                "players": [
                    { "cursor": cfg.palette.base.ansi.cyan.base, "background": cfg.palette.base.ansi.cyan.base, "selection": format!("{}40", cfg.palette.base.ansi.cyan.base) },
                    { "cursor": cfg.palette.base.ansi.magenta.base, "background": cfg.palette.base.ansi.magenta.base, "selection": format!("{}40", cfg.palette.base.ansi.magenta.base) },
                    { "cursor": cfg.palette.base.ansi.green.base, "background": cfg.palette.base.ansi.green.base, "selection": format!("{}40", cfg.palette.base.ansi.green.base) },
                    { "cursor": cfg.palette.base.ansi.yellow.base, "background": cfg.palette.base.ansi.yellow.base, "selection": format!("{}40", cfg.palette.base.ansi.yellow.base) },
                    { "cursor": cfg.palette.base.ansi.blue.base, "background": cfg.palette.base.ansi.blue.base, "selection": format!("{}40", cfg.palette.base.ansi.blue.base) },
                    { "cursor": cfg.palette.base.ansi.red.base, "background": cfg.palette.base.ansi.red.base, "selection": format!("{}40", cfg.palette.base.ansi.red.base) }
                ],
                
                // Terminal
                "terminal.background": terminal_bg,
                "terminal.foreground": ui.foreground,
                "terminal.bright_foreground": "#ffffff",
                "terminal.dim_foreground": cfg.palette.ui.foreground_dim,
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
                "terminal.ansi.bright_white": "#ffffff",
                "terminal.ansi.dim_black": cfg.palette.ui.foreground_dim,
                "terminal.ansi.dim_red": cfg.palette.base.ansi.red.dim,
                "terminal.ansi.dim_green": cfg.palette.base.ansi.green.dim,
                "terminal.ansi.dim_yellow": cfg.palette.base.ansi.yellow.dim,
                "terminal.ansi.dim_blue": cfg.palette.base.ansi.blue.dim,
                "terminal.ansi.dim_magenta": cfg.palette.base.ansi.magenta.dim,
                "terminal.ansi.dim_cyan": cfg.palette.base.ansi.cyan.dim,
                "terminal.ansi.dim_white": cfg.palette.ui.foreground_muted
            },
            "syntax": {
                "comment": {"color": cfg.palette.syntax.gray, "font_style": "italic"},
                "keyword": {"color": cfg.palette.syntax.blue_green},
                "function": {"color": cfg.palette.syntax.teal},
                "string": {"color": cfg.palette.syntax.teal},
                "number": {"color": cfg.palette.syntax.lavender},
                "operator": {"color": cfg.palette.base.ansi.cyan.base},
                "attribute": {"color": cfg.palette.base.ansi.magenta.base, "font_style": "italic"},
                "type": {"color": cfg.palette.base.ansi.yellow.base},
                "variable": {"color": ui.foreground},
                "variable.special": {"color": cfg.palette.base.ansi.magenta.base},
                "constant": {"color": cfg.palette.syntax.lavender},
                "property": {"color": cfg.palette.base.ansi.blue.base},
                "punctuation": {"color": cfg.palette.ui.foreground_muted},
                "punctuation.bracket": {"color": cfg.palette.ui.foreground_muted},
                "punctuation.delimiter": {"color": cfg.palette.ui.foreground_muted},
                "punctuation.special": {"color": cfg.palette.base.ansi.cyan.base},
                "tag": {"color": cfg.palette.base.ansi.red.base},
                "embedded": {"color": ui.foreground},
                "link_text": {"color": cfg.palette.base.ansi.blue.base},
                "link_uri": {"color": cfg.palette.base.ansi.cyan.base},
                "title": {"color": cfg.palette.base.ansi.blue.base, "font_weight": 700},
                "emphasis": {"font_style": "italic"},
                "emphasis.strong": {"font_weight": 700},
                "text.literal": {"color": cfg.palette.syntax.teal},
                "boolean": {"color": cfg.palette.syntax.lavender},
                "primary": {"color": cfg.palette.base.ansi.blue.base},
                "predictive": {"color": cfg.palette.ui.foreground_dim, "font_style": "italic"}
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
        let variant_suffix = if v.name == "base" { String::new() } else { format!(" ({})", capitalize(&v.name)) };
        
        // Create sidebar/panel backgrounds with variant-aware alpha
        let sidebar_bg = if v.alpha.is_some() { ui.background_alt.clone() } else { cfg.palette.ui.background_alt.clone() };
        let elevated_bg = if v.alpha.is_some() { ui.background_elevated.clone() } else { cfg.palette.ui.background_elevated.clone() };
        
        let theme = json!({
            "name": format!("{}{}", cfg.meta.name, variant_suffix),
            "type": "dark",
            "colors": {
                // Main editor
                "editor.background": ui.background,
                "editor.foreground": ui.foreground,
                "editor.lineHighlightBackground": ui.line_highlight,
                "editor.selectionBackground": ui.selection,
                "editorCursor.foreground": ui.cursor,
                "editorLineNumber.foreground": cfg.palette.ui.foreground_dim,
                "editorLineNumber.activeForeground": cfg.palette.ui.foreground_muted,
                "editorIndentGuide.background": cfg.palette.border.border_variant,
                "editorIndentGuide.activeBackground": cfg.palette.border.border,
                "editorWidget.background": elevated_bg,
                "editorWidget.border": cfg.palette.border.border,
                "editorHoverWidget.background": elevated_bg,
                "editorHoverWidget.border": cfg.palette.border.border,
                "editorSuggestWidget.background": elevated_bg,
                "editorSuggestWidget.border": cfg.palette.border.border,
                "editorSuggestWidget.selectedBackground": ui.selection,
                "editorGroup.border": cfg.palette.border.border_variant,
                "editorGroupHeader.tabsBackground": sidebar_bg,
                "editorGroupHeader.noTabsBackground": sidebar_bg,
                
                // Activity bar (left icon bar)
                "activityBar.background": sidebar_bg,
                "activityBar.foreground": ui.foreground,
                "activityBar.inactiveForeground": cfg.palette.ui.foreground_muted,
                "activityBar.border": cfg.palette.border.border_variant,
                "activityBarBadge.background": cfg.palette.base.ansi.blue.base,
                "activityBarBadge.foreground": "#ffffff",
                
                // Sidebar
                "sideBar.background": sidebar_bg,
                "sideBar.foreground": ui.foreground,
                "sideBar.border": cfg.palette.border.border_variant,
                "sideBarTitle.foreground": ui.foreground,
                "sideBarSectionHeader.background": sidebar_bg,
                "sideBarSectionHeader.foreground": ui.foreground,
                "sideBarSectionHeader.border": cfg.palette.border.border_variant,
                
                // Title bar
                "titleBar.activeBackground": sidebar_bg,
                "titleBar.activeForeground": ui.foreground,
                "titleBar.inactiveBackground": sidebar_bg,
                "titleBar.inactiveForeground": cfg.palette.ui.foreground_muted,
                "titleBar.border": cfg.palette.border.border_variant,
                
                // Tabs
                "tab.activeBackground": ui.background,
                "tab.activeForeground": ui.foreground,
                "tab.inactiveBackground": sidebar_bg,
                "tab.inactiveForeground": cfg.palette.ui.foreground_muted,
                "tab.border": cfg.palette.border.border_variant,
                "tab.activeBorder": cfg.palette.base.ansi.cyan.base,
                "tab.activeBorderTop": null,
                "tab.hoverBackground": ui.background,
                
                // Status bar
                "statusBar.background": sidebar_bg,
                "statusBar.foreground": ui.foreground,
                "statusBar.border": cfg.palette.border.border_variant,
                "statusBar.noFolderBackground": sidebar_bg,
                "statusBar.debuggingBackground": cfg.palette.base.ansi.red.dim,
                "statusBarItem.remoteBackground": cfg.palette.base.ansi.blue.base,
                "statusBarItem.remoteForeground": "#ffffff",
                "statusBarItem.hoverBackground": elevated_bg,
                
                // Panels (terminal, output, etc.)
                "panel.background": sidebar_bg,
                "panel.border": cfg.palette.border.border_variant,
                "panelTitle.activeBorder": cfg.palette.base.ansi.cyan.base,
                "panelTitle.activeForeground": ui.foreground,
                "panelTitle.inactiveForeground": cfg.palette.ui.foreground_muted,
                "panelInput.border": cfg.palette.border.border,
                
                // Lists
                "list.activeSelectionBackground": ui.selection,
                "list.activeSelectionForeground": ui.foreground,
                "list.inactiveSelectionBackground": cfg.palette.border.border_variant,
                "list.inactiveSelectionForeground": ui.foreground,
                "list.hoverBackground": cfg.palette.border.border_variant,
                "list.hoverForeground": ui.foreground,
                "list.focusBackground": ui.selection,
                "list.focusForeground": ui.foreground,
                "listFilterWidget.background": elevated_bg,
                "listFilterWidget.noMatchesOutline": cfg.palette.base.ansi.red.base,
                
                // Input fields
                "input.background": elevated_bg,
                "input.foreground": ui.foreground,
                "input.border": cfg.palette.border.border,
                "input.placeholderForeground": cfg.palette.ui.foreground_dim,
                "inputOption.activeBorder": cfg.palette.base.ansi.cyan.base,
                "inputOption.activeBackground": cfg.palette.border.border_variant,
                "inputValidation.errorBackground": cfg.palette.base.ansi.red.dim,
                "inputValidation.errorBorder": cfg.palette.base.ansi.red.base,
                
                // Dropdowns
                "dropdown.background": elevated_bg,
                "dropdown.foreground": ui.foreground,
                "dropdown.border": cfg.palette.border.border,
                "dropdown.listBackground": elevated_bg,
                
                // Buttons
                "button.background": cfg.palette.base.ansi.blue.base,
                "button.foreground": "#ffffff",
                "button.hoverBackground": cfg.palette.base.ansi.blue.bright,
                "button.secondaryBackground": cfg.palette.border.border,
                "button.secondaryForeground": ui.foreground,
                
                // Scrollbar
                "scrollbar.shadow": "#00000033",
                "scrollbarSlider.background": cfg.palette.border.border,
                "scrollbarSlider.hoverBackground": cfg.palette.border.border_variant,
                "scrollbarSlider.activeBackground": cfg.palette.ui.foreground_dim,
                
                // Notifications
                "notifications.background": elevated_bg,
                "notifications.foreground": ui.foreground,
                "notifications.border": cfg.palette.border.border,
                "notificationCenter.border": cfg.palette.border.border,
                "notificationCenterHeader.background": sidebar_bg,
                "notificationToast.border": cfg.palette.border.border,
                
                // Peek view
                "peekView.border": cfg.palette.border.border_focused,
                "peekViewEditor.background": ui.background,
                "peekViewResult.background": sidebar_bg,
                "peekViewTitle.background": elevated_bg,
                "peekViewTitleLabel.foreground": ui.foreground,
                
                // Diff editor
                "diffEditor.insertedTextBackground": "#a9cfa420",
                "diffEditor.removedTextBackground": "#bf616a20",
                "diffEditor.insertedLineBackground": "#a9cfa410",
                "diffEditor.removedLineBackground": "#bf616a10",
                
                // Git colors
                "gitDecoration.addedResourceForeground": cfg.palette.base.ansi.green.base,
                "gitDecoration.modifiedResourceForeground": cfg.palette.base.ansi.yellow.base,
                "gitDecoration.deletedResourceForeground": cfg.palette.base.ansi.red.base,
                "gitDecoration.untrackedResourceForeground": cfg.palette.base.ansi.green.dim,
                "gitDecoration.ignoredResourceForeground": cfg.palette.ui.foreground_dim,
                "gitDecoration.conflictingResourceForeground": cfg.palette.base.ansi.magenta.base,
                
                // Breadcrumbs
                "breadcrumb.background": sidebar_bg,
                "breadcrumb.foreground": cfg.palette.ui.foreground_muted,
                "breadcrumb.focusForeground": ui.foreground,
                "breadcrumb.activeSelectionForeground": ui.foreground,
                "breadcrumbPicker.background": elevated_bg,
                
                // Command center / quick input
                "quickInput.background": elevated_bg,
                "quickInput.foreground": ui.foreground,
                "quickInputList.focusBackground": ui.selection,
                "quickInputTitle.background": sidebar_bg,
                
                // Minimap
                "minimap.background": sidebar_bg,
                "minimap.selectionHighlight": ui.selection,
                "minimapSlider.background": cfg.palette.border.border,
                "minimapSlider.hoverBackground": cfg.palette.border.border_variant,
                "minimapSlider.activeBackground": cfg.palette.ui.foreground_dim,
                
                // Focus/selection borders
                "focusBorder": cfg.palette.border.border_focused,
                "selection.background": ui.selection,
                "foreground": ui.foreground,
                "descriptionForeground": cfg.palette.ui.foreground_muted,
                "errorForeground": cfg.palette.base.ansi.red.base,
                "widget.shadow": "#00000033",
                
                // Borders
                "contrastBorder": null,
                "contrastActiveBorder": null,
                
                // Terminal
                "terminal.background": sidebar_bg,
                "terminal.foreground": ui.foreground,
                "terminal.border": cfg.palette.border.border_variant,
                "terminal.selectionBackground": ui.selection,
                "terminalCursor.foreground": ui.cursor,
                "terminalCursor.background": ui.background,
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
                {"scope": ["comment", "punctuation.definition.comment"], "settings": {"foreground": cfg.palette.syntax.gray, "fontStyle": "italic"}},
                {"scope": ["variable", "variable.other"], "settings": {"foreground": ui.foreground}},
                {"scope": ["entity.name.type", "entity.name.class", "support.type", "support.class"], "settings": {"foreground": cfg.palette.base.ansi.yellow.base}},
                {"scope": ["entity.name.tag"], "settings": {"foreground": cfg.palette.base.ansi.red.base}},
                {"scope": ["entity.other.attribute-name"], "settings": {"foreground": cfg.palette.base.ansi.magenta.base, "fontStyle": "italic"}},
                {"scope": ["punctuation"], "settings": {"foreground": cfg.palette.ui.foreground_muted}},
                {"scope": ["meta.embedded", "source.groovy.embedded"], "settings": {"foreground": ui.foreground}}
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
