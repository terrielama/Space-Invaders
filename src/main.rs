mod enemies;
mod joueur;


use std::collections::HashSet;

use bevy::{prelude::*, sprite::collide_aabb::collide};
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
const MAX_ENEMIESS: u32 = 4;
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
struct ActiveEnemiess(u32);



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
		.insert_resource(ActiveEnemiess(0))
		.add_plugins(DefaultPlugins)
		.add_plugin(JoueurPlugin)
		.add_plugin(EnemiesPlugin)
		.add_startup_system(setup.system())
		.add_system(joueur_laser_hit_enemies.system())
		.add_system(enemies_laser_hit_joueur.system())
		.add_system(explosion_to_spawn.system())
		.add_system(animate_explosion.system())
		.run();
}





fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut windows: ResMut<Windows>,) 
	
	{
	
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
}

fn joueur_laser_hit_enemies(
	mut commands: Commands,
	laser_query: Query<(Entity, &Transform, &Sprite), (With<Laser>, With<FromJoueur>)>,
	enemies_query: Query<(Entity, &Transform, &Sprite), With<Enemies>>,
	mut active_enemiess: ResMut<ActiveEnemiess>,
) {
	let mut enemiess_blasted: HashSet<Entity> = HashSet::new();

	for (laser_entity, laser_tf, laser_sprite) in laser_query.iter() {
		for (enemies_entity, enemies_tf, enemies_sprite) in enemies_query.iter() {
			let laser_scale = Vec2::from(laser_tf.scale);
			let enemies_scale = Vec2::from(enemies_tf.scale);
			let collision = collide(
				laser_tf.translation,
				laser_sprite.size * laser_scale,
				enemies_tf.translation,
				enemies_sprite.size * enemies_scale,
			);

			if let Some(_) = collision {
				if enemiess_blasted.get(&enemies_entity).is_none() {
					// suppr l'enemies
					commands.entity(enemies_entity).despawn();
					active_enemiess.0 -= 1;

					// apparition de l'explosion
					commands
						.spawn()
						.insert(ExplosionToSpawn(enemies_tf.translation.clone()));

					enemiess_blasted.insert(enemies_entity);
				}

				// disparition du laser
				commands.entity(laser_entity).despawn();
			}
		}
	}
}

	
fn enemies_laser_hit_joueur(
	mut commands: Commands,
	mut joueur_state: ResMut<JoueurState>,
	time: Res<Time>,
	laser_query: Query<(Entity, &Transform, &Sprite), (With<Laser>, With<FromEnemies>)>,
	joueur_query: Query<(Entity, &Transform, &Sprite), With<Joueur>>,
) {
	if let Ok((joueur_entity, joueur_tf, joueur_sprite)) = joueur_query.single() {
		let joueur_size = joueur_sprite.size * Vec2::from(joueur_tf.scale.abs());
		// pour chaque laser tiré par les enemies
		for (laser_entity, laser_tf, laser_sprite) in laser_query.iter() {
			let laser_size = laser_sprite.size * Vec2::from(laser_tf.scale.abs());
			//les collisions
			let collision = collide(
				laser_tf.translation,
				laser_size,
				joueur_tf.translation,
				joueur_size,
			);
			// traitement des  collisions
			if let Some(_) = collision {
				// disparition du joueur
				commands.entity(joueur_entity).despawn();
				joueur_state.shot(time.seconds_since_startup());
				//disparition du laser
				commands.entity(laser_entity).despawn();
				//utilisation de la fonction  ExplosionToSpawn 
				commands
					.spawn()
					.insert(ExplosionToSpawn(joueur_tf.translation.clone()));
			}
		}
	}
}	
	
	
fn explosion_to_spawn(
	mut commands: Commands,
	query: Query<(Entity, &ExplosionToSpawn)>,
	materials: Res<Materials>,
) {
	for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
		commands
			.spawn_bundle(SpriteSheetBundle {
				texture_atlas: materials.explosion.clone(),
				transform: Transform {
					translation: explosion_to_spawn.0,
					..Default::default()
				},
				..Default::default()
			})
			.insert(Explosion)
			.insert(Timer::from_seconds(0.05, true));

		commands.entity(explosion_spawn_entity).despawn();
	}
}	
	
fn animate_explosion(
	mut commands: Commands,
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut query: Query<
		(
			Entity,
			&mut Timer,
			&mut TextureAtlasSprite,
			&Handle<TextureAtlas>,
		),
		With<Explosion>,
	>,
) {
	for (entity, mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
		timer.tick(time.delta());
		if timer.finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index += 1;
			if sprite.index == texture_atlas.textures.len() as u32 {
				commands.entity(entity).despawn();
			}
		}
	}
}	




