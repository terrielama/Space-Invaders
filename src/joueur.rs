use bevy::{core::FixedTimestep, prelude::*};

use crate::{
	FromJoueur, Laser, Materials, Joueur, JoueurReadyFire, JoueurState, Speed, WinSize,
	JOUEUR_RESPAWN_DELAY, SCALE, TIME_STEP,
};

pub struct JoueurPlugin;

//implementation du joueur
impl Plugin for JoueurPlugin {
		fn build(&self, app: &mut AppBuilder) {
		app
			.insert_resource(JoueurState::default())
			.add_startup_stage(
				"game_setup_actors",
				SystemStage::single(joueur_spawn.system()),
			)
			.add_system(joueur_mouvement.system())
			.add_system(joueur_fire.system())
			.add_system(laser_mouvement.system())
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(0.5))
					.with_system(joueur_spawn.system()),
			);
	}
}

fn joueur_spawn(
	mut commands: Commands,
	materials: Res<Materials>,
	win_size: Res<WinSize>,
	time: Res<Time>,
	mut joueur_state: ResMut<JoueurState>,
) {
	let now = time.seconds_since_startup();
	let last_shot = joueur_state.last_shot;

	// sprite réapparition du joueur dans le jeu
	if !joueur_state.on && (last_shot == 0. || now > last_shot + JOUEUR_RESPAWN_DELAY) {
		let bottom = -win_size.h / 2.;
		commands
			.spawn_bundle(SpriteBundle {
				material: materials.joueur.clone(),
				transform: Transform {
					translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
					scale: Vec3::new(SCALE, SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Joueur)
			.insert(JoueurReadyFire(true))
			.insert(Speed::default());

		joueur_state.spawned();
	}
}

fn joueur_mouvement(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&Speed, &mut Transform), With<Joueur>>,
) {
	if let Ok((speed, mut transform)) = query.single_mut() {
		let dir = if keyboard_input.pressed(KeyCode::Left) {
			-1.
		} else if keyboard_input.pressed(KeyCode::Right) {
			1.
		} else {
			0.
		};
		transform.translation.x += dir * speed.0 * TIME_STEP;
	}
}

fn joueur_fire(
	mut commands: Commands,
	kb: Res<Input<KeyCode>>,
	materials: Res<Materials>,
	mut query: Query<(&Transform, &mut JoueurReadyFire), With<Joueur>>,
) {
	if let Ok((Joueur_tf, mut ready_fire)) = query.single_mut() {
		//clavier 
		if ready_fire.0 && kb.pressed(KeyCode::Space) {
			let x = Joueur_tf.translation.x;
			let y = Joueur_tf.translation.y;

			let mut spawn_lasers = |x_offset: f32| {
				commands
					.spawn_bundle(SpriteBundle {
						material: materials.joueur_laser.clone(),
						transform: Transform {
							translation: Vec3::new(x + x_offset, y + 15., 0.),
							scale: Vec3::new(SCALE, SCALE, 1.),
							..Default::default()
						},
						..Default::default()
					})
					.insert(Laser)
					.insert(FromJoueur)
					.insert(Speed::default());
			};

			let x_offset = 144.0 / 4.0 - 5.0;
			spawn_lasers(x_offset);
			spawn_lasers(-x_offset);

			ready_fire.0 = false;
		}

		if kb.just_released(KeyCode::Space) {
			ready_fire.0 = true;
		}
	}
}

fn laser_mouvement(
	mut commands: Commands,
	win_size: Res<WinSize>,
	mut query: Query<(Entity, &Speed, &mut Transform), (With<Laser>, With<FromJoueur>)>,
) {
	for (laser_entity, speed, mut laser_tf) in query.iter_mut() {
		let translation = &mut laser_tf.translation;
		translation.y += speed.0 * TIME_STEP;
		if translation.y > win_size.h {
			commands.entity(laser_entity).despawn();
		}
	}
}





















