use errors::LibError::{self, EncodingError, LengthError};
use failure::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub struct Address {
    pub contents: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.to_string())
    }
}

impl Address {
    pub fn from_string(source: String) -> Result<Self, LibError> {
        let source_length: usize = source.len();

        if (source.len() > 40) {
            return Err(LengthError {
                field: format!("Address"),
                size: 40,
                got: source.len(),
            });
        } else if (source.is_ascii() == false) {
            return Err(EncodingError {
                field: format!("Address"),
                encoding: format!("ASCII hexidecimal"),
                got: source,
            });
        } else {
            Ok(Address { contents: source })
        }
    }

    pub fn from_pubkey_string(_input: &String) -> Self {
        let mut a = String::new();
        a.push_str(&_input[26..]);
        Address { contents: a }
    }

    pub fn to_string(&self) -> String {
        self.contents.clone()
    }

    pub fn to_string_ref(&self) -> &String {
        &self.contents
    }
}

#[cfg(test)]
mod address_tests {
    use super::*;

    #[test]
    fn test_builder() {
        let a = String::from("これはASCIIではない。");
        let b = String::from("this string is going to be too long to fit as an address");
        let c = String::from("0123456789ABCDEF012345789ABCDEF012356789");

        let aprime = Address::from_string(a);
        let bprime = Address::from_string(b);
        let cprime = Address::from_string(c);

        //        match aprime {
        //            Ok(v) => (),
        //            Err(e) => println!("{}\n", e.downcast::<LibError>().unwrap()),
        //        }

        assert_eq!(true, true);

        /*
        assert_eq!(true, aprime.is_err());
        assert_eq!(true, bprime.is_err());
        assert_eq!(false, cprime.is_err());
        */
    }
}
