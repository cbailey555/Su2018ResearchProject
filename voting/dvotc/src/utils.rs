use swth_cli_libv2::errors::CliError;
use sawtooth_sdk::signing;
use swth_cli_libv2::keymgmt::load_signing_key;
use failure::Error;

pub fn parse_rb(x: &str) -> Vec<u32> {
    x.chars().into_iter()
             .filter(|x| x.is_ascii_digit())
             .map(|x| x.to_digit(10)
             .unwrap())
             .collect()
}

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




#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_parser() {
        let a = parse_rb("[1, 2, 3, 4, 5]");
        println!("a: {:?}\n", a);
        assert_eq!(vec![1, 2, 3, 4, 5], a);
    }
}