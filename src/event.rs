/// BGP RFC 8.1
/// (https://datatracker.ietf.org/doc/html/rfc4271#section-8.1) で
/// 定義されている列挙型
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Event {
    ManualStart,
    // この実装では正常系しか実装しないため、TcpConnectionConfirmed で TcpCrAcked も兼ねる
    TcpConnectionConfirmed,
}
