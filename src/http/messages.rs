use std::net::TcpStream;
use std::collections::HashMap;
use std::io::{BufReader, Error, ErrorKind, BufRead};
use std::str::FromStr;

/// A structure that represents an HTTP reply
///
/// It contains an already parsed header information, and offers
/// a `BufReader` to read reply content
pub struct HttpReply<'r> {
	pub version: String,
	pub code: u32,
	pub status: String,
	pub header: HashMap<String, String>,
	reader: BufReader<&'r TcpStream>
}

impl <'r> HttpReply<'r> {
	fn new(version: String, code: u32, status: String, header: HashMap<String, String>, reader: BufReader<&TcpStream>) -> HttpReply {
		return HttpReply{version: version, code: code, status: status, header: header, reader: reader};
	}
	
	/// Contruct a new HttpReply by parsing the input from `reader`
	/// # Examples
	/// ```no_run
	/// # extern crate rust_http;
	/// use std::net::TcpStream;
	/// use std::io::BufReader;
	/// use http::messages::HttpReply;
	/// let mut socket = try!(TcpStream::connect("host_address:port"));
	/// // Do some stuff with socket and assume an http reply is coming
	/// let reader = BufReader::new(&socket);
	/// let r = try!(HttpReply::parse(reader));
	/// ```
	pub fn parse(mut reader: BufReader<&TcpStream>) -> Result<HttpReply, Error> {
		let mut code: u32;
		let mut version: String;
		let mut status: String;
		let mut header = HashMap::new();
		{
			let mut line = String::new();
			try!(reader.read_line(&mut line));
			line = line.trim().to_string();
			let mut splt = line.split(" ");
			version = option!(splt.next(), "HTTP version not found").to_string();
			code = match u32::from_str(option!(splt.next(), "HTTP code not found")) {
				Ok(n) => n,
				Err(e) => return Err(Error::new(ErrorKind::Other, format!("Cannot parse HTTP code : {}", e)))
			};
			status = option!(splt.next(), "HTTP status not found").to_string();
			
			//Parse header
			let mut line = String::new();
			loop {
				try!(reader.read_line(&mut line));
				line = line.trim().to_string();
				if line.is_empty() {break;}
				{
					let mut splt = line.split(": ");
					header.insert(
						option!(splt.next(), "Cannot parse header").to_string(),
						option!(splt.next(), "Cannot parse header").to_string()
						);
				}
				line.clear();
			}
		}
		
		let reply = HttpReply::new(version, code, status, header, reader);
		return Ok(reply);
	}
	
	/// Get the `Content-Length` property from header
	pub fn get_length(&self) -> Result<usize, Error> {
		return match self.header.get("Content-Length") {
			Some(s) => match usize::from_str(s) {
				Ok(i) => Ok(i),
				Err(e) => Err(Error::new(ErrorKind::Other, format!("Cannot parse number Content-Lentgh from header : {}", e)))
			},
			None => Err(Error::new(ErrorKind::Other, "No Content-Length provided in header"))
		};
	}
	
	/// Return a `BufReader` to read the reply's content
	pub fn get_reader(&mut self) -> &mut BufReader<&'r TcpStream> {
		return &mut self.reader;
	}
}