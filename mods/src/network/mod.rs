pub fn connect() {}

pub mod server;

#[derive(Debug)]
pub enum TrafficLight {
  Red,
  Yellow,
  Green
}
