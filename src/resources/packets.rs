use bevy::ecs::system::Resource;
use std::vec;

pub struct Packet {
    data: Vec<u8>,
    clientIndex: usize,
}

#[derive(Resource)]
pub struct Packets {
    pub packets: Vec<Packet>,
}

impl Packets {
    pub fn new() -> Packets {
        return Packets { packets: vec![] };
    }

    pub fn addPacket(&mut self, data: Vec<u8>, clientIndex: usize) {
        self.packets.push(Packet { data, clientIndex });
    }
}
