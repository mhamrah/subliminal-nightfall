use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub meta: Meta,
    pub palette: Palette,
    pub variants: Vec<Variant>,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    pub base: BasePalette,
    pub syntax: SyntaxPalette,
    pub ui: UiPalette,
    pub border: BorderPalette,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasePalette {
    pub ansi: AnsiPalette,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnsiPalette {
    pub red: ColorVariant,
    pub green: ColorVariant,
    pub yellow: ColorVariant,
    pub blue: ColorVariant,
    pub magenta: ColorVariant,
    pub cyan: ColorVariant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorVariant {
    pub base: String,
    pub bright: String,
    pub dim: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxPalette {
    pub teal: String,
    #[serde(rename = "blue_green")]
    pub blue_green: String,
    pub lavender: String,
    pub gray: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiPalette {
    pub background: String,
    #[serde(rename = "background_alt")]
    pub background_alt: String,
    #[serde(rename = "background_elevated")]
    pub background_elevated: String,
    pub foreground: String,
    #[serde(rename = "foreground_muted")]
    pub foreground_muted: String,
    #[serde(rename = "foreground_dim")]
    pub foreground_dim: String,
    pub selection: String,
    pub cursor: String,
    #[serde(rename = "line_highlight")]
    pub line_highlight: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderPalette {
    pub border: String,
    #[serde(rename = "border_variant")]
    pub border_variant: String,
    #[serde(rename = "border_focused")]
    pub border_focused: String,
    #[serde(rename = "border_selected")]
    pub border_selected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub alpha: Option<f32>,
    #[serde(default)]
    pub blur_radius: Option<u32>,
    #[serde(default)]
    pub overrides: Option<Overrides>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Overrides {
    pub ui: Option<UiOverrides>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UiOverrides {
    pub background: Option<String>,
    pub background_alt: Option<String>,
    pub background_elevated: Option<String>,
    pub selection: Option<String>,
    pub cursor: Option<String>,
    pub line_highlight: Option<String>,
    pub foreground: Option<String>,
    pub foreground_muted: Option<String>,
    pub foreground_dim: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub id: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub path: String,
    #[serde(default)]
    pub out_file: Option<String>,
    #[serde(default)]
    pub out_names: Option<std::collections::HashMap<String, String>>, // variant -> filename
}

fn default_true() -> bool { true }

impl Config {
    pub fn variant_names(&self) -> Vec<String> {
        self.variants.iter().map(|v| v.name.clone()).collect()
    }
}
