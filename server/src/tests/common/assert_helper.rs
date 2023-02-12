use std::fmt::Debug;
use std::sync::Arc;
use packets::packets::{Packet, PacketZcSpriteChange2};
use packets::packets_parser::parse;
use crate::server::model::events::client_notification::{AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::global_config_service::GlobalConfigService;

pub fn task_queue_contains_event_at_tick<T: PartialEq + Debug>(task_queue: Arc<TasksQueue<T>>, expected_event: T, tick: usize) {
    let mut events = vec![];
    for _ in 0..=tick {
        events = task_queue.pop().unwrap_or_else(|| panic!("Expected task queue to contains events at tick {}", tick));
    }
    for event in events {
        if matches!(&event, expected_event) {
            assert!(event == expected_event, "Expected {:?} == {:?}", event, expected_event);
            return;
        }
    }
    assert!(false, "Task queue does not contains event {:?}", expected_event);
}

pub fn variance(expectation: u32, variance: usize) -> u32 {
    (variance as f32 / 100 as f32).round() as u32 * expectation
}

#[macro_export]
macro_rules! assert_eq_with_variance {
    ($variance:expr, $actual:expr, $expected:expr $(,)?) => {
        let _variance = ($variance as f32 / 100 as f32) * $expected as f32;
        assert!($actual as f32 - _variance <= $expected as f32 && $expected as f32 <= $actual as f32 + _variance);
    };
    ($variance:expr,$actual:expr, $expected:expr, $($arg:tt)+) => {
        let _variance = ($variance as f32 / 100 as f32) * $expected as f32;
        assert!($actual as f32 - _variance <= $expected as f32 && $expected as f32 <= $actual as f32 + _variance, $($arg)+);
    }
}

#[macro_export]
macro_rules! assert_task_queue_contains_event_at_tick {
    ($task_queue:expr, $expected_event:expr, $tick:expr $(,)?) => {
        task_queue_contains_event_at_tick($task_queue.clone(), $expected_event, $tick)
    }
}

pub struct NotificationExpectation<'a> {
    kind: NotificationExpectationKind,
    packets: Vec<SentPacket<'a>>,
    map_name: Option<&'a str>,
    char_id: Option<u32>,
    x: Option<u16>,
    y: Option<u16>,
}

pub struct SentPacket<'a> {
    id: &'a str,
    count: Option<usize>,
}

impl<'a> SentPacket<'a> {
    pub fn with_id(id: &'a str) -> Self {
        Self { id, count: None }
    }

    pub fn with_count(id: &'a str, count: usize) -> Self {
        Self { id, count: Some(count) }
    }

    pub fn id(&self) -> &'a str {
        self.id
    }

    pub fn count(&self) -> Option<usize> {
        self.count
    }
}

pub enum NotificationExpectationKind {
    Char,
    AreaFov,
    AreaMap,
}

impl<'a> NotificationExpectation<'a> {
    pub fn of_char(char_id: u32, packets: Vec<SentPacket<'a>>) -> Self {
        Self { kind: NotificationExpectationKind::Char, packets, map_name: None, char_id: Some(char_id), x: None, y: None }
    }
    pub fn of_map(map_name: &'a str, packets: Vec<SentPacket<'a>>) -> Self {
        Self { kind: NotificationExpectationKind::AreaMap, packets, map_name: Some(map_name), char_id: None, x: None, y: None }
    }
    pub fn of_fov(x: u16, y: u16, packets: Vec<SentPacket<'a>>) -> Self {
        Self { kind: NotificationExpectationKind::AreaFov, packets, map_name: None, char_id: None, x: Some(x), y: Some(y) }
    }
}

pub fn has_sent_notification(notifications: &Vec<Notification>, expectation: NotificationExpectation, packetver: u32) -> bool {
    notifications.iter().find(|sent_notification| {
        match expectation.kind {
            NotificationExpectationKind::Char => {
                if let Notification::Char(sent_char_notification) = sent_notification {
                    if expectation.char_id.is_some() && sent_char_notification.char_id() == expectation.char_id.unwrap() {
                        if contains_packet(&expectation, packetver, sent_char_notification.serialized_packet()) {
                            return true;
                        }
                    }
                }
                false
            }
            NotificationExpectationKind::AreaFov => {
                if let Notification::Area(sent_area_notification) = sent_notification {
                    if let AreaNotificationRangeType::Fov { x, y, .. } = sent_area_notification.range_type {
                        if expectation.x.is_some() && x == expectation.x.unwrap() && expectation.y.is_some() && y == expectation.y.unwrap() {
                            if contains_packet(&expectation, packetver, sent_area_notification.serialized_packet()) {
                                return true;
                            }
                        }
                    }
                }
                false
            }
            NotificationExpectationKind::AreaMap => {
                if let Notification::Area(sent_area_map_notification) = sent_notification {
                    if matches!(sent_area_map_notification.range_type, AreaNotificationRangeType::Map) {
                        if expectation.map_name.is_some() && sent_area_map_notification.map_name.as_str() == expectation.map_name.unwrap() {
                            if contains_packet(&expectation, packetver, sent_area_map_notification.serialized_packet()) {
                                return true;
                            }
                        }
                    }
                }
                false
            }
        }
    }).is_some()
}

pub fn has_sent_persistence_event(persistence_events: &Vec<PersistenceEvent>, persistence_event: PersistenceEvent) -> bool {
    let res = persistence_events.iter().find(|sent_persistence_event| if matches!(&persistence_event, sent_persistence_event) { persistence_event == **sent_persistence_event } else { false }).is_some();
    if !res {
        println!("Can't find {:?} among events below", persistence_event);
        persistence_events.iter().for_each(|e| println!("  {:?}", e));
    }
    res
}

#[macro_export]
macro_rules! assert_sent_packet_in_current_packetver {
    ($context:expr, $expectation:expr $(,)?) => {
        assert!(has_sent_notification($context.test_context.received_notification().lock().unwrap().as_ref(), $expectation, GlobalConfigService::instance().packetver()));
    }
}
#[macro_export]
macro_rules! assert_not_sent_packet_in_current_packetver {
    ($context:expr, $expectation:expr $(,)?) => {
        assert!(!has_sent_notification($context.test_context.received_notification().lock().unwrap().as_ref(), $expectation, GlobalConfigService::instance().packetver()));
    }
}
#[macro_export]
macro_rules! assert_sent_persistence_event {
    ($context:expr, $expectation:expr $(,)?) => {
        assert!(has_sent_persistence_event($context.test_context.received_persistence_events().lock().unwrap().as_ref(), $expectation));
    }
}

fn contains_packet(expectation: &NotificationExpectation, packetver: u32, packets: &Vec<u8>) -> bool {
    let packets = parse_packet(packets, packetver);
    for expectation_packet in expectation.packets.iter() {
        let mut i = 0;
        for packet in packets.iter() {
            if expectation_packet.id() == packet.id() {
                i += 1;
                if let Some(count) = expectation_packet.count {
                    if i == count {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }
    }
    false
}

fn parse_packet(packets: &[u8], packetver: u32) -> Vec<Box<dyn Packet>> {
    let mut packet = parse(packets, packetver);
    let mut parsed_packets = vec![];
    if packet.raw().len() < packets.len() {
        let mut offset = 0;
        while offset < packets.len() {
            packet = parse(&packets[offset..packets.len()], packetver);
            offset += packet.raw().len();
            parsed_packets.push(packet);
        }
    } else {
        parsed_packets.push(packet);
    }
    parsed_packets
}