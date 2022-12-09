//! Arguments for the cfcli utility

use clap::{Args, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Comid(ComidCommand),
    Corim(CorimCommand),
    Coswid(CoswidCommand),
    Cots(CotsCommand),
    Eat(EatCommand),
}

//----------------------------------------------------------
//----------------------------------------------------------
#[derive(Args, Debug)]
pub struct DisplaySubcommand {
    #[clap(short, long)]
    pub file_to_display: String,
}

//----------------------------------------------------------
// CoMID
//----------------------------------------------------------
#[derive(Args, Debug)]
pub struct ComidCommand {
    #[clap(subcommand)]
    pub command: ComidSubCommands,
}
#[derive(Subcommand, Debug)]
pub enum ComidSubCommands {
    Create(ComidCreateSubcommand),
    Display(DisplaySubcommand),
}
#[derive(Args, Debug)]
pub struct ComidCreateSubcommand {
    /// a CoMID template file (in JSON format)
    #[clap(short, long)]
    pub template: Option<String>,

    /// a directory containing CoMID template files
    #[clap(short = 'T', long)]
    pub template_dir: Option<String>,

    /// directory where the created files are stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}

//----------------------------------------------------------
// CoRIM
//----------------------------------------------------------
#[derive(Args, Debug)]
pub struct CorimCommand {
    #[clap(subcommand)]
    pub command: CorimSubCommands,
}
#[derive(Subcommand, Debug)]
pub enum CorimSubCommands {
    Create(CorimCreateSubcommand),
    Display(DisplaySubcommand),
}
#[derive(Args, Debug)]
pub struct CorimCreateSubcommand {
    /// a CBOR-encoded CoMID file
    #[clap(short = 'm', long)]
    pub comid: Option<String>,

    /// a directory containing CBOR-encoded CoMID files
    #[clap(short = 'M', long)]
    pub comid_dir: Option<String>,

    /// a CBOR-encoded CoSWID file
    #[clap(short = 's', long)]
    pub coswid: Option<String>,

    /// a directory containing CBOR-encoded CoSWID files
    #[clap(short = 'S', long)]
    pub coswid_dir: Option<String>,

    /// a CoRIM template file (in JSON format)
    #[clap(short, long)]
    pub template: Option<String>,

    /// a directory containing CoRIM template files
    #[clap(short = 'T', long)]
    pub template_dir: Option<String>,

    /// directory where the created files are stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}

//----------------------------------------------------------
// CoSWID
//----------------------------------------------------------
#[derive(Args, Debug)]
pub struct CoswidCommand {
    #[clap(subcommand)]
    pub command: CoswidSubCommands,
}

#[derive(Subcommand, Debug)]
pub enum CoswidSubCommands {
    Create(CoswidCreateSubcommand),
    Display(DisplaySubcommand),
}
#[derive(Args, Debug)]
pub struct CoswidCreateSubcommand {
    /// a CoSWID template file (in JSON format)
    #[clap(short, long)]
    pub template: Option<String>,

    /// a directory containing CoSWID template files
    #[clap(short = 'T', long)]
    pub template_dir: Option<String>,

    /// directory where the created files are stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}
//----------------------------------------------------------
// CoTS
//----------------------------------------------------------
#[derive(Args, Debug)]
pub struct CotsCommand {
    #[clap(subcommand)]
    pub command: CotsSubCommands,
}
#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
pub enum CotsSubCommands {
    Create(CotsCreateSubcommand),
    Display(DisplaySubcommand),
    CreateStore(CotsCreateStoreSubcommand),
    CreateCorim(CotsCreateCorimSubcommand),
}
#[derive(Args, Debug)]
pub struct CotsCreateSubcommand {
    /// a directory containing binary CBOR-encoded concise-ta-store-map files
    #[clap(short, long)]
    pub cts: Option<Vec<String>>,

    /// a CBOR-encoded concise-ta-store-map file
    #[clap(long)]
    pub ctsfile: Option<Vec<String>>,

    /// a CoTS template file (in JSON format)
    #[clap(short, long)]
    pub template: Option<String>,

    /// a directory containing CoTS template files
    #[clap(short = 'T', long)]
    pub template_dir: Option<String>,

    /// directory where the created files are stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}
#[derive(Args, Debug)]
pub struct CotsCreateStoreSubcommand {
    /// a DER-encoded certificate file
    #[clap(long)]
    pub cafile: Option<Vec<String>>,

    /// a directory containing binary DER-encoded X.509 CA certificate files
    #[clap(short, long)]
    pub cas: Option<Vec<String>>,

    /// an environment template file (in JSON format)
    #[clap(short = 'T', long)]
    pub environment: Option<String>,

    /// an excluded claims template file (in JSON format)
    #[clap(short = 'x', long)]
    pub exclclaims: Option<String>,

    /// string value containing a tag ID value (mutually exclusive from --uuid and --uuid-str)
    #[clap(long)]
    pub id: Option<String>,

    /// nlanguage tag
    #[clap(short, long)]
    pub language: String,

    /// name of the generated (unsigned) CoTS file
    #[clap(short, long, default_value = ".")]
    pub output: String,

    /// an permitted claims template file (in JSON format)
    #[clap(short = 'p', long)]
    pub permclaims: Option<String>,

    /// a directory containing binary DER-encoded X.509 CA certificate files
    #[clap(short = 'u', long)]
    pub purpose: Option<Vec<String>>,

    /// a DER-encoded certificate file
    #[clap(long)]
    pub tafile: Option<Vec<String>>,

    /// integer value indicating version of tag identity (ignored if neither --uuid nor --id are supplied)
    #[clap(long)]
    pub tag_version: Option<u64>,

    /// a directory containing binary DER-encoded trust anchor files
    #[clap(short, long)]
    pub tas: Option<Vec<String>>,

    /// boolean indicating a random UUID value should be used as tag ID (mutually exclusive from --id and --uuid-str)
    #[clap(long)]
    pub uuid: Option<bool>,

    /// string representation of a UUID to use as tag ID (mutually exclusive from --uuid and --id)
    #[clap(long)]
    pub uuid_str: Option<String>,
}
#[derive(Args, Debug)]
pub struct CotsCreateCorimSubcommand {
    /// a CoRIM template file (in JSON format)
    #[clap(short, long)]
    pub template: Option<String>,

    /// a CoTS file (in CBOR format)
    #[clap(short, long)]
    pub cots: Option<String>,

    /// name of the generated (unsigned) CoRIM file
    #[clap(short, long)]
    pub output: String,
}
//----------------------------------------------------------
// EAT
//----------------------------------------------------------
#[derive(Args, Debug)]
pub struct EatCommand {
    #[clap(subcommand)]
    pub command: EatSubCommands,
}
#[derive(Subcommand, Debug)]
pub enum EatSubCommands {
    Create(EatCreateSubcommand),
    Display(DisplaySubcommand),
}
#[derive(Args, Debug)]
pub struct EatCreateSubcommand {
    /// a EAT template file (in JSON format)
    #[clap(short, long)]
    pub template: Option<String>,

    /// a directory containing EAT template files
    #[clap(short = 'T', long)]
    pub template_dir: Option<String>,

    /// directory where the created files are stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}

/// cfcli
#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct CfcliArgs {
    #[clap(subcommand)]
    pub command: Commands,
}
