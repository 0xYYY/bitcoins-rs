/// We treat the xpub/ypub/zpub convention as a hint regarding address type. Users are free to
/// follow or ignore these hints.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Hint {
    /// Standard Bip32 hint
    Legacy,
    /// Bip32 + Bip49 hint for Witness-via-P2SH
    Compatibility,
    /// Bip32 + Bip84 hint for Native SegWit
    SegWit,
}

/// A 4-byte key fingerprint
#[derive(Eq, PartialEq, Clone, Copy)]
pub struct KeyFingerprint(pub [u8; 4]);

impl From<[u8; 4]> for KeyFingerprint {
    fn from(v: [u8; 4]) -> Self {
        Self(v)
    }
}

impl KeyFingerprint {
    /// Determines if the slice represents the same key fingerprint
    pub fn eq_slice(self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl std::fmt::Debug for KeyFingerprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("KeyFingerprint {:x?}", self.0))
    }
}

/// A 32-byte chain code
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct ChainCode(pub [u8; 32]);

impl From<[u8; 32]> for ChainCode {
    fn from(v: [u8; 32]) -> Self {
        Self(v)
    }
}

/// Info associated with an extended key
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XKeyInfo {
    /// The key depth in the HD tree
    pub depth: u8,
    /// The 4-byte Fingerprint of the parent
    pub parent: KeyFingerprint,
    /// The 4-byte derivation index of the key. If the most-significant byte is set, this key is
    /// hardened
    pub index: u32,
    /// The 32-byte chain code used to generate child keys
    pub chain_code: ChainCode,
    /// The key's stanadard output type preference
    pub hint: Hint,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fingerprint_slice_equality() {
        let print = [0; 4];
        let k: KeyFingerprint = print.into();
        assert!(k.eq_slice(&print[..]));
        println!("{:?}", &k);
    }
}
