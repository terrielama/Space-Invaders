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

//main

fn main() {
	App::build()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.insert_resource(WindowDescriptor {
			title: "Rust Invaders!".to_string(),
			width: 598.0,
			height: 676.0,
			..Default::default()
		})
		.insert_resource(ActiveEnemies(0))
		.add_plugins(DefaultPlugins)
		.add_plugin(JoueurPlugin)
		.add_plugin(EnemiesPlugin)
		.add_startup_system(setup.system())
		.add_system(joueur_laser_hit_enemies.system())
		.add_system(enemies_laser_hit_player.system())
		.add_system(explosion_to_spawn.system())
		.add_system(animate_explosion.system())
		.run();
}





fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut windows: ResMut<Windows>,
) {
	let window = windows.get_primary_mut().unwrap();

	// camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// ressources du  main 
	let texture_handle = asset_server.load(EXPLOSION_SHEET);
	let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 4);
	commands.insert_resource(Materials {
		joueur: materials.add(asset_server.load(JOUEUR_SPRITE).into()),
		joueur_laser: materials.add(asset_server.load(JOUEUR_LASER_SPRITE).into()),
		enemies: materials.add(asset_server.load(ENEMIES_SPRITE).into()),
		enemies_laser: materials.add(asset_server.load(ENEMIES_LASER_SPRITE).into()),
		explosion: texture_atlases.add(texture_atlas),
	});
	commands.insert_resource(WinSize {
		w: window.width(),
		h: window.height(),
	});

	// position de la fenetre (window)








