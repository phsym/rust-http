use std::net::TcpStream;
use std::collections::HashMap;
use std::io::{BufReader, Error, ErrorKind, BufRead};
use std::str::FromStr;

macro_rules! option {
	($expr:expr, $msg:expr) => (match $expr {
		Some(val) => val,
		None => return Err(Error::new(ErrorKind::Other, $msg))
	})
}

pub struct HttpReply<'r> {
	pub code: u32,
	pub header: HashMap<String, String>,
	reader: BufReader<&'r TcpStream>
}

impl <'r> HttpReply<'r> {
	fn new(code: u32, header: HashMap<String, String>, reader: BufReader<&TcpStream>) -> HttpReply {
		return HttpReply{code: code, header: header, reader: reader};
	}
	
	pub fn parse(mut reader: BufReader<&TcpStream>) -> Result<HttpReply, Error> {
		let mut code: u32;
		let mut header = HashMap::new();
		{
			let mut line = String::new();
			try!(reader.read_line(&mut line));
			let mut splt = line.split(" ");
			splt.next();
			code = match u32::from_str(option!(splt.next(), "Cannot parse HTTP code")) {
				Ok(n) => n,
				Err(e) => return Err(Error::new(ErrorKind::Other, format!("Cannot parse HTTP code : {}", e)))
			};
			
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
		
		let reply = HttpReply::new(code, header, reader);
		return Ok(reply);
	}
	
	pub fn get_length(&self) -> Result<usize, Error> {
		return match self.header.get("Content-Length") {
			Some(s) => match usize::from_str(s) {
				Ok(i) => Ok(i),
				Err(e) => Err(Error::new(ErrorKind::Other, format!("Cannot parse number Content-Lentgh from header{}", e)))
			},
			None => Err(Error::new(ErrorKind::Other, "No Content-Length provided in header"))
		};
	}
	
	pub fn get_reader(&mut self) -> &mut BufReader<&'r TcpStream> {
		return &mut self.reader;
	}
}