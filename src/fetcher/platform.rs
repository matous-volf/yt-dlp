use derive_more::Display;

/// Represents the operating system where the program is running.
#[derive(Clone, Debug, Display)]
pub enum Platform {
    #[display("Windows")]
    Windows,
    #[display("Linux")]
    Linux,
    #[display("MacOS")]
    Mac,

    #[display("Unknown: {}", _0)]
    Unknown(String),
}

/// Represents the architecture of the CPU where the program is running.
#[derive(Clone, Debug, Display)]
pub enum Architecture {
    #[display("x64")]
    X64,
    #[display("x86")]
    X86,
    #[display("armv7l")]
    Armv7l,
    #[display("aarch64")]
    Aarch64,

    #[display("Unknown: {}", _0)]
    Unknown(String),
}

impl Platform {
    /// Detects the current platform where the program is running.
    pub fn detect() -> Self {
        let os = std::env::consts::OS;

        match os {
            "windows" => Platform::Windows,
            "linux" => Platform::Linux,
            "macos" => Platform::Mac,
            _ => Platform::Unknown(os.to_string()),
        }
    }
}

impl Architecture {
    /// Detects the current architecture of the CPU where the program is running.
    pub fn detect() -> Self {
        let arch = std::env::consts::ARCH;

        match arch {
            "x86_64" => Architecture::X64,
            "x86" => Architecture::X86,
            "armv7l" => Architecture::Armv7l,
            "aarch64" => Architecture::Aarch64,
            _ => Architecture::Unknown(arch.to_string()),
        }
    }
}
