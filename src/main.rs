mod utils;

use bip39::Mnemonic;
use clap::{Args, Parser};
use rand_core::{OsRng, RngCore};
use utils::lagrange::lagrange_interpolation;

#[derive(Debug, thiserror::Error)]
pub enum CmdError {
    #[error("Threshold setting error")]
    InvalidThreshold,

    #[error("Share cannot be ZERO")]
    NoneZero,
}

#[derive(Debug, Parser)]
enum Cmd {
    Generate,
    Split(TofN),
    Recover,
}

#[derive(Args, Debug)]
struct TofN {
    #[clap(long, short)]
    threshold: u8,

    #[clap(long, short)]
    number: u8,
}

fn generate() -> Result<(), CmdError> {
    let mut entropy = [0u8; 32]; // only supports 32 bites
    OsRng.fill_bytes(&mut entropy);
    let seed = Mnemonic::from_entropy(&entropy).expect("fail to generate");
    println!("{}", seed);
    Ok(())
}

fn split(t: u8, n: u8) -> Result<(), CmdError> {
    if n == 0 {
        return Err(CmdError::NoneZero);
    }
    if n < t {
        return Err(CmdError::InvalidThreshold);
    }

    let share = lagrange_interpolation();
    Ok(())
}

fn main() {
    let r = match Cmd::parse() {
        Cmd::Generate => generate(),
        Cmd::Split(args) => split(args.threshold, args.number),
        Cmd::Recover => unimplemented!(),
    };
    if let Err(e) = r {
        eprintln!("{:#}", e);
    }
}
