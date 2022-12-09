use crate::utils::find_files;
use crate::{ComidCommand, ComidCreateSubcommand, ComidSubCommands, DisplaySubcommand};
use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use corim::maps::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn comid_main(args: &ComidCommand) {
    //todo cfcli support
    match &args.command {
        ComidSubCommands::Create(c) => comid_create(c),
        ComidSubCommands::Display(c) => comid_display(c),
    }
}

fn comid_create(args: &ComidCreateSubcommand) {
    if args.template.is_none()
        && (args.template_dir.is_none() || args.template_dir.as_ref().unwrap().is_empty())
    {
        println!("No templates supplied");
        return;
    }

    let mut files = vec![];
    match &args.template {
        Some(f) => files.push(f.clone()),
        None => {}
    };

    if let Some(f) = args.template_dir.as_ref() {
        find_files(f, "json", &mut files)
    }

    let output_dir = Path::new(&args.output_dir);

    for f in &files {
        comid_template_to_cbor(f, output_dir);
    }
}

fn comid_display(args: &DisplaySubcommand) {
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
    let comid_cbor: ConciseMidTagCbor = match from_reader(data.as_slice()) {
        Ok(c) => c,
        Err(e) => {
            println!(
                "Unable to parse data read from {} as a CBOR-encoded CoMID with error {}",
                args.file_to_display, e
            );
            return;
        }
    };
    let comid_json: ConciseMidTag = match comid_cbor.try_into() {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to convert CBOR CoMID object to JSON CoMID object for {} with error: {}",
                args.file_to_display, e
            );
            return;
        }
    };

    let json = match serde_json::to_string_pretty(&comid_json) {
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

fn comid_template_to_cbor(template_file: &String, output_dir: &Path) {
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

    let comid_json: ConciseMidTag = match serde_json::from_str(&data) {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Unable to parse CoMID template from {} with error {}",
                template_file, e
            );
            return;
        }
    };

    let comid_cbor: ConciseMidTagCbor = match comid_json.try_into() {
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
    let mut output_pathbuf = output_path.join(template_filename.to_str().unwrap());
    output_pathbuf.set_extension("cbor");

    let mut output_file = File::create(output_pathbuf).unwrap();
    output_file
        .write_all(encoded_token.as_slice())
        .expect("Unable to write manifest file");
}
