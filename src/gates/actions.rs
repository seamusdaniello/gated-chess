use crate::gates::GateType;
use crate::board::Square;

pub fn set_gate(square: &mut Square, gate: GateType) {
    square.gate = Some(gate);
    square.animation_frame = Some(0);
    square.animation_direction = Some(1);
}

pub fn remove_gate(square: &mut Square) {
    square.gate = None;
    square.animation_frame = None;
    square.animation_direction = None;
}