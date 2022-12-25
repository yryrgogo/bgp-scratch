use tracing::{debug, info, instrument};

use crate::config::Config;
use crate::connection::Connection;
use crate::event::Event;
use crate::event_queue::EventQueue;
use crate::packets::message::Message;
use crate::state::State;

/// BGP の RFC で示されている実装方針
/// (https://datatracker.ietf.org/doc/html/rfc4271#section-8) では、
/// 1つの Peer を1つのイベント駆動ステートマシンとして実装しています。
/// Peer 構造体は RFC 内で示されている実装方針に従ったイベント駆動ステートマシンです。
#[derive(Debug)]
pub struct Peer {
    state: State,
    event_queue: EventQueue,
    tcp_connection: Option<Connection>,
    config: Config,
}

impl Peer {
    pub fn new(config: Config) -> Self {
        let state = State::Idle;
        let event_queue = EventQueue::new();
        Self {
            state,
            event_queue,
            tcp_connection: None,
            config,
        }
    }

    #[instrument]
    pub fn start(&mut self) {
        info!("peer is started.");
        self.event_queue.enqueue(Event::ManualStart);
    }

    #[instrument]
    pub async fn next(&mut self) {
        if let Some(event) = self.event_queue.dequeue() {
            info!("event is occured, event={:?}.", event);
            self.handle_event(event).await;
        }
    }

    async fn handle_event(&mut self, event: Event) {
        match &self.state {
            State::Idle => match event {
                Event::ManualStart => {
                    self.tcp_connection = Connection::connect(&self.config).await.ok();
                    if self.tcp_connection.is_some() {
                        self.event_queue.enqueue(Event::TcpConnectionConfirmed);
                    } else {
                        panic!(
                            ":TCP Connection の確立ができませんでした。{:?}",
                            self.config
                        )
                    }
                    self.state = State::Connect;
                }
                _ => {}
            },
            State::Connect => match event {
                Event::TcpConnectionConfirmed => {
                    self.tcp_connection
                        .as_mut()
                        .expect("TCP Connection が確立できていません。")
                        .send(Message::new_open(
                            self.config.local_as,
                            self.config.local_ip,
                        ))
                        .await;
                    self.state = State::OpenSent
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn peer_can_transition_to_connect_state() {
        let config: Config = "64512 127.0.0.1 65413 127.0.0.2 active".parse().unwrap();
        let mut peer = Peer::new(config);
        peer.start();

        tokio::spawn(async move {
            let remote_config = "64513 127.0.0.2 65412 127.0.0.1 passive".parse().unwrap();
            let mut remote_peer = Peer::new(remote_config);
            remote_peer.start();
            remote_peer.next().await;
        });

        // 先に remote_peer 側の処理が進むことを保証するための await
        tokio::time::sleep(Duration::from_secs(1)).await;

        peer.next().await;
        assert_eq!(peer.state, State::Connect);
    }

    #[tokio::test]
    async fn peer_can_transition_to_open_sent_state() {
        let config: Config = "64512 127.0.0.1 65413 127.0.0.2 active".parse().unwrap();
        let mut peer = Peer::new(config);
        peer.start();

        tokio::spawn(async move {
            let remote_config = "64513 127.0.0.2 65412 127.0.0.1 passive".parse().unwrap();
            let mut remote_peer = Peer::new(remote_config);
            remote_peer.start();
            remote_peer.next().await;
            remote_peer.next().await;
        });

        // 先に remote_peer 側の処理が進むことを進めたの sleep
        tokio::time::sleep(Duration::from_secs(1)).await;
        peer.next().await;
        peer.next().await;
        assert_eq!(peer.state, State::OpenSent);
    }
}
