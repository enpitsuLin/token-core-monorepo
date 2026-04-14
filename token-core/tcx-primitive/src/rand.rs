use bip39::{Language, Mnemonic, MnemonicType};

pub fn generate_mnemonic() -> String {
    Mnemonic::new(MnemonicType::Words12, Language::English).to_string()
}

pub fn mnemonic_from_entropy(entropy: &[u8]) -> anyhow::Result<String> {
    let mnemonic = Mnemonic::from_entropy(entropy, Language::English)?;
    Ok(mnemonic.to_string())
}
