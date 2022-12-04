mod server;

use crate::server::web_socket;

fn main() {
	web_socket::listen(&String::from("localhost:4000"));
}