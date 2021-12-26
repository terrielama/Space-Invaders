use std::f32::consts::PI;

use crate::{
	ActiveEnemiess, Enemies, FromEnemies, Laser, Materials, Speed, WinSize, MAX_ENEMIESS,
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

// Resource
#[derive(Default)]
struct FormationMaker {
	group_seq: u32,
	current_formation: Option<Formation>,
	current_formation_membres: u32,
}

impl FormationMaker {
	fn make(&mut self, win_size: &WinSize) -> Formation {
		match (
			&self.current_formation,
			self.current_formation_membres >= MAX_FORMATION_MEMBRES,
		) {
			// si la  formation ou la prochaine formation  est pleine
			(None, _) | (_, true) => {
				// le depart
				let mut rng = thread_rng();
				let h_span = win_size.h / 2. - 100.;
				let w_span = win_size.w / 4.;
				let x = if rng.gen::<bool>() {
					win_size.w
				} else {
					-win_size.w
				};
				let y = rng.gen_range(-h_span..h_span) as f32;
				let start = (x, y);

				
				let offset = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));
				let radius = (rng.gen_range(80.0..150.), 100.);
				let angle: f32 = (y - offset.0).atan2(x - offset.1);

				// creation de la nouvelle formation
				self.group_seq += 1;
				let group_id = self.group_seq;
				let formation = Formation {
					start,
					offset,
					radius,
					angle,
					group_id,
				};

				// close, set, and return
				self.current_formation = Some(formation.clone());
				self.current_formation_membres = 1;
				formation
			}
			// if still within the formation count
			(Some(tmpl), false) => {
				self.current_formation_membres += 1;
				tmpl.clone()
			}
		}
	}
}
//  Formation

impl Plugin for EnemiesPlugin {
	fn build(&self, app: &mut bevy::prelude::AppBuilder) {
		app
			.insert_resource(FormationMaker::default())
			.add_system(enemies_laser_movement.system())
			.add_system(enemies_movement.system())
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(1.0))
					.with_system(enemy_spawn.system()),
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(0.9))
					.with_system(enemies_fire.system()),
			);
	}
}


fn enemies_spawn(
	mut commands: Commands,
	mut active_enemiess: ResMut<ActiveEnemiess>,
	mut formation_maker: ResMut<FormationMaker>,
	win_size: Res<WinSize>,
	materials: Res<Materials>,
) {
	if active_enemiess.0 < MAX_ENEMIESS {
		
		let formation = formation_maker.make(&win_size);
		let (x, y) = formation.start;

		//appariton des enemies
		commands
			.spawn_bundle(SpriteBundle {
				material: materials.enemies.clone(),
				transform: Transform {
					translation: Vec3::new(x, y, 10.0),
					scale: Vec3::new(SCALE, SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Enemies)
			.insert(Speed::default())
			.insert(formation);

		active_enemiess.0 += 1;
	}
}








