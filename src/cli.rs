use std::cmp::Ordering;

use clap::{
    builder::{TypedValueParser, ValueParserFactory},
    error::{ContextKind, ContextValue, ErrorKind},
    Parser,
};

use crate::ports;

/// Port scanner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    /// Maximum time in seconds to scan
    #[arg(long, short = 't', default_value_t = 3)]
    pub timeout: u16,
    /// Number of parallel port scanning
    #[arg(long, short = 'b', default_value_t = 4000)]
    pub batch_size: u16,
    /// Ports to be scanned e.g. 80,443,19-26
    #[arg(long, short='p', value_delimiter=',', default_value = "1-65535", value_parser = PortValueParser)]
    pub ports: Vec<PortValue>,
    /// CIDRs, IPs, or hosts to scan ports
    #[arg(required = true)]
    pub addresses: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum PortValue {
    One(u16),
    Range(u16, u16),
    Top(u16),
}

impl PortValue {
    pub fn values(&self) -> Vec<u16> {
        match self {
            PortValue::One(v) => vec![*v],
            PortValue::Range(start, end) => (*start..=*end).collect(),
            PortValue::Top(v) => ports::NAMP_TOP_PORTS
                .iter()
                .cloned()
                .take(*v as usize)
                .collect(),
        }
    }
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
        value
            .to_str()
            .and_then(|v| match v.split_once('-') {
                None => {
                    if let Some(n) = v.strip_prefix("top") {
                        n.parse::<u16>().ok().map(PortValue::Top)
                    } else {
                        v.parse::<u16>().ok().map(PortValue::One)
                    }
                }
                Some((x, y)) => match (x.parse::<u16>().ok(), y.parse::<u16>().ok()) {
                    (Some(x), Some(y)) => match x.cmp(&y) {
                        Ordering::Less => Some(PortValue::Range(x, y)),
                        Ordering::Equal => Some(PortValue::One(x)),
                        Ordering::Greater => None,
                    },
                    _ => None,
                },
            })
            .ok_or_else(|| {
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
