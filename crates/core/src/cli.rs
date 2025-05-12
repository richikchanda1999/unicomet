use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Unicomet project
    Init(InitArgs),
    
    /// Publish your Unicomet project
    Publish(PublishArgs),
    
    /// Run development server
    Dev(DevArgs),
}

#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    /// Project name
    #[arg(short, long)]
    pub name: Option<String>,
    
    /// Project description
    #[arg(short, long)]
    pub description: Option<String>,
    
    /// Initialize git repository
    #[arg(short, long, default_value = "true")]
    pub git: bool,
    
    /// Author information
    #[arg(short = 'a', long)]
    pub author: Option<String>,
    
    /// License type
    #[arg(short, long)]
    pub license: Option<String>,
    
    /// Boilerplate template
    #[arg(short, long)]
    pub boilerplate: Option<String>,
    
    /// Language [TS/JS]
    #[arg(short = 'T', long, default_value = "true")]
    pub typescript: bool,
    
    /// Include web3 library
    #[arg(short = 'w', long)]
    pub web3: Option<String>,
    
    /// Target chains (comma separated)
    #[arg(short, long)]
    pub chains: Option<String>,
    
    /// Spin up Anvil on dev runs
    #[arg(short = 'A', long, default_value = "true")]
    pub anvil: bool,
    
    /// Include a "Hello, Uniswap!" sample script
    #[arg(short = 'S', long)]
    pub sample: bool,
    
    /// Generate GitHub Actions workflow
    #[arg(short = 'G', long, default_value = "true")]
    pub github_actions: bool,
    
    /// Configure 'unicomet publish' defaults
    #[arg(short, long)]
    pub publish_config: Option<String>,
    
    /// Skip confirmation prompt
    #[arg(short = 'y', long)]
    pub yes: bool,
}

#[derive(Args, Debug, Clone)]
pub struct PublishArgs {
    /// Publishing target (e.g., "production", "staging")
    #[arg(short, long)]
    pub target: Option<String>,
    
    /// Version to publish
    #[arg(short, long)]
    pub version: Option<String>,
    
    // Add other publish-specific arguments
}

#[derive(Args, Debug, Clone)]
pub struct DevArgs {
    /// Port for the development server
    #[arg(short, long, default_value = "3000")]
    pub port: Option<u16>,
    
    /// Automatically open in browser
    #[arg(short, long, default_value = "true")]
    pub open: bool,
    
    // Add other dev-specific arguments
}