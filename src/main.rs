use bip39::Mnemonic;
use clap::{Args, Parser};
use num_bigint::BigInt;
use rand_core::{OsRng, RngCore};
use std::io;
use tiny_ss::SS;

#[derive(Debug, thiserror::Error)]
pub enum CmdError {
    #[error("Threshold setting error")]
    InvalidThreshold,

    #[error("Share cannot be ZERO")]
    NoneZero,

    #[error("Io Error : {}", _0)]
    IoError(io::Error),

    #[error("Mnemonic Error : {}", _0)]
    MnemonicError(bip39::Error),
}

impl From<io::Error> for CmdError {
    fn from(err: io::Error) -> CmdError {
        CmdError::IoError(err)
    }
}

impl From<bip39::Error> for CmdError {
    fn from(err: bip39::Error) -> CmdError {
        CmdError::MnemonicError(err)
    }
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
    threshold: usize,

    #[clap(long, short)]
    number: usize,
}

fn generate() -> Result<(), CmdError> {
    let mut entropy = [0u8; 16]; // only supports 16 bites
    OsRng.fill_bytes(&mut entropy);
    let seed = Mnemonic::from_entropy(&entropy).expect("fail to generate");
    println!("{}", seed);
    Ok(())
}

fn split(t: usize, n: usize) -> Result<(), CmdError> {
    if n == 0 {
        return Err(CmdError::NoneZero);
    }
    if n < t {
        return Err(CmdError::InvalidThreshold);
    }

    println!("Seed Phrase:");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let mnemonic = bip39::Mnemonic::parse(&buf)?;
    let entropy = mnemonic.to_entropy();
    let r = entropy.iter().map(|e| *e as u32).collect::<Vec<_>>();
    let s = BigInt::from_slice(num_bigint::Sign::Plus, &r);
    //dbg!(&s);
    let p = BigInt::from(u64::MAX - 1);
    let ss = SS { t, n, p };
    let shares = ss.split(s);
    dbg!(&shares);
    let r = ss.recover(&shares[0..ss.t as usize]);
    println!("{}", r);
    //    assert_eq!(BigInt::from(10), ss.recover(&shares[0..ss.t as usize]));
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
