use bevy::prelude::*;
use player::PlayerPlugin;

mod components;
mod player;

// region: --- Assests Constant

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const SPRITE_SCALE: f32 = 0.5;

// endregion: --- Assests Constant

// region: --- Game Constants

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

// endregion: --- Game Constants

// region: --- Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}
// endregion: --- Resources

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Auri's First Bevy Project".to_string(),
            width: 500.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    //add winsize resource
    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);

    //add GameTexture resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };
    commands.insert_resource(game_textures);
}
