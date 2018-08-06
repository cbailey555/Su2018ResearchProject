use failure::Error;
use sawtooth_sdk::signing;

use swth_cli_libv2::keymgmt::load_signing_key;
use swth_cli_libv2::errors::CliError;
use dmktp::auction::Bid;
use dmktp::wrapper::Wrapper;
use dmktp::address::Address;


pub fn pubkey_from_keyfile(x: Option<&str>) -> Result<String, Error> {
    let loaded = match load_signing_key(x) {
        Ok(v) => v,
        Err(e) => return Err(Error::from(CliError::CustomError { contents: format!("could not find keyfile from given -k flag value: {:?}\n", e)}))
    };

    let context = match signing::create_context("secp256k1") {
        Ok(v) => v,
        Err(e) => return Err(Error::from(CliError::CustomError { contents: format!("could not create secp256k1 context: {:?}\n", e)}))
    };

    match context.get_public_key(&loaded) {
        Ok(v) => return Ok(v.as_hex()),
        Err(e) => return Err(Error::from(CliError::CustomError { contents: format!("could not derive public key from private key: {:?}\n", e)}))

    }

} 

pub fn addr_from_keyfile(x: Option<&str>) -> Result<Address, Error> {
    let loaded = match load_signing_key(x) {
        Ok(v) => v,
        Err(e) => return Err(Error::from(CliError::CustomError { contents: format!("could not find keyfile from given -k flag value: {:?}\n", e)}))
    };

    let context = match signing::create_context("secp256k1") {
        Ok(v) => v,
        Err(e) => return Err(Error::from(CliError::CustomError { contents: format!("could create secp256k1 context: {:?}\n", e)}))
    };

    match context.get_public_key(&loaded) {
        Ok(v) => return Ok(Address::from_pubkey_string(&v.as_hex())),
        Err(e) => return Err(Error::from(CliError::CustomError { contents: format!("could not derive public key from private key: {:?}\n", e)}))
    }

}

pub fn mk_bid(_keyfile: Option<&str>, _serial: u64, _amt: u64) -> Result<Wrapper, Error> {
    let address = addr_from_keyfile(_keyfile)?;
    Ok(Wrapper::Ab(Bid::from_vals(address, _serial, _amt)))
}