mod enemies;
mod joueur;


//sprites
const JOUEUR_SPRITE: &str = "joueur.png";
const JOUEUR_LASER_SPRITE: &str = "laser1.png";
const ENEMIES_SPRITE: &str = "enemies.png";
const ENEMIES_LASER_SPRITE: &str = "laser2.png";
const EXPLOSION_SHEET: &str = "explosions.png";
const SCALE: f32 = 0.5;
const TIME_STEP: f32 = 1. / 60.;
const MAX_ENEMIES: u32 = 4;
const MAX_FORMATION_MEMBERS: u32 = 2;
const JOUEUR_RESPAWN_DELAY: f64 = 2.;




