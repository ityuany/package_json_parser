use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum License {
    AGPL3Only,
    Apache20,
    BSD2Clause,
    BSD3Clause,
    BSL10,
    CC010,
    CDDL10,
    CDDL11,
    EPL10,
    EPL20,
    GPL20Only,
    GPL30Only,
    ISC,
    LGPL20Only,
    LGPL21Only,
    LGPL21OrLater,
    LGPL30Only,
    LGPL30OrLater,
    MIT,
    MPL20,
    MSPL,
    UnLicense(String),
}

impl Serialize for License {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            License::AGPL3Only => serializer.serialize_str("AGPL-3.0-only"),
            License::Apache20 => serializer.serialize_str("Apache-2.0"),
            License::BSD2Clause => serializer.serialize_str("BSD-2-Clause"),
            License::BSD3Clause => serializer.serialize_str("BSD-3-Clause"),
            License::BSL10 => serializer.serialize_str("BSL-1.0"),
            License::CC010 => serializer.serialize_str("CC0-1.0"),
            License::CDDL10 => serializer.serialize_str("CDDL-1.0"),
            License::CDDL11 => serializer.serialize_str("CDDL-1.1"),
            License::EPL10 => serializer.serialize_str("EPL-1.0"),
            License::EPL20 => serializer.serialize_str("EPL-2.0"),
            License::GPL20Only => serializer.serialize_str("GPL-2.0-only"),
            License::GPL30Only => serializer.serialize_str("GPL-3.0-only"),
            License::ISC => serializer.serialize_str("ISC"),
            License::LGPL20Only => serializer.serialize_str("LGPL-2.0-only"),
            License::LGPL21Only => serializer.serialize_str("LGPL-2.1-only"),
            License::LGPL21OrLater => serializer.serialize_str("LGPL-2.1-or-later"),
            License::LGPL30Only => serializer.serialize_str("LGPL-3.0-only"),
            License::LGPL30OrLater => serializer.serialize_str("LGPL-3.0-or-later"),
            License::MIT => serializer.serialize_str("MIT"),
            License::MPL20 => serializer.serialize_str("MPL-2.0"),
            License::MSPL => serializer.serialize_str("MSPL"),
            License::UnLicense(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for License {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "AGPL-3.0-only" {
            Ok(License::AGPL3Only)
        } else if s == "Apache-2.0" {
            Ok(License::Apache20)
        } else if s == "BSD-2-Clause" {
            Ok(License::BSD2Clause)
        } else if s == "BSD-3-Clause" {
            Ok(License::BSD3Clause)
        } else if s == "BSL-1.0" {
            Ok(License::BSL10)
        } else if s == "CC0-1.0" {
            Ok(License::CC010)
        } else if s == "CDDL-1.0" {
            Ok(License::CDDL10)
        } else if s == "CDDL-1.1" {
            Ok(License::CDDL11)
        } else if s == "EPL-1.0" {
            Ok(License::EPL10)
        } else if s == "EPL-2.0" {
            Ok(License::EPL20)
        } else if s == "GPL-2.0-only" {
            Ok(License::GPL20Only)
        } else if s == "GPL-3.0-only" {
            Ok(License::GPL30Only)
        } else if s == "ISC" {
            Ok(License::ISC)
        } else if s == "LGPL-2.0-only" {
            Ok(License::LGPL20Only)
        } else if s == "LGPL-2.1-only" {
            Ok(License::LGPL21Only)
        } else if s == "LGPL-2.1-or-later" {
            Ok(License::LGPL21OrLater)
        } else if s == "LGPL-3.0-only" {
            Ok(License::LGPL30Only)
        } else if s == "LGPL-3.0-or-later" {
            Ok(License::LGPL30OrLater)
        } else if s == "MIT" {
            Ok(License::MIT)
        } else if s == "MPL-2.0" {
            Ok(License::MPL20)
        } else if s == "MSPL" {
            Ok(License::MSPL)
        } else {
            Ok(License::UnLicense(s))
        }
    }
}
