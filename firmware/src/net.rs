use embassy_net::TcpSocket;
use format::{DeviceToServer, ServerToDevice};
use smoltcp::wire::IpEndpoint;
use smoltcp::Error as TcpError;

use self::state::{ConnectionState, Disconnected, Uninit, Connected};
use core::marker::PhantomData;

pub struct ServerConnection<'a, S: ConnectionState = Uninit> {
    socket: TcpSocket<'a>,
    _marker: PhantomData<S>,
}

impl<'a> ServerConnection<'a, Uninit> {
    pub fn new(rx_buffer: &'a mut [u8], tx_buffer: &'a mut [u8]) -> ServerConnection<'a, Disconnected> {
        let mut socket = TcpSocket::new(rx_buffer, tx_buffer);
        socket.set_timeout(Some(embassy_net::SmolDuration::from_secs(10)));

        ServerConnection {
            socket,
            _marker: PhantomData,
        }
    }
}

impl<'a> ServerConnection<'a, Disconnected> {
    pub async fn connect<T: Into<IpEndpoint>>(mut self, remote_endpoint: T) -> Result<ServerConnection<'a, Connected>, TcpError> {
        self.socket.connect(remote_endpoint).await?;
        Ok(ServerConnection {
            socket:  self.socket,
            _marker: PhantomData
        })
    }
}

impl<'a> ServerConnection<'a, Connected> {
    pub async fn send_message(&mut self, msg: DeviceToServer) -> Result<(), TcpError> {
        todo!();
    }

    pub async fn read_message(&mut self, msg: DeviceToServer) -> Result<ServerToDevice, TcpError> {
        todo!();
    }
}

mod state {
    pub trait ConnectionState {}

    pub struct Uninit;

    impl ConnectionState for Uninit {}

    pub struct Connected;

    impl ConnectionState for Connected {}
    pub struct Disconnected;

    impl ConnectionState for Disconnected {}
}
