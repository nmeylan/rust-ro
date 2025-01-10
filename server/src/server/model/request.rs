#![allow(dead_code)]

use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{SyncSender};

use packets::packets::Packet;
use configuration::configuration::Config;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::response::Response;
use crate::server::model::session::Session;

pub struct Request<'server: 'request, 'request> {
    session_id: Option<u32>,
    packet_ver: u32,
    configuration: &'server Config,
    packet: &'request dyn Packet,
    socket: Arc<RwLock<TcpStream>>,
    session: Option<Arc<Session>>,
    response_channel: SyncSender<Response>,
    client_notification_channel: SyncSender<Notification>,
}

impl<'server: 'request, 'request> Request<'server, 'request> {
    pub fn new(configuration: &'server Config, session_id: Option<u32>, packet_ver: u32, socket: Arc<RwLock<TcpStream>>, packet: &'request dyn Packet, response_sender: SyncSender<Response>, client_notification_channel: SyncSender<Notification>) -> Self {
        Self {
            session_id,
            packet_ver,
            configuration,
            packet,
            socket,
            session: None,
            response_channel: response_sender,
            client_notification_channel,
        }
    }

    pub fn packet(&self) -> &'request dyn Packet {
        self.packet
    }

    pub fn set_session(&mut self, session: Arc<Session>) {
        self.session = Some(session);
    }

    pub fn packet_ver(&self) -> u32 {
        self.packet_ver
    }

    pub fn socket(&self) -> Arc<RwLock<TcpStream>> {
        self.socket.clone()
    }

    pub fn session(&self) -> Arc<Session> {
        self.session.as_ref().expect("Expected session to not be null in RequestContext. Ensure session exists before calling this method").clone()
    }

    pub fn response_sender(&self) -> SyncSender<Response> {
        self.response_channel.clone()
    }

    pub fn client_notification_channel(&self) -> SyncSender<Notification> {
        self.client_notification_channel.clone()
    }

    pub fn configuration(&self) -> &'server Config {
        self.configuration
    }
}