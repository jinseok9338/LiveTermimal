#[derive(Debug)]
pub enum BufferEncoding {
    Ascii,
    Utf8,
    Utf16Le,
    Ucs2,
    Base64,
    Base64Url,
    Latin1,
    Binary,
    Hex,
}
