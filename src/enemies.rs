use std::f32::consts::PI;

use crate::{
	ActiveEnemies, Enemies, FromEnemies, Laser, Materials, Speed, WinSize, MAX_ENEMIES,
	MAX_FORMATION_MEMBERS, SCALE, TIME_STEP,
};
use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

pub struct EnemiesPlugin;
