use std::ops::{Index, IndexMut};

use crate::types::nrof::NrOf;

#[derive(Clone, Copy, Debug)]
pub struct EverySide<T>([T; NrOf::SIDES]);

impl<T> EverySide<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self([value; NrOf::SIDES])
    }
}

impl<T> Index<Side> for EverySide<T> {
    type Output = T;
    fn index(&self, side: Side) -> &Self::Output {
        &self.0[side as usize]
    }
}

impl<T> IndexMut<Side> for EverySide<T> {
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    White = 0,
    Black = 1,
}

impl Side {
    pub fn other(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub fn flip_mut(&mut self) {
        *self = self.other();
    }
}

impl From<usize> for Side {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::White,
            1 => Self::Black,
            _ => panic!("Invalid value for Side: {}", value),
        }
    }
}
