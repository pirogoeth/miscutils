use failure::Error;
use miscutils_core::Executable;
use openssl::ssl;
use structopt::StructOpt;
use std::net::TcpStream;

#[derive(Debug, StructOpt)]
#[structopt(name = "ssl-tester")]
/// `ssl-tester` is a utility that connects to a given target
/// and tests compatibility with a given set of ciphers, protocols, etc.
pub struct Command {
    /// Whether or not certificate verification should be enabled
    #[structopt(short, long, default_value = "peer", parse(try_from_str = try_parse_ssl_verify_mode))]
    tls_verify: ssl::SslVerifyMode,

    /// Depth of certificate verification
    #[structopt(long, default_value = "100")]
    tls_verify_depth: u32,

    /// Set the lowest supported protocol version
    #[structopt(long, parse(try_from_str = try_parse_proto_version))]
    tls_min_proto_version: Option<::std::option::Option<ssl::SslVersion>>,

    /// Set the highest supported protocol version
    #[structopt(long, parse(try_from_str = try_parse_proto_version))]
    tls_max_proto_version: Option<::std::option::Option<ssl::SslVersion>>,

    /// Set the highest and lowest supported protocol version
    #[structopt(long, parse(try_from_str = try_parse_proto_version))]
    tls_proto_version: Option<::std::option::Option<ssl::SslVersion>>,

    /// Disable the use of SNI
    #[structopt(long)]
    tls_disable_sni: bool,

    /// Disable the use of hostname verification
    #[structopt(long)]
    tls_disable_hostname_verification: bool,

    /// Set the domain to use for TLS SNI, if it is different from the host
    #[structopt(long)]
    tls_sni_domain: Option<String>,

    /// `host` is the remote host to connect to
    host: String,

    /// `port` is the remote port to connect to
    #[structopt(default_value = "443")]
    port: u16,
}

impl Executable for Command {
    type Error = Error;

    fn execute(&self) -> Result<(), Self::Error> {
        let mut builder = ssl::SslConnector::builder(ssl::SslMethod::tls())?;
        builder.set_verify(self.tls_verify);
        builder.set_verify_depth(self.tls_verify_depth);

        let mut tls_min_proto_version = self.tls_min_proto_version;
        let mut tls_max_proto_version = self.tls_max_proto_version;

        if self.tls_proto_version.is_some() {
            if tls_min_proto_version.is_some() {
                return Err(super::error::Error::ConflictingOptions("tls-proto-version", "tls-min-proto-version").into());
            } else if tls_max_proto_version.is_some() {
                return Err(super::error::Error::ConflictingOptions("tls-proto-version", "tls-max-proto-version").into());
            } else {
                if let Some(version) = self.tls_proto_version {
                    tls_max_proto_version = Some(version);
                    tls_min_proto_version = Some(version);
                } else {
                    tls_max_proto_version = None;
                    tls_min_proto_version = None;
                }
            }
        }

        if tls_min_proto_version.is_some() {
            if let Some(version) = tls_min_proto_version {
                builder.set_min_proto_version(version)?;
            } else {
                builder.set_min_proto_version(None)?;
            }
        }

        if tls_max_proto_version.is_some() {
            if let Some(version) = tls_max_proto_version {
                builder.set_max_proto_version(version)?;
            } else {
                builder.set_max_proto_version(None)?;
            }
        }

        let connector = builder.build();
        let mut ssl_configuration = connector.configure()?;

        ssl_configuration.set_use_server_name_indication(!self.tls_disable_sni);
        ssl_configuration.set_verify_hostname(!self.tls_disable_hostname_verification);

        let stream = TcpStream::connect(format!("{}:{}", self.host, self.port))?;
        let mut stream = connector.connect(&self.host, stream)?;
        stream.shutdown()?;

        Ok(())
    }
}

fn try_parse_proto_version(version: &str) -> Result<Option<ssl::SslVersion>, Error> {
    match version.to_lowercase().as_str() {
        "sslv3" => Ok(Some(ssl::SslVersion::SSL3)),
        "tlsv1" => Ok(Some(ssl::SslVersion::TLS1)),
        "tlsv1.1" => Ok(Some(ssl::SslVersion::TLS1_1)),
        "tlsv1.2" => Ok(Some(ssl::SslVersion::TLS1_2)),
        "tlsv1.3" => Ok(Some(ssl::SslVersion::TLS1_3)),
        "" => Ok(None),
        _ => Err(super::error::Error::UnknownProtocolVersion { version: version.to_string() }.into()),
    }
}

fn try_parse_ssl_verify_mode(mode: &str) -> Result<ssl::SslVerifyMode, Error> {
    match mode.to_lowercase().as_str() {
        "none" => Ok(ssl::SslVerifyMode::NONE),
        "peer" => Ok(ssl::SslVerifyMode::PEER),
        _ => Err(super::error::Error::UnknownSslVerifyMode { mode: mode.to_string() }.into())
    }
}