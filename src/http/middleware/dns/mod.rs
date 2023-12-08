mod error;
mod layer;
mod resolver;
mod service;

pub use error::{DnsError, DnsResult};
pub use layer::DnsLayer;
pub use resolver::{DefaultDnsResolver, DnsResolver, NoDnsResolver, ResolvedSocketAddr};
pub use service::DnsService;
