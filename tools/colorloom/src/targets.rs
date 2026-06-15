use crate::config::{Config, Variant};
use anyhow::{anyhow, Result};
use serde_json::{json, Map, Value};
use std::{fs, path::Path};

fn strip_alpha(hex: &str) -> String {
    let h = hex.trim_start_matches('#');
    if h.len() >= 6 {
        format!("#{}", &h[0..6])
    } else {
        format!("#{}", h)
    }
}

fn apply_alpha(hex: &str, a: f32) -> String {
    let a = a.clamp(0.0, 1.0);
    let alpha = (a * 255.0).round() as u8;
    let h = hex.trim_start_matches('#');
    let base = if h.len() >= 6 { &h[0..6] } else { h };
    format!("#{}{:02X}", base, alpha)
}

fn shaded(hex: &str, alpha: f32) -> String {
    let base = strip_alpha(hex);
    apply_alpha(&base, alpha)
}

fn player_entry(color: &str) -> Value {
    let selection = shaded(color, 0.3);
    json!({
        "cursor": color,
        "background": color,
        "selection": selection
    })
}

#[derive(Clone)]
struct BlurLayers {
    surface: String,
    elevated: String,
    panel: String,
    overlay: String,
    toolbar: String,
    soft: String,
    status: String,
    terminal: String,
    inactive_title: String,
}

fn syntax_entry(color: &str, font_style: Option<&str>, font_weight: Option<u32>) -> Value {
    let mut map = Map::new();
    map.insert("color".into(), Value::String(color.to_string()));
    if let Some(style) = font_style {
        map.insert("font_style".into(), Value::String(style.to_string()));
    }
    if let Some(weight) = font_weight {
        map.insert("font_weight".into(), json!(weight));
    }
    Value::Object(map)
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
                ui.background = if let Some(alpha) = variant.alpha {
                    if v.len() <= 7 {
                        apply_alpha(v, alpha)
                    } else {
                        v.clone()
                    }
                } else {
                    v.clone()
                };
            }
            if let Some(v) = &uo.background_alt {
                ui.background_alt = if let Some(alpha) = variant.alpha {
                    if v.len() <= 7 {
                        apply_alpha(v, alpha)
                    } else {
                        v.clone()
                    }
                } else {
                    v.clone()
                };
            }
            if let Some(v) = &uo.background_elevated {
                ui.background_elevated = if let Some(alpha) = variant.alpha {
                    if v.len() <= 7 {
                        apply_alpha(v, alpha)
                    } else {
                        v.clone()
                    }
                } else {
                    v.clone()
                };
            }
            if let Some(v) = &uo.selection {
                ui.selection = v.clone();
            }
            if let Some(v) = &uo.cursor {
                ui.cursor = v.clone();
            }
            if let Some(v) = &uo.line_highlight {
                ui.line_highlight = v.clone();
            }
            if let Some(v) = &uo.foreground {
                ui.foreground = v.clone();
            }
            if let Some(v) = &uo.foreground_muted {
                ui.foreground_muted = v.clone();
            }
            if let Some(v) = &uo.foreground_dim {
                ui.foreground_dim = v.clone();
            }
        }
    }
    ui
}

pub fn generate_target(cfg: &Config, target: &crate::config::Target, root: &Path) -> Result<()> {
    match target.id.as_str() {
        "ghostty" => gen_ghostty(cfg, target, root),
        "zed" => gen_zed(cfg, target, root),
        "cursor" => gen_cursor(cfg, target, root),
        "neovim" => gen_neovim(cfg, target, root),
        "website" => gen_website(cfg, target, root),
        other => Err(anyhow!("Unknown target id: {}", other)),
    }
}

fn gen_ghostty(cfg: &Config, target: &crate::config::Target, root: &Path) -> Result<()> {
    let dir = root.join(&target.path);
    fs::create_dir_all(&dir)?;
    for v in &cfg.variants {
        if target
            .out_names
            .as_ref()
            .is_some_and(|m| !m.contains_key(&v.name))
        {
            continue;
        }
        let ui = ui_with_variant(cfg, v);
        let name = target
            .out_names
            .as_ref()
            .and_then(|m| m.get(&v.name))
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "{}-{}",
                    cfg.meta.name.to_lowercase().replace(' ', "-"),
                    v.name
                )
            });
        let mut out = String::new();
        out.push_str(&format!("# {} for Ghostty\n", cfg.meta.name));
        out.push_str("palette = 0=#7f7f7f\n");
        out.push_str(&format!("palette = 1={}\n", cfg.palette.base.ansi.red.base));
        out.push_str(&format!(
            "palette = 2={}\n",
            cfg.palette.base.ansi.green.base
        ));
        out.push_str(&format!(
            "palette = 3={}\n",
            cfg.palette.base.ansi.yellow.base
        ));
        out.push_str(&format!(
            "palette = 4={}\n",
            cfg.palette.base.ansi.blue.base
        ));
        out.push_str(&format!(
            "palette = 5={}\n",
            cfg.palette.base.ansi.magenta.base
        ));
        out.push_str(&format!(
            "palette = 6={}\n",
            cfg.palette.base.ansi.cyan.base
        ));
        out.push_str("palette = 7=#d4d4d4\n");
        out.push_str("palette = 8=#7f7f7f\n");
        out.push_str(&format!(
            "palette = 9={}\n",
            cfg.palette.base.ansi.red.bright
        ));
        out.push_str(&format!(
            "palette = 10={}\n",
            cfg.palette.base.ansi.green.bright
        ));
        out.push_str(&format!(
            "palette = 11={}\n",
            cfg.palette.base.ansi.yellow.bright
        ));
        out.push_str(&format!(
            "palette = 12={}\n",
            cfg.palette.base.ansi.blue.bright
        ));
        out.push_str(&format!(
            "palette = 13={}\n",
            cfg.palette.base.ansi.magenta.bright
        ));
        out.push_str(&format!(
            "palette = 14={}\n",
            cfg.palette.base.ansi.cyan.bright
        ));
        out.push_str("palette = 15=#ffffff\n");
        out.push_str(&format!("background = {}\n", strip_alpha(&ui.background)));
        out.push_str(&format!("foreground = {}\n", strip_alpha(&ui.foreground)));
        out.push_str(&format!("cursor-color = {}\n", strip_alpha(&ui.cursor)));
        out.push_str("cursor-text = #ffffff\n");
        out.push_str(&format!(
            "selection-background = {}\n",
            strip_alpha(&ui.selection)
        ));
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

fn gen_zed(cfg: &Config, target: &crate::config::Target, root: &Path) -> Result<()> {
    let dir = root.join(&target.path);
    fs::create_dir_all(&dir)?;
    let mut themes = vec![];
    for v in &cfg.variants {
        let ui = ui_with_variant(cfg, v);
        let title = match v.name.as_str() {
            "base" => cfg.meta.name.clone(),
            other => format!("{} {}", cfg.meta.name, capitalize(other)),
        };
        let style = build_zed_style(cfg, v, &ui);
        themes.push(json!({
            "name": title,
            "appearance": "dark",
            "style": style
        }));
    }
    let root_obj = json!({
        "$schema": "https://zed.dev/schema/themes/v0.2.0.json",
        "name": cfg.meta.name.clone(),
        "author": cfg.meta.author.clone().unwrap_or_default(),
        "themes": themes
    });
    let file = target
        .out_file
        .clone()
        .unwrap_or_else(|| format!("{}.json", cfg.meta.name.to_lowercase().replace(' ', "-")));
    fs::write(dir.join(file), serde_json::to_string_pretty(&root_obj)?)?;
    Ok(())
}

fn build_zed_style(cfg: &Config, variant: &Variant, ui: &crate::config::UiPalette) -> Value {
    let alpha = variant.alpha.unwrap_or(1.0);
    let appearance = variant.appearance.clone().unwrap_or_else(|| {
        if alpha < 1.0 {
            "blurred".into()
        } else {
            "opaque".into()
        }
    });
    let uses_tiers = alpha < 1.0 || appearance == "transparent";
    let tint_alpha = match variant.name.as_str() {
        "cloudy" => 0xD9 as f32 / 255.0,
        "hazy" => 0xB3 as f32 / 255.0,
        _ => alpha,
    };
    let background_alpha = if uses_tiers {
        tint_alpha.clamp(0.65, 0.95)
    } else {
        1.0
    };
    let surface_alpha = if uses_tiers {
        (tint_alpha * 0.9).clamp(0.5, 0.95)
    } else {
        1.0
    };
    let elevated_alpha = if uses_tiers {
        (tint_alpha * 1.05).clamp(0.6, 1.0)
    } else {
        1.0
    };
    let panel_alpha = if uses_tiers {
        (tint_alpha * 0.85).clamp(0.45, 0.92)
    } else {
        1.0
    };
    let overlay_alpha = if uses_tiers {
        (tint_alpha * 0.95).clamp(0.55, 1.0)
    } else {
        1.0
    };
    let subtle_alpha = if uses_tiers {
        (tint_alpha * 0.6).clamp(0.3, 0.85)
    } else {
        0.65
    };
    let ghost_alpha = if uses_tiers {
        (tint_alpha * 0.35).clamp(0.2, 0.65)
    } else {
        0.35
    };
    let tab_inactive_alpha = if uses_tiers {
        (tint_alpha * 0.35).clamp(0.2, 0.7)
    } else {
        0.5
    };
    let track_alpha = if uses_tiers {
        (tint_alpha * 0.25).clamp(0.15, 0.45)
    } else {
        0.3
    };
    let status_alpha = if uses_tiers { 0.18 } else { 0.24 };
    let accent_alpha = if tint_alpha <= 0.78 {
        0.45
    } else if tint_alpha < 0.95 {
        0.6
    } else {
        0.72
    };

    let base_bg = strip_alpha(&ui.background);
    let alt_bg = strip_alpha(&ui.background_alt);
    let elevated_bg = strip_alpha(&ui.background_elevated);
    let selection = strip_alpha(&ui.selection);
    let highlight = strip_alpha(&ui.line_highlight);
    let border_base = strip_alpha(&cfg.palette.border.border);
    let border_variant = strip_alpha(&cfg.palette.border.border_variant);
    let border_selected = strip_alpha(&cfg.palette.border.border_selected);

    let blur_layers = if uses_tiers {
        let tint = |mult: f32, min_v: f32, max_v: f32| -> String {
            apply_alpha(&base_bg, (tint_alpha * mult).clamp(min_v, max_v))
        };
        Some(BlurLayers {
            surface: tint(1.05, 0.55, 0.98),
            elevated: tint(1.25, 0.6, 1.0),
            panel: tint(0.92, 0.45, 0.95),
            overlay: tint(1.35, 0.7, 1.0),
            toolbar: tint(0.65, 0.35, 0.88),
            soft: tint(0.5, 0.25, 0.8),
            status: tint(1.05, 0.55, 0.98),
            terminal: tint(0.9, 0.45, 0.95),
            inactive_title: tint(0.6, 0.35, 0.85),
        })
    } else {
        None
    };

    let mut background = apply_alpha(&base_bg, background_alpha);
    let surface = blur_layers
        .as_ref()
        .map(|layers| layers.surface.clone())
        .unwrap_or_else(|| apply_alpha(&alt_bg, surface_alpha));
    let mut elevated_surface = blur_layers
        .as_ref()
        .map(|layers| layers.elevated.clone())
        .unwrap_or_else(|| apply_alpha(&elevated_bg, elevated_alpha));
    let mut panel_background = blur_layers
        .as_ref()
        .map(|layers| layers.panel.clone())
        .unwrap_or_else(|| apply_alpha(&alt_bg, panel_alpha));
    let overlay_background = blur_layers
        .as_ref()
        .map(|layers| layers.overlay.clone())
        .unwrap_or_else(|| apply_alpha(&elevated_bg, overlay_alpha));
    let mut toolbar_background = blur_layers
        .as_ref()
        .map(|layers| layers.toolbar.clone())
        .unwrap_or_else(|| apply_alpha(&alt_bg, subtle_alpha));
    let mut tab_bar_background = blur_layers
        .as_ref()
        .map(|layers| layers.toolbar.clone())
        .unwrap_or_else(|| apply_alpha(&alt_bg, (tab_inactive_alpha * 0.75).clamp(0.2, 0.85)));
    let mut tab_inactive_background = blur_layers
        .as_ref()
        .map(|layers| layers.soft.clone())
        .unwrap_or_else(|| apply_alpha(&alt_bg, tab_inactive_alpha));
    let mut tab_active_background = blur_layers
        .as_ref()
        .map(|layers| layers.elevated.clone())
        .unwrap_or_else(|| apply_alpha(&elevated_bg, elevated_alpha));
    let scrollbar_track = apply_alpha(&alt_bg, track_alpha);
    let mut element_background = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.78).clamp(0.45, 0.98))
    } else {
        apply_alpha(&alt_bg, (subtle_alpha + 0.1).min(1.0))
    };
    let element_hover = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.95).clamp(0.55, 1.0))
    } else {
        apply_alpha(&elevated_bg, (subtle_alpha + 0.2).min(1.0))
    };
    let element_active = apply_alpha(&selection, 0.85);
    let element_selected = apply_alpha(&selection, 0.8);
    let element_disabled = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.35).clamp(0.2, 0.65))
    } else {
        apply_alpha(&alt_bg, (subtle_alpha * 0.6).max(0.2))
    };
    let mut ghost_background = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.4).clamp(0.2, 0.75))
    } else {
        apply_alpha(&alt_bg, ghost_alpha)
    };
    let ghost_hover = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.5).clamp(0.25, 0.85))
    } else {
        apply_alpha(&alt_bg, (ghost_alpha + 0.15).min(0.75))
    };
    let ghost_active = apply_alpha(&selection, 0.6);
    let ghost_selected = apply_alpha(&selection, 0.7);
    let ghost_disabled = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.22).clamp(0.15, 0.55))
    } else {
        apply_alpha(&alt_bg, (ghost_alpha * 0.5).max(0.15))
    };
    let drop_target = shaded(&cfg.palette.base.ansi.blue.base, 0.35);
    let panel_indent = apply_alpha(&border_variant, 0.65);
    let pane_group_border = apply_alpha(&border_variant, if uses_tiers { 0.72 } else { 0.45 });
    let scrollbar_thumb = apply_alpha(&border_base, 0.7);
    let minimap_thumb = apply_alpha(&border_base, 0.35);
    let minimap_thumb_hover = shaded(&cfg.palette.base.ansi.magenta.base, 0.45);
    let minimap_thumb_active = shaded(&cfg.palette.base.ansi.magenta.base, 0.65);
    let mut editor_background = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.45).clamp(0.3, 0.8))
    } else {
        apply_alpha(&base_bg, 1.0)
    };
    let mut editor_gutter = editor_background.clone();
    let mut editor_active_line = apply_alpha(&highlight, (tint_alpha * 0.8).clamp(0.35, 1.0));
    let inline_highlight = shaded(&cfg.palette.base.ansi.blue.base, 0.12);
    let doc_highlight_bracket = shaded(&cfg.palette.base.ansi.magenta.base, 0.15);
    let doc_highlight_read = shaded(&cfg.palette.base.ansi.blue.base, 0.2);
    let doc_highlight_write = shaded(&cfg.palette.base.ansi.cyan.base, 0.25);
    let editor_selection = apply_alpha(&selection, 0.9);
    let search_background = shaded(&cfg.palette.base.ansi.yellow.base, 0.2);
    let hidden_background = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.35).clamp(0.2, 0.65))
    } else {
        apply_alpha(&alt_bg, (subtle_alpha * 0.8).max(0.25))
    };
    let ignored_background = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.28).clamp(0.2, 0.6))
    } else {
        apply_alpha(&alt_bg, (subtle_alpha * 0.6).max(0.2))
    };
    let hint_background = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.55).clamp(0.3, 0.85))
    } else {
        apply_alpha(&elevated_bg, (subtle_alpha * 0.9).max(0.35))
    };
    let predictive_background = if uses_tiers {
        apply_alpha(&base_bg, (tint_alpha * 0.5).clamp(0.3, 0.85))
    } else {
        apply_alpha(&elevated_bg, 0.45)
    };
    let renamed_background = shaded(&cfg.palette.base.ansi.blue.base, status_alpha);
    let info_background = shaded(&cfg.palette.base.ansi.blue.base, status_alpha);
    let success_background = shaded(&cfg.palette.base.ansi.green.base, status_alpha);
    let warning_background = shaded(&cfg.palette.base.ansi.yellow.base, status_alpha);
    let error_background = shaded(&cfg.palette.base.ansi.red.base, status_alpha);
    let unreachable_background = shaded(&cfg.palette.base.ansi.red.base, status_alpha);
    let created_background = shaded(&cfg.palette.base.ansi.green.base, status_alpha);
    let modified_background = shaded(&cfg.palette.base.ansi.yellow.base, status_alpha);
    let deleted_background = shaded(&cfg.palette.base.ansi.red.base, status_alpha);
    let conflict_background = shaded(&cfg.palette.base.ansi.magenta.base, status_alpha);
    let status_bar_background = blur_layers
        .as_ref()
        .map(|layers| layers.status.clone())
        .unwrap_or_else(|| surface.clone());
    let title_bar_background = blur_layers
        .as_ref()
        .map(|layers| layers.status.clone())
        .unwrap_or_else(|| surface.clone());
    let title_bar_inactive = blur_layers
        .as_ref()
        .map(|layers| layers.inactive_title.clone())
        .unwrap_or_else(|| apply_alpha(&alt_bg, (surface_alpha * 0.9).clamp(0.4, 0.95)));
    let mut terminal_background = blur_layers
        .as_ref()
        .map(|layers| layers.terminal.clone())
        .unwrap_or_else(|| panel_background.clone());
    let mut terminal_ansi_background = blur_layers
        .as_ref()
        .map(|layers| layers.terminal.clone())
        .unwrap_or_else(|| panel_background.clone());
    let debugger_active_line = shaded(&cfg.palette.base.ansi.yellow.base, 0.15);

    let variant_name = variant.name.as_str();
    if matches!(variant_name, "cloudy" | "hazy" | "clear") {
        let base_override = match variant_name {
            "cloudy" => Some(0xD9 as f32 / 255.0),
            "hazy" => Some(0xB3 as f32 / 255.0),
            _ => None,
        };
        if let Some(alpha_override) = base_override {
            background = apply_alpha(&base_bg, alpha_override);
        }
        let gutter_alpha = 0.0;
        let tab_bar_alpha = 0x11 as f32 / 255.0;
        let tab_inactive_alpha_override = 0x22 as f32 / 255.0;
        let tab_active_override = 0x55 as f32 / 255.0;
        let terminal_alpha = if variant_name == "clear" {
            0.0
        } else {
            0x22 as f32 / 255.0
        };
        let toolbar_alpha = 0x1A as f32 / 255.0;
        editor_background = apply_alpha(&base_bg, 0.0);
        editor_gutter = apply_alpha(&base_bg, gutter_alpha);
        tab_bar_background = apply_alpha(&alt_bg, tab_bar_alpha);
        tab_inactive_background = apply_alpha(&alt_bg, tab_inactive_alpha_override);
        tab_active_background = apply_alpha(&elevated_bg, tab_active_override);
        editor_active_line = apply_alpha(&highlight, 0xDD as f32 / 255.0);
        panel_background = apply_alpha(&base_bg, 0.0);
        toolbar_background = apply_alpha(&base_bg, toolbar_alpha);
        terminal_background = apply_alpha(&base_bg, terminal_alpha);
        terminal_ansi_background = terminal_background.clone();
        let element_alpha = 0x22 as f32 / 255.0;
        let elevated_alpha = 0xE6 as f32 / 255.0;
        let ghost_alpha_override = 0x11 as f32 / 255.0;
        element_background = apply_alpha(&base_bg, element_alpha);
        elevated_surface = apply_alpha(&base_bg, elevated_alpha);
        ghost_background = apply_alpha(&base_bg, ghost_alpha_override);
    }

    let accent_sources = vec![
        cfg.palette.base.ansi.blue.base.as_str(),
        cfg.palette.base.ansi.cyan.base.as_str(),
        cfg.palette.base.ansi.magenta.base.as_str(),
        cfg.palette.base.ansi.green.base.as_str(),
        cfg.palette.base.ansi.yellow.base.as_str(),
        cfg.palette.base.ansi.red.base.as_str(),
        cfg.palette.syntax.lavender.as_str(),
    ];
    let accents: Vec<String> = accent_sources
        .into_iter()
        .map(|hex| shaded(hex, accent_alpha))
        .collect();

    let player_colors = vec![
        cfg.palette.base.ansi.cyan.base.as_str(),
        cfg.palette.base.ansi.magenta.base.as_str(),
        cfg.palette.base.ansi.green.base.as_str(),
        cfg.palette.base.ansi.yellow.base.as_str(),
        cfg.palette.base.ansi.blue.base.as_str(),
        cfg.palette.base.ansi.red.base.as_str(),
        cfg.palette.syntax.lavender.as_str(),
        cfg.palette.syntax.teal.as_str(),
    ];
    let players: Vec<Value> = player_colors.into_iter().map(player_entry).collect();

    let mut style = Map::new();
    style.insert("accents".into(), json!(accents));
    style.insert(
        "background.appearance".into(),
        Value::String(appearance.clone()),
    );
    style.insert("background".into(), Value::String(background.clone()));
    style.insert("surface.background".into(), Value::String(surface.clone()));
    style.insert(
        "elevated_surface.background".into(),
        Value::String(elevated_surface.clone()),
    );
    style.insert(
        "status_bar.background".into(),
        Value::String(status_bar_background),
    );
    style.insert(
        "title_bar.background".into(),
        Value::String(title_bar_background),
    );
    style.insert(
        "title_bar.inactive_background".into(),
        Value::String(title_bar_inactive),
    );
    style.insert(
        "toolbar.background".into(),
        Value::String(toolbar_background.clone()),
    );
    style.insert(
        "tab_bar.background".into(),
        Value::String(tab_bar_background),
    );
    style.insert(
        "tab.inactive_background".into(),
        Value::String(tab_inactive_background),
    );
    style.insert(
        "tab.active_background".into(),
        Value::String(tab_active_background),
    );
    style.insert(
        "panel.background".into(),
        Value::String(panel_background.clone()),
    );
    style.insert(
        "panel.focused_border".into(),
        Value::String(cfg.palette.border.border_focused.clone()),
    );
    style.insert(
        "panel.overlay_background".into(),
        Value::String(overlay_background.clone()),
    );
    style.insert(
        "panel.indent_guide".into(),
        Value::String(panel_indent.clone()),
    );
    style.insert(
        "panel.indent_guide_active".into(),
        Value::String(cfg.palette.border.border.clone()),
    );
    style.insert(
        "panel.indent_guide_hover".into(),
        Value::String(cfg.palette.base.ansi.cyan.base.clone()),
    );
    style.insert(
        "pane.focused_border".into(),
        Value::String(cfg.palette.border.border_focused.clone()),
    );
    style.insert(
        "pane_group.border".into(),
        Value::String(pane_group_border.clone()),
    );
    style.insert("drop_target.background".into(), Value::String(drop_target));
    style.insert(
        "ghost_element.background".into(),
        Value::String(ghost_background),
    );
    style.insert("ghost_element.hover".into(), Value::String(ghost_hover));
    style.insert("ghost_element.active".into(), Value::String(ghost_active));
    style.insert(
        "ghost_element.selected".into(),
        Value::String(ghost_selected),
    );
    style.insert(
        "ghost_element.disabled".into(),
        Value::String(ghost_disabled),
    );
    style.insert(
        "element.background".into(),
        Value::String(element_background),
    );
    style.insert("element.hover".into(), Value::String(element_hover));
    style.insert("element.active".into(), Value::String(element_active));
    style.insert("element.selected".into(), Value::String(element_selected));
    style.insert("element.disabled".into(), Value::String(element_disabled));
    style.insert("text".into(), Value::String(ui.foreground.clone()));
    style.insert(
        "text.muted".into(),
        Value::String(cfg.palette.ui.foreground_muted.clone()),
    );
    style.insert(
        "text.placeholder".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "text.disabled".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "text.accent".into(),
        Value::String(cfg.palette.base.ansi.cyan.base.clone()),
    );
    style.insert(
        "icon".into(),
        Value::String(cfg.palette.ui.foreground_muted.clone()),
    );
    style.insert(
        "icon.muted".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "icon.disabled".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "icon.placeholder".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "icon.accent".into(),
        Value::String(cfg.palette.base.ansi.cyan.base.clone()),
    );
    style.insert(
        "search.match_background".into(),
        Value::String(search_background),
    );
    style.insert(
        "link_text.hover".into(),
        Value::String(cfg.palette.base.ansi.blue.bright.clone()),
    );
    style.insert(
        "scrollbar.thumb.background".into(),
        Value::String(scrollbar_thumb.clone()),
    );
    style.insert(
        "scrollbar.thumb.hover_background".into(),
        Value::String(cfg.palette.border.border.clone()),
    );
    style.insert(
        "scrollbar.thumb.active_background".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert("scrollbar.thumb.border".into(), Value::Null);
    style.insert(
        "scrollbar.track.background".into(),
        Value::String(scrollbar_track),
    );
    style.insert("scrollbar.track.border".into(), Value::Null);
    style.insert(
        "minimap.thumb.background".into(),
        Value::String(minimap_thumb),
    );
    style.insert(
        "minimap.thumb.hover_background".into(),
        Value::String(minimap_thumb_hover),
    );
    style.insert(
        "minimap.thumb.active_background".into(),
        Value::String(minimap_thumb_active),
    );
    style.insert("minimap.thumb.border".into(), Value::Null);
    style.insert("editor.background".into(), Value::String(editor_background));
    style.insert(
        "editor.gutter.background".into(),
        Value::String(editor_gutter),
    );
    style.insert(
        "editor.foreground".into(),
        Value::String(ui.foreground.clone()),
    );
    style.insert(
        "editor.subheader.background".into(),
        Value::String(surface.clone()),
    );
    style.insert(
        "editor.active_line.background".into(),
        Value::String(editor_active_line),
    );
    style.insert(
        "editor.highlighted_line.background".into(),
        Value::String(inline_highlight),
    );
    style.insert(
        "editor.line_number".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "editor.active_line_number".into(),
        Value::String(ui.foreground.clone()),
    );
    style.insert(
        "editor.invisible".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "editor.wrap_guide".into(),
        Value::String(cfg.palette.border.border_variant.clone()),
    );
    style.insert(
        "editor.indent_guide".into(),
        Value::String(cfg.palette.border.border_variant.clone()),
    );
    style.insert(
        "editor.indent_guide_active".into(),
        Value::String(cfg.palette.border.border.clone()),
    );
    style.insert(
        "editor.active_wrap_guide".into(),
        Value::String(cfg.palette.border.border.clone()),
    );
    style.insert(
        "editor.document_highlight.bracket_background".into(),
        Value::String(doc_highlight_bracket),
    );
    style.insert(
        "editor.document_highlight.read_background".into(),
        Value::String(doc_highlight_read),
    );
    style.insert(
        "editor.document_highlight.write_background".into(),
        Value::String(doc_highlight_write),
    );
    style.insert(
        "editor.selection.background".into(),
        Value::String(editor_selection),
    );
    style.insert(
        "editor.selection.foreground".into(),
        Value::String("#ffffff".into()),
    );
    style.insert(
        "editor.debugger_active_line.background".into(),
        Value::String(debugger_active_line),
    );
    style.insert(
        "terminal.background".into(),
        Value::String(terminal_background.clone()),
    );
    style.insert(
        "terminal.ansi.background".into(),
        Value::String(terminal_ansi_background),
    );
    style.insert(
        "terminal.foreground".into(),
        Value::String(ui.foreground.clone()),
    );
    style.insert(
        "terminal.dim_foreground".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "terminal.bright_foreground".into(),
        Value::String("#ffffff".into()),
    );
    style.insert(
        "terminal.ansi.black".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "terminal.ansi.white".into(),
        Value::String("#d4d4d4".into()),
    );
    style.insert(
        "terminal.ansi.red".into(),
        Value::String(cfg.palette.base.ansi.red.base.clone()),
    );
    style.insert(
        "terminal.ansi.green".into(),
        Value::String(cfg.palette.base.ansi.green.base.clone()),
    );
    style.insert(
        "terminal.ansi.yellow".into(),
        Value::String(cfg.palette.base.ansi.yellow.base.clone()),
    );
    style.insert(
        "terminal.ansi.blue".into(),
        Value::String(cfg.palette.base.ansi.blue.base.clone()),
    );
    style.insert(
        "terminal.ansi.magenta".into(),
        Value::String(cfg.palette.base.ansi.magenta.base.clone()),
    );
    style.insert(
        "terminal.ansi.cyan".into(),
        Value::String(cfg.palette.base.ansi.cyan.base.clone()),
    );
    style.insert(
        "terminal.ansi.bright_black".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "terminal.ansi.bright_red".into(),
        Value::String(cfg.palette.base.ansi.red.bright.clone()),
    );
    style.insert(
        "terminal.ansi.bright_green".into(),
        Value::String(cfg.palette.base.ansi.green.bright.clone()),
    );
    style.insert(
        "terminal.ansi.bright_yellow".into(),
        Value::String(cfg.palette.base.ansi.yellow.bright.clone()),
    );
    style.insert(
        "terminal.ansi.bright_blue".into(),
        Value::String(cfg.palette.base.ansi.blue.bright.clone()),
    );
    style.insert(
        "terminal.ansi.bright_magenta".into(),
        Value::String(cfg.palette.base.ansi.magenta.bright.clone()),
    );
    style.insert(
        "terminal.ansi.bright_cyan".into(),
        Value::String(cfg.palette.base.ansi.cyan.bright.clone()),
    );
    style.insert(
        "terminal.ansi.bright_white".into(),
        Value::String("#ffffff".into()),
    );
    style.insert(
        "terminal.ansi.dim_black".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "terminal.ansi.dim_red".into(),
        Value::String(cfg.palette.base.ansi.red.dim.clone()),
    );
    style.insert(
        "terminal.ansi.dim_green".into(),
        Value::String(cfg.palette.base.ansi.green.dim.clone()),
    );
    style.insert(
        "terminal.ansi.dim_yellow".into(),
        Value::String(cfg.palette.base.ansi.yellow.dim.clone()),
    );
    style.insert(
        "terminal.ansi.dim_blue".into(),
        Value::String(cfg.palette.base.ansi.blue.dim.clone()),
    );
    style.insert(
        "terminal.ansi.dim_magenta".into(),
        Value::String(cfg.palette.base.ansi.magenta.dim.clone()),
    );
    style.insert(
        "terminal.ansi.dim_cyan".into(),
        Value::String(cfg.palette.base.ansi.cyan.dim.clone()),
    );
    style.insert(
        "terminal.ansi.dim_white".into(),
        Value::String(cfg.palette.ui.foreground_muted.clone()),
    );
    style.insert(
        "version_control.added".into(),
        Value::String(cfg.palette.base.ansi.green.base.clone()),
    );
    style.insert(
        "version_control.deleted".into(),
        Value::String(cfg.palette.base.ansi.red.base.clone()),
    );
    style.insert(
        "version_control.modified".into(),
        Value::String(cfg.palette.base.ansi.yellow.base.clone()),
    );
    style.insert(
        "version_control.renamed".into(),
        Value::String(cfg.palette.base.ansi.blue.base.clone()),
    );
    style.insert(
        "version_control.conflict".into(),
        Value::String(cfg.palette.base.ansi.magenta.base.clone()),
    );
    style.insert(
        "version_control.ignored".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "version_control.conflict_marker.ours".into(),
        Value::String(shaded(&cfg.palette.base.ansi.green.base, 0.25)),
    );
    style.insert(
        "version_control.conflict_marker.theirs".into(),
        Value::String(shaded(&cfg.palette.base.ansi.blue.base, 0.25)),
    );
    style.insert("players".into(), Value::Array(players));

    let border_transparent = format!("{}00", base_bg);
    style.insert(
        "border".into(),
        Value::String(apply_alpha(
            &border_base,
            if uses_tiers {
                (tint_alpha * 0.75).clamp(0.55, 0.9)
            } else {
                0.8
            },
        )),
    );
    style.insert(
        "border.variant".into(),
        Value::String(cfg.palette.border.border_variant.clone()),
    );
    style.insert(
        "border.focused".into(),
        Value::String(cfg.palette.border.border_focused.clone()),
    );
    style.insert(
        "border.selected".into(),
        Value::String(apply_alpha(
            &border_selected,
            if uses_tiers {
                (tint_alpha * 0.85).clamp(0.5, 1.0)
            } else {
                1.0
            },
        )),
    );
    style.insert(
        "border.transparent".into(),
        Value::String(border_transparent),
    );
    style.insert(
        "border.disabled".into(),
        Value::String(cfg.palette.border.border_variant.clone()),
    );

    let status_defs = vec![
        (
            "conflict",
            cfg.palette.base.ansi.magenta.base.as_str(),
            cfg.palette.base.ansi.magenta.dim.as_str(),
            conflict_background,
        ),
        (
            "created",
            cfg.palette.base.ansi.green.base.as_str(),
            cfg.palette.base.ansi.green.dim.as_str(),
            created_background,
        ),
        (
            "deleted",
            cfg.palette.base.ansi.red.base.as_str(),
            cfg.palette.base.ansi.red.dim.as_str(),
            deleted_background,
        ),
        (
            "modified",
            cfg.palette.base.ansi.yellow.base.as_str(),
            cfg.palette.base.ansi.yellow.dim.as_str(),
            modified_background,
        ),
        (
            "renamed",
            cfg.palette.base.ansi.blue.base.as_str(),
            cfg.palette.base.ansi.blue.dim.as_str(),
            renamed_background,
        ),
        (
            "info",
            cfg.palette.base.ansi.blue.base.as_str(),
            cfg.palette.base.ansi.blue.dim.as_str(),
            info_background,
        ),
        (
            "warning",
            cfg.palette.base.ansi.yellow.base.as_str(),
            cfg.palette.base.ansi.yellow.dim.as_str(),
            warning_background,
        ),
        (
            "error",
            cfg.palette.base.ansi.red.base.as_str(),
            cfg.palette.base.ansi.red.dim.as_str(),
            error_background,
        ),
        (
            "success",
            cfg.palette.base.ansi.green.base.as_str(),
            cfg.palette.base.ansi.green.dim.as_str(),
            success_background,
        ),
        (
            "unreachable",
            cfg.palette.base.ansi.red.base.as_str(),
            cfg.palette.base.ansi.red.dim.as_str(),
            unreachable_background,
        ),
    ];
    for (name, color, border, background_value) in status_defs {
        push_status_with_background(&mut style, name, color, border, background_value);
    }

    style.insert(
        "hidden".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "hidden.border".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert("hidden.background".into(), Value::String(hidden_background));
    style.insert(
        "hint".into(),
        Value::String(cfg.palette.ui.foreground_muted.clone()),
    );
    style.insert(
        "hint.border".into(),
        Value::String(cfg.palette.ui.foreground_muted.clone()),
    );
    style.insert("hint.background".into(), Value::String(hint_background));
    style.insert(
        "ignored".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "ignored.border".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "ignored.background".into(),
        Value::String(ignored_background),
    );
    style.insert(
        "predictive".into(),
        Value::String(cfg.palette.ui.foreground_dim.clone()),
    );
    style.insert(
        "predictive.border".into(),
        Value::String(cfg.palette.base.ansi.blue.base.clone()),
    );
    style.insert(
        "predictive.background".into(),
        Value::String(predictive_background),
    );

    style.insert(
        "notification.background".into(),
        Value::String(elevated_surface.clone()),
    );
    style.insert(
        "notification.border".into(),
        Value::String(cfg.palette.border.border.clone()),
    );
    style.insert(
        "debugger.accent".into(),
        Value::String(cfg.palette.base.ansi.yellow.base.clone()),
    );

    style.insert("vim.mode.text".into(), Value::String(ui.foreground.clone()));
    style.insert(
        "vim.normal.background".into(),
        Value::String(cfg.palette.base.ansi.magenta.base.clone()),
    );
    style.insert(
        "vim.helix_normal.background".into(),
        Value::String(cfg.palette.base.ansi.magenta.base.clone()),
    );
    style.insert(
        "vim.visual.background".into(),
        Value::String(cfg.palette.base.ansi.blue.base.clone()),
    );
    style.insert(
        "vim.helix_select.background".into(),
        Value::String(cfg.palette.base.ansi.blue.base.clone()),
    );
    style.insert(
        "vim.insert.background".into(),
        Value::String(cfg.palette.base.ansi.green.base.clone()),
    );
    style.insert(
        "vim.visual_line.background".into(),
        Value::String(cfg.palette.base.ansi.blue.bright.clone()),
    );
    style.insert(
        "vim.visual_block.background".into(),
        Value::String(cfg.palette.base.ansi.magenta.bright.clone()),
    );
    style.insert(
        "vim.replace.background".into(),
        Value::String(cfg.palette.base.ansi.red.base.clone()),
    );
    style.insert("syntax".into(), build_zed_syntax(cfg, ui));

    Value::Object(style)
}

fn push_status_with_background(
    style: &mut Map<String, Value>,
    name: &str,
    color: &str,
    border: &str,
    background: String,
) {
    style.insert(name.to_string(), Value::String(color.to_string()));
    style.insert(
        format!("{}.border", name),
        Value::String(border.to_string()),
    );
    style.insert(format!("{}.background", name), Value::String(background));
}

fn build_zed_syntax(cfg: &Config, ui: &crate::config::UiPalette) -> Value {
    let mut syntax = Map::new();
    syntax.insert("variable".into(), syntax_entry(&ui.foreground, None, None));
    syntax.insert(
        "variable.builtin".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "variable.parameter".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, None, None),
    );
    syntax.insert(
        "variable.member".into(),
        syntax_entry(&cfg.palette.base.ansi.blue.base, None, None),
    );
    syntax.insert(
        "variable.special".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, Some("italic"), None),
    );
    syntax.insert(
        "constant".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, None, None),
    );
    syntax.insert(
        "constant.builtin".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, None, None),
    );
    syntax.insert(
        "constant.macro".into(),
        syntax_entry(&cfg.palette.syntax.lavender, None, None),
    );
    syntax.insert(
        "module".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, Some("italic"), None),
    );
    syntax.insert(
        "label".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, None, None),
    );
    syntax.insert(
        "string".into(),
        syntax_entry(&cfg.palette.syntax.teal, None, None),
    );
    syntax.insert(
        "string.documentation".into(),
        syntax_entry(&cfg.palette.syntax.teal, None, None),
    );
    syntax.insert(
        "string.regexp".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, None, None),
    );
    syntax.insert(
        "string.regex".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, None, None),
    );
    syntax.insert(
        "string.escape".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "string.special".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "string.special.path".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "string.special.symbol".into(),
        syntax_entry(&cfg.palette.base.ansi.red.base, None, None),
    );
    syntax.insert(
        "string.special.url".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, Some("italic"), None),
    );
    syntax.insert(
        "character".into(),
        syntax_entry(&cfg.palette.syntax.teal, None, None),
    );
    syntax.insert(
        "character.special".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "boolean".into(),
        syntax_entry(&cfg.palette.syntax.lavender, None, None),
    );
    syntax.insert(
        "number".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, None, None),
    );
    syntax.insert(
        "number.float".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, None, None),
    );
    syntax.insert(
        "type".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "type.builtin".into(),
        syntax_entry(&cfg.palette.syntax.lavender, Some("italic"), None),
    );
    syntax.insert(
        "type.definition".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "type.interface".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, Some("italic"), None),
    );
    syntax.insert(
        "type.super".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, Some("italic"), None),
    );
    syntax.insert(
        "attribute".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, Some("italic"), None),
    );
    syntax.insert(
        "property".into(),
        syntax_entry(&cfg.palette.base.ansi.blue.base, None, None),
    );
    syntax.insert(
        "function".into(),
        syntax_entry(&cfg.palette.syntax.teal, None, None),
    );
    syntax.insert(
        "function.builtin".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "function.call".into(),
        syntax_entry(&cfg.palette.syntax.teal, None, None),
    );
    syntax.insert(
        "function.macro".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, None, None),
    );
    syntax.insert(
        "function.method".into(),
        syntax_entry(&cfg.palette.syntax.teal, None, None),
    );
    syntax.insert(
        "function.method.call".into(),
        syntax_entry(&cfg.palette.syntax.teal, None, None),
    );
    syntax.insert(
        "constructor".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "operator".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, None, None),
    );
    syntax.insert(
        "keyword".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.modifier".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.type".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.coroutine".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.function".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.operator".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.import".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.repeat".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.return".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.debug".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.exception".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.conditional".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.conditional.ternary".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, None),
    );
    syntax.insert(
        "keyword.directive".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "keyword.directive.define".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "keyword.export".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, None, None),
    );
    syntax.insert(
        "punctuation".into(),
        syntax_entry(&cfg.palette.ui.foreground_dim, None, None),
    );
    syntax.insert(
        "punctuation.delimiter".into(),
        syntax_entry(&cfg.palette.ui.foreground_dim, None, None),
    );
    syntax.insert(
        "punctuation.bracket".into(),
        syntax_entry(&cfg.palette.ui.foreground_dim, None, None),
    );
    syntax.insert(
        "punctuation.special".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "punctuation.special.symbol".into(),
        syntax_entry(&cfg.palette.base.ansi.red.base, None, None),
    );
    syntax.insert(
        "punctuation.list_marker".into(),
        syntax_entry(&cfg.palette.syntax.teal, None, None),
    );
    syntax.insert(
        "comment".into(),
        syntax_entry(&cfg.palette.syntax.gray, Some("italic"), None),
    );
    syntax.insert(
        "comment.doc".into(),
        syntax_entry(&cfg.palette.syntax.gray, Some("italic"), None),
    );
    syntax.insert(
        "comment.documentation".into(),
        syntax_entry(&cfg.palette.syntax.gray, Some("italic"), None),
    );
    syntax.insert(
        "comment.error".into(),
        syntax_entry(&cfg.palette.base.ansi.red.base, Some("italic"), None),
    );
    syntax.insert(
        "comment.warning".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, Some("italic"), None),
    );
    syntax.insert(
        "comment.hint".into(),
        syntax_entry(&cfg.palette.base.ansi.blue.base, Some("italic"), None),
    );
    syntax.insert(
        "comment.todo".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, Some("italic"), None),
    );
    syntax.insert(
        "comment.note".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, Some("italic"), None),
    );
    syntax.insert(
        "diff.plus".into(),
        syntax_entry(&cfg.palette.base.ansi.green.base, None, None),
    );
    syntax.insert(
        "diff.minus".into(),
        syntax_entry(&cfg.palette.base.ansi.red.base, None, None),
    );
    syntax.insert(
        "tag".into(),
        syntax_entry(&cfg.palette.base.ansi.red.base, None, None),
    );
    syntax.insert(
        "tag.attribute".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, Some("italic"), None),
    );
    syntax.insert(
        "tag.delimiter".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, None, None),
    );
    syntax.insert(
        "parameter".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, None, None),
    );
    syntax.insert(
        "field".into(),
        syntax_entry(&cfg.palette.base.ansi.blue.base, None, None),
    );
    syntax.insert(
        "namespace".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, Some("italic"), None),
    );
    syntax.insert(
        "float".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, None, None),
    );
    syntax.insert(
        "symbol".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert("text".into(), syntax_entry(&ui.foreground, None, None));
    syntax.insert(
        "emphasis".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, Some("italic"), None),
    );
    syntax.insert(
        "emphasis.strong".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, Some(700)),
    );
    syntax.insert(
        "embedded".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "text.literal".into(),
        syntax_entry(&cfg.palette.syntax.lavender, None, None),
    );
    syntax.insert(
        "concept".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, None, None),
    );
    syntax.insert(
        "enum".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, None, Some(700)),
    );
    syntax.insert(
        "function.decorator".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, None, None),
    );
    syntax.insert(
        "type.class.definition".into(),
        syntax_entry(&cfg.palette.syntax.blue_green, None, Some(700)),
    );
    syntax.insert(
        "hint".into(),
        syntax_entry(&cfg.palette.syntax.gray, Some("italic"), None),
    );
    syntax.insert(
        "link_text".into(),
        syntax_entry(&cfg.palette.base.ansi.cyan.base, None, None),
    );
    syntax.insert(
        "link_uri".into(),
        syntax_entry(&cfg.palette.base.ansi.blue.base, Some("italic"), None),
    );
    syntax.insert(
        "parent".into(),
        syntax_entry(&cfg.palette.base.ansi.yellow.base, None, None),
    );
    syntax.insert(
        "predictive".into(),
        syntax_entry(&cfg.palette.ui.foreground_dim, None, None),
    );
    syntax.insert(
        "predoc".into(),
        syntax_entry(&cfg.palette.base.ansi.red.base, None, None),
    );
    syntax.insert(
        "primary".into(),
        syntax_entry(&cfg.palette.base.ansi.magenta.base, None, None),
    );
    syntax.insert(
        "tag.doctype".into(),
        syntax_entry(&cfg.palette.syntax.lavender, None, None),
    );
    syntax.insert(
        "string.doc".into(),
        syntax_entry(&cfg.palette.syntax.teal, Some("italic"), None),
    );
    syntax.insert(
        "title".into(),
        syntax_entry(&cfg.palette.base.ansi.blue.base, None, Some(800)),
    );
    syntax.insert(
        "variant".into(),
        syntax_entry(&cfg.palette.base.ansi.red.base, None, None),
    );
    Value::Object(syntax)
}

fn gen_cursor(cfg: &Config, target: &crate::config::Target, root: &Path) -> Result<()> {
    let dir = root.join(&target.path);
    fs::create_dir_all(&dir)?;
    for v in &cfg.variants {
        if target
            .out_names
            .as_ref()
            .is_some_and(|m| !m.contains_key(&v.name))
        {
            continue;
        }
        let ui = ui_with_variant(cfg, v);
        let name = target
            .out_names
            .as_ref()
            .and_then(|m| m.get(&v.name))
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "{}-{}.json",
                    cfg.meta.name.to_lowercase().replace(' ', "-"),
                    v.name
                )
            });
        let variant_suffix = if v.name == "base" {
            String::new()
        } else {
            format!(" ({})", capitalize(&v.name))
        };

        // Create sidebar/panel backgrounds with variant-aware alpha
        let sidebar_bg = if v.alpha.is_some() {
            ui.background_alt.clone()
        } else {
            cfg.palette.ui.background_alt.clone()
        };
        let elevated_bg = if v.alpha.is_some() {
            ui.background_elevated.clone()
        } else {
            cfg.palette.ui.background_elevated.clone()
        };
        let list_selection_bg = shaded(&cfg.palette.ui.selection, 0.72);

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
                "activityBar.activeBackground": sidebar_bg,
                "activityBar.activeBorder": cfg.palette.border.border_selected,
                "activityBar.activeFocusBorder": cfg.palette.border.border_focused,
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
                "tab.activeBackground": ui.background_elevated,
                "tab.activeForeground": ui.foreground,
                "tab.inactiveBackground": sidebar_bg,
                "tab.inactiveForeground": cfg.palette.ui.foreground_muted,
                "tab.border": cfg.palette.border.border_variant,
                "tab.activeBorder": cfg.palette.base.ansi.cyan.base,
                "tab.activeBorderTop": cfg.palette.base.ansi.cyan.base,
                "tab.hoverBackground": ui.background_elevated,

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
                "list.activeSelectionBackground": list_selection_bg.clone(),
                "list.activeSelectionForeground": ui.foreground,
                "list.inactiveSelectionBackground": cfg.palette.border.border_variant,
                "list.inactiveSelectionForeground": ui.foreground,
                "list.hoverBackground": cfg.palette.border.border_variant,
                "list.hoverForeground": ui.foreground,
                "list.focusBackground": list_selection_bg,
                "list.focusForeground": ui.foreground,
                "list.focusOutline": cfg.palette.border.border_variant,
                "list.focusAndSelectionOutline": cfg.palette.border.border_variant,
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

                // Chat (Cursor / VS Code chat input)
                "chat.inputBackground": elevated_bg,
                "chat.inputForeground": ui.foreground,
                "chat.inputBorder": cfg.palette.border.border,
                "chat.inputPlaceholderForeground": cfg.palette.ui.foreground_dim,

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
                {"scope": ["entity.name.type", "entity.name.class", "support.type", "support.class"], "settings": {"foreground": cfg.palette.base.ansi.magenta.base}},
                {"scope": ["entity.name.tag"], "settings": {"foreground": cfg.palette.base.ansi.red.base}},
                {"scope": ["entity.other.attribute-name"], "settings": {"foreground": cfg.palette.base.ansi.magenta.base, "fontStyle": "italic"}},
                {"scope": ["support.type.property-name", "meta.object-literal.key"], "settings": {"foreground": ui.foreground}},
                {"scope": ["punctuation.brackets", "punctuation.section", "punctuation.separator", "punctuation.delimiter", "meta.brace", "meta.bracket"], "settings": {"foreground": cfg.palette.ui.foreground_muted}},
                {"scope": ["punctuation"], "settings": {"foreground": cfg.palette.ui.foreground_muted}},
                {"scope": ["meta.embedded", "source.groovy.embedded"], "settings": {"foreground": ui.foreground}},
                {"scope": ["markup.heading"], "settings": {"foreground": cfg.palette.base.ansi.blue.base}},
                {"scope": ["markup.bold"], "settings": {"foreground": cfg.palette.base.ansi.magenta.base, "fontStyle": "bold"}},
                {"scope": ["markup.italic"], "settings": {"foreground": cfg.palette.base.ansi.magenta.base, "fontStyle": "italic"}},
                {"scope": ["markup.inline.raw"], "settings": {"foreground": cfg.palette.syntax.lavender}}
            ]
        });
        fs::write(dir.join(name), serde_json::to_string_pretty(&theme)?)?;
    }
    Ok(())
}

fn gen_neovim(cfg: &Config, target: &crate::config::Target, root: &Path) -> Result<()> {
    let dir = root.join(&target.path);
    fs::create_dir_all(&dir)?;
    for v in &cfg.variants {
        if target
            .out_names
            .as_ref()
            .is_some_and(|m| !m.contains_key(&v.name))
        {
            continue;
        }
        let ui = ui_with_variant(cfg, v);
        let name = target
            .out_names
            .as_ref()
            .and_then(|m| m.get(&v.name))
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "{}-{}.lua",
                    cfg.meta.name.to_lowercase().replace(' ', "-"),
                    v.name
                )
            });
        let variant_suffix = if v.name == "base" {
            String::new()
        } else {
            format!(" ({})", capitalize(&v.name))
        };
        let lua = format!(
            r#"-- Generated by colorloom
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

fn gen_website(cfg: &Config, target: &crate::config::Target, root: &Path) -> Result<()> {
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
    let file = target
        .out_file
        .clone()
        .unwrap_or_else(|| "palette.json".to_string());
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
