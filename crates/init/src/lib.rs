use std::process::{Command, exit};
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm, MultiSelect};
use indicatif::{ProgressBar, ProgressStyle};
use unicomet_core::cli::InitArgs;

// Available templates
const TEMPLATES: &[&str] = &[
    "React + Vite",
    "Next.js (TS)",
    "Vanilla JS",
    "Other (path)",
];

// Available licenses
const LICENSES: &[&str] = &[
    "MIT",
    "Apache-2.0",
    "GPL-3.0",
    "BSD-3-Clause",
    "None",
];

// Available chains
const CHAINS: &[&str] = &[
    "Ethereum",
    "Polygon",
    "Arbitrum",
    "Optimism",
    "Base",
    "zkSync",
    "Solana",
];

// Available web3 libraries
const WEB3_LIBS: &[&str] = &[
    "ethers.js",
    "viem",
    "web3.js",
    "None",
];

pub fn execute(args: InitArgs) {
    let term = Term::stdout();
    let theme = ColorfulTheme::default();
    
    println!("{}", style("🚀 Initializing new Unicomet project...").bold().cyan());
    
    // Project name
    let name = match args.name {
        Some(name) => name,
        None => text_input(&term, &theme, "What should I call your new extension?", None),
    };
    
    // Project description
    let description = match args.description {
        Some(desc) => desc,
        None => text_input(&term, &theme, "Add a short description (optional)", Some(String::new())),
    };
    
    // Git init
    let init_git = match args.git {
        true => {
            if args.yes {
                true
            } else {
                confirm(&term, &theme, "Initialize a Git repo?", true)
            }
        },
        false => false,
    };
    
    // Author info
    let author = match args.author {
        Some(author) => author,
        None => {
            let git_name = get_git_config("user.name").unwrap_or_default();
            let git_email = get_git_config("user.email").unwrap_or_default();
            let default = if !git_name.is_empty() && !git_email.is_empty() {
                format!("{} <{}>", git_name, git_email)
            } else {
                String::new()
            };
            
            text_input(&term, &theme, "Author (name <email>)", Some(default))
        }
    };
    
    // License
    let license = match args.license {
        Some(license) => license,
        None => {
            let index = Select::with_theme(&theme)
                .with_prompt("License")
                .default(0)
                .items(LICENSES)
                .interact_on(&term)
                .unwrap();
            LICENSES[index].to_string()
        }
    };
    
    // Boilerplate
    let boilerplate = match args.boilerplate {
        Some(template) => template,
        None => {
            let index = Select::with_theme(&theme)
                .with_prompt("Which boilerplate?")
                .default(0)
                .items(TEMPLATES)
                .interact_on(&term)
                .unwrap();
            TEMPLATES[index].to_string()
        }
    };
    
    // Language
    let typescript = if args.yes {
        args.typescript
    } else {
        let lang = text_input(&term, &theme, "Language (TS/JS)", Some("TS".to_string()));
        lang.to_uppercase() == "TS"
    };
    
    // Web3 library
    let web3_lib = match args.web3 {
        Some(lib) => lib,
        None => {
            let index = Select::with_theme(&theme)
                .with_prompt("Include web3 library?")
                .default(0)
                .items(WEB3_LIBS)
                .interact_on(&term)
                .unwrap();
            WEB3_LIBS[index].to_string()
        }
    };
    
    // Target chains
    let chains = match args.chains {
        Some(chains) => chains,
        None => {
            let defaults = vec![true, true, false, false, false, false, false];
            let selections = MultiSelect::with_theme(&theme)
                .with_prompt("Target chains (␣ to select)")
                .items(CHAINS)
                .defaults(&defaults)
                .interact_on(&term)
                .unwrap();
                
            selections.iter()
                .map(|&i| CHAINS[i])
                .collect::<Vec<&str>>()
                .join(", ")
        }
    };
    
    // Anvil
    let anvil = match args.anvil {
        true => {
            if args.yes {
                true
            } else {
                confirm(&term, &theme, "Spin up Anvil on dev runs?", true)
            }
        },
        false => false,
    };
    
    // Sample
    let sample = if args.yes {
        args.sample
    } else {
        confirm(&term, &theme, "Include a \"Hello, Uniswap!\" sample script?", false)
    };
    
    // GitHub Actions
    let github_actions = match args.github_actions {
        true => {
            if args.yes {
                true
            } else {
                confirm(&term, &theme, "Generate GitHub Actions workflow for build/test?", true)
            }
        },
        false => false,
    };
    
    // Publish config
    let publish_config = match args.publish_config {
        Some(config) => config,
        None => {
            if args.yes {
                String::new()
            } else {
                let input = text_input(
                    &term, 
                    &theme, 
                    "Configure 'unicomet publish' defaults? (repo/org, branch)", 
                    Some(String::new())
                );
                
                if input.trim().is_empty() {
                    "(skip)".to_string()
                } else {
                    input
                }
            }
        }
    };
    
    // Confirmation
    if !args.yes {
        let proceed = confirm(
            &term, 
            &theme, 
            &format!("Proceed to scaffold project \"{}\"?", name), 
            true
        );
        
        if !proceed {
            println!("Aborting...");
            exit(0);
        }
    }
    
    // Show a progress bar while "scaffolding"
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner} {msg}")
            .unwrap()
    );
    pb.set_message("Creating project files...");
    
    // In a real app, we'd do actual work here
    // For now, let's just simulate it
    std::thread::sleep(std::time::Duration::from_millis(1500));
    pb.finish_and_clear();
    
    println!("\n{} Done!", style("🎉").bold().green());
}

// Helper functions moved from utils/prompts.rs to here to simplify
fn text_input(
    term: &Term,
    theme: &ColorfulTheme,
    prompt: &str,
    default: Option<String>
) -> String {
    let mut input = Input::with_theme(theme).with_prompt(prompt);
    
    if let Some(default_value) = default {
        input = input.default(default_value);
    }
    
    input.interact_on(term).unwrap()
}

fn confirm(
    term: &Term,
    theme: &ColorfulTheme,
    prompt: &str,
    default: bool
) -> bool {
    Confirm::with_theme(theme)
        .with_prompt(prompt)
        .default(default)
        .interact_on(term)
        .unwrap()
}

fn get_git_config(key: &str) -> Option<String> {
    let output = Command::new("git")
        .args(&["config", "--get", key])
        .output()
        .ok()?;
    
    if output.status.success() {
        String::from_utf8(output.stdout)
            .map(|s| s.trim().to_string())
            .ok()
    } else {
        None
    }
}