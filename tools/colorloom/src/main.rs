mod config;
mod targets;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "colorloom", version, about = "Subliminal Nightfall theme generator")] 
struct Cli {
    /// Path to theme TOML
    #[arg(short, long, default_value = "theme.toml")]
    config: PathBuf,

    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Validate the configuration file
    Validate,
    /// Generate themes for all enabled targets
    Generate,
    /// List targets and variants
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = std::env::current_dir()?;
    let cfg_text = fs::read_to_string(&cli.config).with_context(|| format!("reading {}", cli.config.display()))?;
    let cfg: config::Config = toml::from_str(&cfg_text).context("parsing theme.toml")?;

    match cli.cmd.unwrap_or(Cmd::Generate) {
        Cmd::Validate => {
            println!("OK: {} variants, {} targets", cfg.variants.len(), cfg.targets.len());
        }
        Cmd::List => {
            println!("Variants: {}", cfg.variant_names().join(", "));
            for t in cfg.targets.iter().filter(|t| t.enabled) {
                println!("- {} -> {}", t.id, t.path);
            }
        }
        Cmd::Generate => {
            for t in cfg.targets.iter().filter(|t| t.enabled) {
                targets::generate_target(&cfg, t, &root).with_context(|| format!("generating target {}", t.id))?;
            }
            println!("Generated themes for {} targets", cfg.targets.iter().filter(|t| t.enabled).count());
        }
    }
    Ok(())
}
