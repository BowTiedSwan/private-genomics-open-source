use crate::error::AppResult;
use keyring::Entry;

const KEYRING_SERVICE: &str = "com.morpheus.genomics";
const KEYRING_USER: &str = "morpheus-api-key";

pub fn save_api_key(key: &str) -> AppResult<()> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)?;
    entry.set_password(key)?;
    Ok(())
}

pub fn load_api_key() -> AppResult<Option<String>> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)?;
    match entry.get_password() {
        Ok(v) => Ok(Some(v)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn clear_api_key() -> AppResult<()> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
