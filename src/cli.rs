use clap::{
    builder::{TypedValueParser, ValueParserFactory},
    error::{ContextKind, ContextValue, ErrorKind},
    Parser,
};

use crate::ports::PortValue;

/// Port scanner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    /// Ports to be scanned e.g. 22,80-443,top100
    #[arg(long, short='p', value_delimiter=',', value_parser = PortValueParser)]
    pub ports: Vec<PortValue>,
    /// Maximum time in milliseconds to scan
    #[arg(long, short = 't')]
    pub timeout: Option<u16>,
    /// Number of concurrent port scanning
    #[arg(long, short = 'c')]
    pub concurrency: Option<u16>,

    /// CIDRs, IPs, or hosts to scan ports
    pub addresses: Vec<String>,
}

impl ValueParserFactory for PortValue {
    type Parser = PortValueParser;
    fn value_parser() -> Self::Parser {
        PortValueParser
    }
}

#[derive(Clone, Debug)]
pub struct PortValueParser;
impl TypedValueParser for PortValueParser {
    type Value = PortValue;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        value.to_str().and_then(|v| v.parse().ok()).ok_or_else(|| {
            let mut err = clap::Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(
                    ContextKind::InvalidArg,
                    ContextValue::String(arg.to_string()),
                );
            }
            value.to_str().and_then(|v| {
                err.insert(
                    ContextKind::InvalidValue,
                    ContextValue::String(v.to_string()),
                )
            });
            err
        })
    }
}
