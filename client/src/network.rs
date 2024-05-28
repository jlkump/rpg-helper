use std::net::TcpStream;

use crate::data::Game;

pub struct ClientInstance<'a> {
    client_id: i32,
    server_stream: Option<TcpStream>, // For commands coming to the server and sending requests to the server, None when not connected to a server. Allows for re-connection to server when disconnected
    authority_instance: Option<AuthorityInstance>, // Only exists when the user is hosting from the desktop application as the server. On the web-client, the user hosts a game as GM in which the web-server spins up an authority instance.
    active_game: Option<Game<'a>> // None when no game is active
}

struct ClientHandle {
    client_id: i32,
    client_stream: TcpStream
}

impl ClientHandle {
    pub fn set_active_game(&mut self, game_id: i32) {
        // Send request to client, which does the following:
        //      Save data changes
        //      Load active game data
        //           If the game data is not known by client, request from authority
        //      Prompt Character Selection or Character Creation if no usable characters
    }

    pub fn update_shared_data(&mut self) {
        // Todo
    }
}

struct ServerHandle {
    sever_stream: TcpStream,
}

pub struct AuthorityInstance {
    connected_clients: Vec<ClientHandle>
}