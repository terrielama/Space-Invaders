use std::f32::consts::PI;

use crate::{
	ActiveEnemies, Enemies, FromEnemies, Laser, Materials, Speed, WinSize, MAX_ENEMIES,
	MAX_FORMATION_MEMBERS, SCALE, TIME_STEP,
};
use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

pub struct EnemiesPlugin;


//placement au depart
// Component
#[derive(Default, Clone)]
struct Formation {
	start: (f32, f32),
	radius: (f32, f32),
	offset: (f32, f32),
	angle: f32,
	group_id: u32,
}











