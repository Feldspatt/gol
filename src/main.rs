mod fly_cam;

use rand::prelude::*;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use crate::fly_cam::PlayerPlugin;

struct SetupScene;

const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);

struct WorldSize {
    x: i32,
    y: i32,
}

const WORLD_SIZE: WorldSize = WorldSize { x: 160, y: 160 };

impl Plugin for SetupScene {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PlayerPlugin)
            .add_startup_system(setup_light);
            // .add_startup_system(setup_plane);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GameOfLifePlugin)
        .add_plugin(SetupScene)
        .run();
}

fn setup_light(
    mut commands: Commands,
) {
    commands.insert_resource(AmbientLight {
        color: Color::ANTIQUE_WHITE,
        brightness: 2.0,
    });
}

struct GameOfLifePlugin;


const SIM_SPEED: f64 = 0.1;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_game_of_life)
            .add_system_set(SystemSet::new()
                .with_run_criteria(FixedTimestep::step(SIM_SPEED))
                .with_system(set_life_status)
                .with_system(update_game_of_life)
            );
    }
}

#[derive(Component)]
struct LifeStatus{
    current_status: bool,
}



fn setup_game_of_life(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let white_material = materials.add(StandardMaterial {
        base_color: WHITE,
        ..Default::default()
    });

    for x in 0..WORLD_SIZE.x {
        for y in 0..WORLD_SIZE.y {
            let random_number: f32 = rng.gen();

            let life_status: LifeStatus;
            if random_number > 0.5 {
                life_status = LifeStatus { current_status: true};
            } else {
                life_status = LifeStatus { current_status: false};
            }

            commands
                .spawn_bundle(PbrBundle {
                    mesh : meshes.add(Mesh::from(shape::Cube { size: 0.9 })),
                    material: white_material.clone(),
                    transform: Transform::from_xyz(x as f32, 0.0, y as f32),
                    visibility: Visibility {
                        is_visible: life_status.current_status,
                    },
                    ..Default::default()
                })
                .insert(life_status);
        }
    }

}

fn update_game_of_life(
    mut query: Query<(&LifeStatus, &mut Visibility)>
) {
    for (life_status, mut visibility) in query.iter_mut() {
        visibility.is_visible = life_status.current_status;
    }
}


fn set_life_status(
    mut query: Query<(&mut LifeStatus, &Transform)>
)  {

    let mut new_life_statuses: Vec<bool> = Vec::new();
    
    let alive_coords : Vec<(i32, i32)> = query.iter_mut()
        .filter(|(life_status, _)| life_status.current_status)
        .map(|(_, transform)| (transform.translation.x as i32, transform.translation.z as i32))
        .collect();
    
    for item in query.iter() {

        let ls = item.0;
        let t = item.1;

        let x = t.translation.x as i32;
        let z = t.translation.z as i32;

        let mut living_neighbors = 0;

        for alive_coord in alive_coords.iter() {
            if( alive_coord.0 as i32 == x - 1 && alive_coord.1 as i32 == z - 1) ||
                (alive_coord.0 as i32 == x - 1 && alive_coord.1 as i32 == z) ||
                (alive_coord.0 as i32 == x - 1 && alive_coord.1 as i32 == z + 1) ||
                (alive_coord.0 as i32 == x && alive_coord.1 as i32 == z - 1) ||
                (alive_coord.0 as i32 == x && alive_coord.1 as i32 == z + 1) ||
                (alive_coord.0 as i32 == x + 1 && alive_coord.1 as i32 == z - 1) ||
                (alive_coord.0 as i32 == x + 1 && alive_coord.1 as i32 == z) ||
                (alive_coord.0 as i32 == x + 1 && alive_coord.1 as i32 == z + 1) {
                living_neighbors += 1;
            }
        }

        if (!ls.current_status && living_neighbors == 3) || (ls.current_status && (living_neighbors == 2 || living_neighbors == 3)) {
            new_life_statuses.push(true);
        } else {
            new_life_statuses.push(false);
        }
    }

    let mut new_statuses_iter = new_life_statuses.iter();
    for mut item in query.iter_mut() {
        item.0.current_status = new_statuses_iter.next().unwrap().clone();
    }
}