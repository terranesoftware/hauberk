#![no_std]

/// An entity that holds a finite amount of resources.
pub struct Palatinate<R, T> {
    pub holding: R,
    pub demesne: R,
    pub palatine: T
}