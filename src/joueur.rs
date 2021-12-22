use bevy::{core::FixedTimestep, prelude::*};

use crate::{
	FromJoueur, Laser, Materials, Joueur, JoueurReadyFire, JoueurState, Speed, WinSize,
	JOUEUR_RESPAWN_DELAY, SCALE, TIME_STEP,
};

pub struct JoueurPlugin;

//implementation du joueur
impl Plugin for PlayerPlugin {
		fn build(&self, app: &mut AppBuilder) {
		app
			.insert_resource(JoueurState::default())
			.add_startup_stage(
				"game_setup_actors",
				SystemStage::single(pjoueur_spawn.system()),
			)
			.add_system(oueur_mouvement.system())
			.add_system(oueur_fire.system())
			.add_system(laser_mouvement.system())
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(0.5))
					.with_system(player_spawn.system()),
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
			.insert(Player)
			.insert(PlayerReadyFire(true))
			.insert(Speed::default());

		joueur_state.spawned();
	}
}


