use std::cmp::Ordering;

use clap::{
    builder::ValueParserFactory,
    error::{ContextKind, ContextValue, ErrorKind},
    Parser,
};

/// Port scanner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    /// Milliseconds for waiting connection
    #[arg(long, short = 't', default_value_t = 1500)]
    pub timeout: u16,
    /// A number of parallel scannings
    #[arg(long, short = 'j', default_value_t = 256)]
    pub jobs: u16,
    /// A list of comma separed ports to be scanned.
    #[arg(long, short='p', value_delimiter=',', default_value = "1-1024", value_parser = PortValueParser)]
    pub ports: Vec<PortValue>,
    /// CIDRs, IPs, or hosts to be scanned
    pub addresses: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum PortValue {
    One(u16),
    Range(u16, u16),
}

impl PortValue {
    pub fn values(&self) -> Vec<u16> {
        match self {
            PortValue::One(v) => vec![*v],
            PortValue::Range(start, end) => (*start..=*end).collect(),
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
impl clap::builder::TypedValueParser for PortValueParser {
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
                None => v.parse::<u16>().ok().map(PortValue::One),
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
