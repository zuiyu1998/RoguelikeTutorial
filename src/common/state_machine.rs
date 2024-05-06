use bevy::prelude::*;

#[derive(Debug, Component, Clone)]
#[component(storage = "SparseSet")]
pub struct Idle;

#[derive(Debug, Component, Clone)]
#[component(storage = "SparseSet")]
pub struct Follow;
