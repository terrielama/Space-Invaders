mod enemies;
mod joueur;


use std::collections::HashSet;

use bevy::{prelude::*, sprite::collision_aabb::collision};
use enemies::EnemiesPlugin;
use joueur::JoueurPlugin;


//sprites
const JOUEUR_SPRITE: &str = "joueur.png";
const JOUEUR_LASER_SPRITE: &str = "laser1.png";
const ENEMIES_SPRITE: &str = "enemies.png";
const ENEMIES_LASER_SPRITE: &str = "laser2.png";
const EXPLOSION_SHEET: &str = "explosions.png";
const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const MAX_ENEMIES: u32 = 4;
const MAX_FORMATION_MEMBRES: u32 = 2;
const JOUEUR_RESPAWN_DELAY: f64 = 2.;


//couleur des elements du jeu
pub struct Materials {
	joueur: Handle<ColorMaterial>,
	joueur_laser: Handle<ColorMaterial>,
	enemies: Handle<ColorMaterial>,
	enemies_laser: Handle<ColorMaterial>,
	explosion: Handle<TextureAtlas>,
}

//
struct WinSize {
	#[allow(unused)]
	w: f32,
	h: f32,
}
struct ActiveEnemies(u32);



//etat du joueur
struct JoueurState {
	on: bool,
	last_shot: f64,
}
impl Default for JoueurState {
	fn default() -> Self {
		Self {
			on: false,
			last_shot: 0.,
		}
	}
}


impl JoueurState {
	fn shot(&mut self, time: f64) {
		self.on = false;
		self.last_shot = time;
	}
	fn spawned(&mut self) {
		self.on = true;
		self.last_shot = 0.
	}
}

// Components,ce qui compose le jeu
struct Laser;

struct Joueur;
struct JoueurReadyFire(bool);
struct FromJoueur;

struct Enemies;
struct FromEnemies;

struct Explosion;
struct ExplosionToSpawn(Vec3);

struct Speed(f32);
impl Default for Speed {
	fn default() -> Self {
		Self(500.)
	}
}


















