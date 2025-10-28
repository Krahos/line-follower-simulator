use bevy::{math::Vec2, text::cosmic_text::Angle};

use crate::{
    TrackId,
    track::{SegmentTransform, Track, TrackSegment},
    utils::Side,
};

fn build_simple_track() -> Track {
    Track::new(
        v2(5.0, 6.5),
        SegmentTransform::new(v2(0.5, -2.3), deg(0.0)),
        vec![
            start(),
            straight(2.0),
            t90(RIGHT, 0.5),
            turn(Angle::from_degrees(120.0), LEFT, 1.0),
            t90(LEFT, 1.0),
            turn(Angle::from_degrees(60.0), RIGHT, 2.0),
            end(),
        ],
    )
}

pub fn build_track(id: TrackId) -> Track {
    match id {
        TrackId::Line => unimplemented!(),
        TrackId::Angle => unimplemented!(),
        TrackId::Turn => unimplemented!(),
        TrackId::Simple => build_simple_track(),
        TrackId::Race => unimplemented!(),
    }
}

fn v2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

fn deg(degrees: f32) -> Angle {
    Angle::from_degrees(degrees)
}

fn start() -> TrackSegment {
    TrackSegment::start()
}

fn end() -> TrackSegment {
    TrackSegment::end()
}

fn straight(length: f32) -> TrackSegment {
    TrackSegment::straight(length)
}

fn t90(side: Side, radius: f32) -> TrackSegment {
    TrackSegment::ninety_deg_turn(radius, side)
}

fn turn(angle: Angle, side: Side, radius: f32) -> TrackSegment {
    TrackSegment::cyrcle_turn(radius, angle, side)
}

const LEFT: Side = Side::Left;
const RIGHT: Side = Side::Right;
