// =======================================================
// Project: GatedChess
// File: pieces/mod.rs
// Description: Defines pieces used in-game.
// Author: Seamus Daniello
// Created: 2025-11-07
// Last Modified: 2025-11-30
// License: MIT
// =======================================================

pub mod piece_animations;
pub mod piece_fsm;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceForm {
    Base,
    Corrupted,
    Ascended,
    Fragmented,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PieceState {
    Idle,
    Selected,
    Moving,
    Morphing(PieceForm),
    Attacking,
    Dead,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PieceId(pub u32);

static NEXT_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

impl PieceId {
    pub fn new() -> Self {
        PieceId(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub id: PieceId,
    pub kind: PieceType,
    pub color: Color,
    pub form: PieceForm,
    pub state: PieceState,
    pub has_moved: Option<bool>,
}

impl Piece {
    pub fn new(kind: PieceType, color: Color) -> Piece {
        Piece {
            id: PieceId::new(),
            kind,
            color,
            form: PieceForm::Base,
            state: PieceState::Idle,
            has_moved: Some(false), 
        }
    }

    pub fn get_id(&self) -> PieceId {
        return self.id;
    }

    pub fn get_kind(&self) -> PieceType {
        return self.kind;
    }

    pub fn get_color(&self) -> Color {
        return self.color;
    }

    pub fn get_form(&self) -> PieceForm {
        return self.form;
    }

    pub fn get_state(&self) -> PieceState {
        return self.state;
    }

    pub fn get_has_moved(&self) -> Option<bool> {
        return self.has_moved;
    }
}