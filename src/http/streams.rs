//! Http I/O streams definitions

use std::net::{ToSocketAddrs, TcpStream};
use std::io::{Error, Read, Write};
#[cfg(feature="ssl")]
use std::io::ErrorKind;

/// Represent a type that can be opened (ie connected) to a remote `SocketAddress`
pub trait Open: Sized {
	/// Create a new Instance of `Self` connected to `addr`
	fn open<A: ToSocketAddrs>(addr: A) -> Result<Self, Error>;
}

/// A trait representing an openable read/write stream
pub trait Stream: Read+Write+Open {}

/// HttpStream for unsecured HTTP Input/Output
pub type HttpStream = TcpStream;
impl Stream for HttpStream{}

impl Open for HttpStream {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<HttpStream, Error> {
		return Ok(try!(TcpStream::connect(addr)));
	}
}

#[cfg(feature="ssl")]
use openssl::ssl::{SslContext, SslStream, SslMethod};

/// HttpsStream for secured HTTPS Input/Output. Only available if "ssl" feature is enabled
#[cfg(feature="ssl")]
pub type HttpsStream = SslStream<TcpStream>;
#[cfg(feature="ssl")]
impl Stream for HttpsStream{}

#[cfg(feature="ssl")]
impl Open for HttpsStream {
	fn open<A: ToSocketAddrs>(addr: A) -> Result<HttpsStream, Error> {
		let ctx = match SslContext::new(SslMethod::Tlsv1) {
			Ok(c) => c,
			Err(e) => return Err(Error::new(ErrorKind::Other, format!("Cannot create SSL context : {}", e)))
		};
		let sock = try!(TcpStream::connect(addr));
		let stream = match SslStream::new(&ctx, sock) {
			Ok(s) => s,
			Err(e) => return Err(Error::new(ErrorKind::Other, format!("Cannot create SSL stream : {}", e)))
		};
		return Ok(stream);
	}
}
