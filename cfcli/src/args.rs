//! Arguments for the cfcli utility

use clap::{Args, Parser, Subcommand, ValueEnum};

/// Signing format: COSE (CWT) or JWS (JWT).
#[derive(Clone, Debug, Default, ValueEnum)]
pub enum SigningFormat {
    /// COSE Sign1 (CWT) — default
    #[default]
    Cose,
    /// JWS Compact Serialization (JWT)
    Jws,
}

/// Display output format.
#[derive(Clone, Debug, Default, ValueEnum)]
pub enum DisplayFormat {
    /// JSON (decoded to JSON-friendly structs) — default
    #[default]
    Json,
    /// CBOR diagnostic notation (RFC 8949 Section 8)
    Diag,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create and display CoMID (Concise Module Identifier) objects
    Comid(ComidCommand),
    /// Create, display, sign, verify, and extract CoRIM (Concise Reference Integrity Manifest) objects
    Corim(CorimCommand),
    /// Create and display CoSWID (Concise Software Identifier) objects
    Coswid(CoswidCommand),
    /// Create and display CoTS (Concise Trust Anchor Store) objects
    Cots(CotsCommand),
    /// Create, display, sign, verify, and extract CoSERV (Concise Service) objects
    Coserv(CoservCommand),
    /// Create, display, sign, verify, and extract EAR (EAT Attestation Result) objects
    Ear(EarCommand),
    /// Create, display, sign, verify, and extract EAT (Entity Attestation Token) objects
    Eat(EatCommand),
    /// Convert between CBOR binary and diagnostic notation
    Diag(DiagCommand),
}

//----------------------------------------------------------
//----------------------------------------------------------
#[derive(Args, Debug)]
pub struct DisplaySubcommand {
    /// a CBOR-encoded file to decode and display
    #[clap(short, long)]
    pub file_to_display: String,

    /// output format: json (default) or diag (CBOR diagnostic notation)
    #[clap(long, value_enum, default_value_t = DisplayFormat::Json)]
    pub format: DisplayFormat,
}

//----------------------------------------------------------
// CoMID
//----------------------------------------------------------
/// CoMID operations
#[derive(Args, Debug)]
pub struct ComidCommand {
    #[clap(subcommand)]
    pub command: ComidSubCommands,
}
#[derive(Subcommand, Debug)]
pub enum ComidSubCommands {
    /// Create a CBOR-encoded CoMID from a JSON template
    Create(ComidCreateSubcommand),
    /// Decode and display a CBOR-encoded CoMID
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
/// CoRIM operations
#[derive(Args, Debug)]
pub struct CorimCommand {
    #[clap(subcommand)]
    pub command: CorimSubCommands,
}
#[derive(Subcommand, Debug)]
pub enum CorimSubCommands {
    /// Create an unsigned CoRIM from CoMID/CoSWID tags and a JSON template
    Create(CorimCreateSubcommand),
    /// Decode and display a CBOR-encoded CoRIM
    Display(DisplaySubcommand),
    /// Sign a CoRIM using a COSE Sign1 structure
    Sign(CorimSignSubcommand),
    /// Verify the signature on a signed CoRIM
    Verify(CorimVerifySubcommand),
    /// Extract the payload and tags from a signed CoRIM
    Extract(CorimExtractSubcommand),
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
#[derive(Args, Debug)]
pub struct CorimSignSubcommand {
    /// an unsigned CoRIM file (in CBOR format)
    #[clap(short, long)]
    pub corim_file: String,

    /// a key file (JWK JSON or COSE Key CBOR)
    #[clap(short, long)]
    pub key_file: String,

    /// a CoRIM meta template file (in JSON format)
    #[clap(short, long)]
    pub meta_file: String,

    /// directory where the signed file is stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}
#[derive(Args, Debug)]
pub struct CorimVerifySubcommand {
    /// a signed CoRIM file (COSE Sign1, tag #18)
    #[clap(short = 'f', long)]
    pub signed_corim_file: String,

    /// a key file (JWK JSON or COSE Key CBOR)
    #[clap(short, long)]
    pub key_file: String,
}
#[derive(Args, Debug)]
pub struct CorimExtractSubcommand {
    /// a signed CoRIM file (COSE Sign1, tag #18)
    #[clap(short = 'f', long)]
    pub signed_corim_file: String,

    /// directory where extracted tags are stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}

//----------------------------------------------------------
// CoSWID
//----------------------------------------------------------
/// CoSWID operations
#[derive(Args, Debug)]
pub struct CoswidCommand {
    #[clap(subcommand)]
    pub command: CoswidSubCommands,
}

#[derive(Subcommand, Debug)]
pub enum CoswidSubCommands {
    /// Create a CBOR-encoded CoSWID from a JSON template
    Create(CoswidCreateSubcommand),
    /// Decode and display a CBOR-encoded CoSWID
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
/// CoTS operations
#[derive(Args, Debug)]
pub struct CotsCommand {
    #[clap(subcommand)]
    pub command: CotsSubCommands,
}
#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
pub enum CotsSubCommands {
    /// Create a CBOR-encoded CoTS from concise-ta-store-map files and/or a JSON template
    Create(CotsCreateSubcommand),
    /// Decode and display a CBOR-encoded CoTS
    Display(DisplaySubcommand),
    /// Create a concise-ta-store-map from certificates and environment templates
    CreateStore(CotsCreateStoreSubcommand),
    /// Create a CoRIM containing a CoTS payload
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
// CoSERV
//----------------------------------------------------------
/// CoSERV operations
#[derive(Args, Debug)]
pub struct CoservCommand {
    #[clap(subcommand)]
    pub command: CoservSubCommands,
}
#[derive(Subcommand, Debug)]
pub enum CoservSubCommands {
    /// Create a CBOR-encoded CoSERV from a JSON template
    Create(CoservCreateSubcommand),
    /// Decode and display a CBOR-encoded CoSERV
    Display(DisplaySubcommand),
    /// Sign a CoSERV using a COSE Sign1 structure
    Sign(CoservSignSubcommand),
    /// Verify the signature on a signed CoSERV
    Verify(CoservVerifySubcommand),
    /// Extract the payload from a signed CoSERV
    Extract(CoservExtractSubcommand),
    /// Create a CBOR-encoded CoSERV discovery document from a JSON template
    CreateDiscovery(CoservCreateSubcommand),
    /// Decode and display a CBOR-encoded CoSERV discovery document
    DisplayDiscovery(DisplaySubcommand),
}
#[derive(Args, Debug)]
pub struct CoservCreateSubcommand {
    /// a CoSERV template file (in JSON format)
    #[clap(short, long)]
    pub template: Option<String>,

    /// a directory containing CoSERV template files
    #[clap(short = 'T', long)]
    pub template_dir: Option<String>,

    /// directory where the created files are stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}
#[derive(Args, Debug)]
pub struct CoservSignSubcommand {
    /// an unsigned CoSERV file (in CBOR format)
    #[clap(short = 'f', long)]
    pub coserv_file: String,

    /// a key file (JWK JSON or COSE Key CBOR)
    #[clap(short, long)]
    pub key_file: String,

    /// directory where the signed file is stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}
#[derive(Args, Debug)]
pub struct CoservVerifySubcommand {
    /// a signed CoSERV file (COSE Sign1, tag #18)
    #[clap(short = 'f', long)]
    pub signed_coserv_file: String,

    /// a key file (JWK JSON or COSE Key CBOR)
    #[clap(short, long)]
    pub key_file: String,
}
#[derive(Args, Debug)]
pub struct CoservExtractSubcommand {
    /// a signed CoSERV file (COSE Sign1, tag #18)
    #[clap(short = 'f', long)]
    pub signed_coserv_file: String,

    /// directory where the extracted payload is stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}

//----------------------------------------------------------
// EAR
//----------------------------------------------------------
/// EAR operations
#[derive(Args, Debug)]
pub struct EarCommand {
    #[clap(subcommand)]
    pub command: EarSubCommands,
}
#[derive(Subcommand, Debug)]
pub enum EarSubCommands {
    /// Create a CBOR-encoded EAR from a JSON template
    Create(EarCreateSubcommand),
    /// Decode and display a CBOR-encoded EAR
    Display(DisplaySubcommand),
    /// Sign an EAR using a COSE Sign1 structure or JWS
    Sign(EarSignSubcommand),
    /// Verify the signature on a signed EAR
    Verify(EarVerifySubcommand),
    /// Extract the payload from a signed EAR
    Extract(EarExtractSubcommand),
}
#[derive(Args, Debug)]
pub struct EarCreateSubcommand {
    /// an EAR template file (in JSON format)
    #[clap(short, long)]
    pub template: Option<String>,

    /// a directory containing EAR template files
    #[clap(short = 'T', long)]
    pub template_dir: Option<String>,

    /// directory where the created files are stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,
}
#[derive(Args, Debug)]
pub struct EarSignSubcommand {
    /// an unsigned EAR file (in CBOR format)
    #[clap(short = 'f', long)]
    pub ear_file: String,

    /// a key file (JWK JSON or COSE Key CBOR)
    #[clap(short, long)]
    pub key_file: String,

    /// directory where the signed file is stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,

    /// signing format: cose (default) or jws
    #[clap(long, value_enum, default_value_t = SigningFormat::Cose)]
    pub format: SigningFormat,
}
#[derive(Args, Debug)]
pub struct EarVerifySubcommand {
    /// a signed EAR file (COSE Sign1 or JWS compact)
    #[clap(short = 'f', long)]
    pub signed_ear_file: String,

    /// a key file (JWK JSON or COSE Key CBOR)
    #[clap(short, long)]
    pub key_file: String,

    /// signing format: cose (default) or jws
    #[clap(long, value_enum, default_value_t = SigningFormat::Cose)]
    pub format: SigningFormat,
}
#[derive(Args, Debug)]
pub struct EarExtractSubcommand {
    /// a signed EAR file (COSE Sign1 or JWS compact)
    #[clap(short = 'f', long)]
    pub signed_ear_file: String,

    /// directory where the extracted payload is stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,

    /// signing format: cose (default) or jws
    #[clap(long, value_enum, default_value_t = SigningFormat::Cose)]
    pub format: SigningFormat,
}

//----------------------------------------------------------
// EAT
//----------------------------------------------------------
/// EAT operations
#[derive(Args, Debug)]
pub struct EatCommand {
    #[clap(subcommand)]
    pub command: EatSubCommands,
}
#[derive(Subcommand, Debug)]
pub enum EatSubCommands {
    /// Create a CBOR-encoded EAT from a JSON template
    Create(EatCreateSubcommand),
    /// Decode and display a CBOR-encoded EAT
    Display(DisplaySubcommand),
    /// Sign an EAT using a COSE Sign1 structure or JWS
    Sign(EatSignSubcommand),
    /// Verify the signature on a signed EAT
    Verify(EatVerifySubcommand),
    /// Extract the payload from a signed EAT
    Extract(EatExtractSubcommand),
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
#[derive(Args, Debug)]
pub struct EatSignSubcommand {
    /// an unsigned EAT file (in CBOR format)
    #[clap(short = 'f', long)]
    pub eat_file: String,

    /// a key file (JWK JSON or COSE Key CBOR)
    #[clap(short, long)]
    pub key_file: String,

    /// directory where the signed file is stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,

    /// signing format: cose (default) or jws
    #[clap(long, value_enum, default_value_t = SigningFormat::Cose)]
    pub format: SigningFormat,
}
#[derive(Args, Debug)]
pub struct EatVerifySubcommand {
    /// a signed EAT file (COSE Sign1 or JWS compact)
    #[clap(short = 'f', long)]
    pub signed_eat_file: String,

    /// a key file (JWK JSON or COSE Key CBOR)
    #[clap(short, long)]
    pub key_file: String,

    /// signing format: cose (default) or jws
    #[clap(long, value_enum, default_value_t = SigningFormat::Cose)]
    pub format: SigningFormat,
}
#[derive(Args, Debug)]
pub struct EatExtractSubcommand {
    /// a signed EAT file (COSE Sign1 or JWS compact)
    #[clap(short = 'f', long)]
    pub signed_eat_file: String,

    /// directory where the extracted payload is stored
    #[clap(short, long, default_value = ".")]
    pub output_dir: String,

    /// signing format: cose (default) or jws
    #[clap(long, value_enum, default_value_t = SigningFormat::Cose)]
    pub format: SigningFormat,
}

//----------------------------------------------------------
// Diag
//----------------------------------------------------------
/// CBOR diagnostic notation conversions
#[derive(Args, Debug)]
pub struct DiagCommand {
    #[clap(subcommand)]
    pub command: DiagSubCommands,
}
#[derive(Subcommand, Debug)]
pub enum DiagSubCommands {
    /// Convert CBOR diagnostic notation to binary CBOR
    ToCbor(DiagToCborSubcommand),
    /// Convert binary CBOR to diagnostic notation
    ToDiag(DiagToDiagSubcommand),
}
#[derive(Args, Debug)]
pub struct DiagToCborSubcommand {
    /// input file containing CBOR diagnostic notation (or - for stdin)
    #[clap(short, long)]
    pub input: String,

    /// output file for binary CBOR
    #[clap(short, long)]
    pub output: String,
}
#[derive(Args, Debug)]
pub struct DiagToDiagSubcommand {
    /// input file containing binary CBOR
    #[clap(short, long)]
    pub input: String,
}

/// CLI utility for creating, displaying, signing, and verifying CBOR-encoded RATS and SCITT objects
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct CfcliArgs {
    #[clap(subcommand)]
    pub command: Commands,
}
