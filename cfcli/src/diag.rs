//! CBOR diagnostic notation conversion commands.

use std::{
    fs,
    io::{self, Read},
};

use crate::{DiagCommand, DiagSubCommands, cbor_diag, diag_parse};

/// Dispatch diag subcommands.
pub fn diag_main(args: &DiagCommand) {
    match &args.command {
        DiagSubCommands::ToCbor(c) => diag_to_cbor(c),
        DiagSubCommands::ToDiag(c) => diag_to_diag(c),
    }
}

/// Convert CBOR diagnostic notation to binary CBOR.
fn diag_to_cbor(args: &crate::args::DiagToCborSubcommand) {
    let input = if args.input == "-" {
        let mut buf = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buf) {
            println!("Failed to read stdin: {}", e);
            return;
        }
        buf
    } else {
        match fs::read_to_string(&args.input) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to read {}: {}", args.input, e);
                return;
            }
        }
    };

    let value = match diag_parse::from_diag(&input) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to parse diagnostic notation: {}", e);
            return;
        }
    };

    let mut buf = Vec::new();
    if let Err(e) = ciborium::ser::into_writer(&value, &mut buf) {
        println!("Failed to encode CBOR: {}", e);
        return;
    }

    if let Err(e) = fs::write(&args.output, &buf) {
        println!("Failed to write {}: {}", args.output, e);
        return;
    }

    println!("Wrote {} bytes of CBOR to {}", buf.len(), args.output);
}

/// Convert binary CBOR to diagnostic notation.
fn diag_to_diag(args: &crate::args::DiagToDiagSubcommand) {
    cbor_diag::display_diag(&args.input);
}
