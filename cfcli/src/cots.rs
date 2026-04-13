//! CoTS (Concise Trust Anchor Store) create and display operations.

use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use cots::arrays::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::utils::find_files;
use crate::{CotsCommand, CotsCreateSubcommand, CotsSubCommands, DisplaySubcommand};

/// Dispatch CoTS subcommands.
pub fn cots_main(args: &CotsCommand) {
    match &args.command {
        CotsSubCommands::Create(c) => cots_create(c),
        CotsSubCommands::Display(c) => cots_display(c),
        CotsSubCommands::CreateStore(_) => {
            println!("CoTS create-store is not yet implemented");
        }
        CotsSubCommands::CreateCorim(_) => {
            println!("CoTS create-corim is not yet implemented");
        }
    }
}

/// Create CBOR-encoded CoTS files from JSON templates.
fn cots_create(args: &CotsCreateSubcommand) {
    if args.template.is_none() && args.template_dir.as_ref().is_none_or(|d| d.is_empty()) {
        println!("No templates supplied");
        return;
    }

    let mut files = vec![];
    if let Some(f) = &args.template {
        files.push(f.clone());
    }

    if let Some(f) = args.template_dir.as_ref() {
        find_files(f, "json", &mut files)
    }

    let output_dir = Path::new(&args.output_dir);

    for f in &files {
        cots_template_to_cbor(f, output_dir);
    }
}

/// Decode and display a CBOR-encoded CoTS as JSON.
fn cots_display(args: &DisplaySubcommand) {
    if matches!(args.format, crate::args::DisplayFormat::Diag) {
        crate::cbor_diag::display_diag(&args.file_to_display);
        return;
    }
    let data = match fs::read(&args.file_to_display) {
        Ok(b) => b,
        Err(e) => {
            println!(
                "Unable to read CoMID to display from {} with error {}",
                args.file_to_display, e
            );
            return;
        }
    };
    let cbor: ConciseTaStoresCbor = match from_reader(data.as_slice()) {
        Ok(c) => c,
        Err(e) => {
            println!(
                "Unable to parse data read from {} as a CBOR-encoded CoMID with error {}",
                args.file_to_display, e
            );
            return;
        }
    };
    let json: ConciseTaStores = match cbor.try_into() {
        Ok(s) => s,
        Err(_) => {
            println!(
                "Unable to convert CBOR CoMID object to JSON CoMID object for {}",
                args.file_to_display
            );
            return;
        }
    };

    let json = match serde_json::to_string(&json) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to produce JSON CoMID object for {} with error: {}",
                args.file_to_display, e
            );
            return;
        }
    };
    println!("{}", json);
}

/// Convert a single CoTS JSON template to a CBOR-encoded file.
fn cots_template_to_cbor(template_file: &String, output_dir: &Path) {
    let data = match fs::read_to_string(template_file) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to read CoMID template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let comid_json: ConciseTaStores = match serde_json::from_str(&data) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to parse CoMID template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let comid_cbor: ConciseTaStoresCbor = match comid_json.try_into() {
        Ok(s) => s,
        Err(_) => {
            println!(
                "Unable to convert JSON CoMID object to CBOR CoMID object for template {}",
                template_file
            );
            return;
        }
    };

    let mut encoded_token = vec![];
    match into_writer(&comid_cbor, &mut encoded_token) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "Unable to generate CBOR-encoded CoMID from template in {} with error {}",
                template_file, e
            )
        }
    };

    let template_path = Path::new(template_file);
    let template_filename = match template_path.file_name() {
        Some(s) => s,
        None => {
            println!("Failed to read file name from template {}", template_file);
            return;
        }
    };

    let output_path = Path::new(output_dir);
    let filename_str = match template_filename.to_str() {
        Some(s) => s,
        None => {
            println!("Failed to convert filename to string");
            return;
        }
    };
    let mut output_pathbuf = output_path.join(filename_str);
    output_pathbuf.set_extension("cbor");

    let mut output_file = match File::create(&output_pathbuf) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to create output file {:?}: {}", output_pathbuf, e);
            return;
        }
    };
    if let Err(e) = output_file.write_all(encoded_token.as_slice()) {
        println!("Failed to write CoTS file {:?}: {}", output_pathbuf, e);
    }
}
