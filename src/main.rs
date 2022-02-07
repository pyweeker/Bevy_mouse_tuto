use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
//use rand::Rng;
use rand::prelude::*;

use bevy::render::texture::ImageType;
use std::path::Path;

use bevy::sprite::collide_aabb::{collide, Collision};

use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},    
    window::CursorMoved,
};



//use rand::{prelude::SliceRandom, Rng};

const SPRITE_DIR: &str = "assets";


const HOVER_SPRITE : &str = "hover.png";
const DRAG_SPRITE : &str = "drag.png";
const HOVERDRAG_SPRITE : &str = "hoverdrag.png";


#[derive(Component)]
struct MouseTargetable;


struct CursorPosition {
    pos: Vec2,
}



// region:    Resources
pub struct SpriteInfos {
    hover_info: (Handle<Image>, Vec2),
    drag_info: (Handle<Image>, Vec2),
    hoverdrag_info: (Handle<Image>, Vec2),
    //enemy_laser: (Handle<Image>, Vec2),
    //explosion: Handle<TextureAtlas>,
}


// Note - With bevy v0.6, load images directly and synchronously to capture size
//        See https://github.com/bevyengine/bevy/pull/3696
fn load_image(images: &mut ResMut<Assets<Image>>, path: &str) -> (Handle<Image>, Vec2) {
    let path = Path::new(SPRITE_DIR).join(path);
    let bytes = std::fs::read(&path).expect(&format!("Cannot find {}", path.display()));
    //let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png")).unwrap();
    let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png")).unwrap();
    let size = image.texture_descriptor.size;
    let size = Vec2::new(size.width as f32, size.height as f32);
    let image_handle = images.add(image);
    (image_handle, size)
}




/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    //mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {

    let window = windows.get_primary_mut().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());


    commands.spawn()
        //.insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    //commands.insert_resource(SpriteInfos {
    //    hover_info: load_image(&mut images, HOVER_SPRITE),
    //    drag_info: load_image(&mut images, DRAG_SPRITE),
    //    hoverdrag_info: load_image(&mut images, HOVERDRAG_SPRITE),




    // DEBUNK POSITION W/2 H/2
    //let mut rnd = rand::thread_rng();
    //let pos = (rnd.gen_range(-200.0..200.0), rnd.gen_range(-200.0..200.0));
    let pos = (0.0, 0.0);



    let transform = Transform::from_xyz(pos.0, pos.1, 0.0);

    let texture_handle = asset_server.load(HOVER_SPRITE);

    let entity = commands
            .spawn()
            
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    //custom_size: Some(Vec2::new(1.0, 1.0) * SPRITE_SIZE),
                    custom_size: Some(Vec2::new(256.0, 256.0)),
                    
                    //color: Color::hsla(hue, SATURATION_DESELECTED, LIGHTNESS_DESELECTED, ALPHA),
                    color: Color::hsla(180.0, 0.5, 0.5, 1.0),

                    
                    ..Default::default()
                },
                texture: texture_handle.clone(),
                transform,
                ..Default::default()
            })

            .insert(MouseTargetable)
            .id();




        /////asset_server.load(picked_name),
        //enemy_laser: load_image(&mut images, ENEMY_LASER_SPRITE),
        //explosion: texture_atlases.add(texture_atlas),
    //});
}

/*

fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>
) {
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single();

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        eprintln!("World coords: {}/{}", pos_wld.x, pos_wld.y);
    }
}

*/

//fn mouse_move(mut commands: Commands, cursor: Res<Cursor>, mut query: Query<(&Translation,&mut Sprite, &MouseTargetable)>) 
//fn mouse_move(windows: Res<Windows>, mouse_input: Res<Input<MouseButton>>, mut commands: Commands, mut query: Query<(&Translation,&mut Sprite, &MouseTargetable)>) 
fn mouse_move_system(windows: Res<Windows>, mouse_input: Res<Input<MouseButton>>, mut commands: Commands, mut query: Query<(&Transform,&mut Sprite, &MouseTargetable)>) 

{

    let win = windows.get_primary().expect("no primary window");

    let mut cursor = win.cursor_position();

    //println!(" cursor {:?} ",cursor);  // type = Some(Vec2)

    //match *cursor {
    //match cursor.unwrap() {
    match cursor {
        //Some(_) => true,
        None => {},
        Some(_) => {


            //let mut cursor_tup = cursor.unwrap();  //  collide => expected `bool`, found enum `std::option::Option
            let mut cursor_tup = cursor.unwrap().clone();

            //- win.width() / 2.0
            //- win.height() / 2.0


            //let cursor_vec = Vec3::new(cursor_tup[0],cursor_tup[1],0.0);
            let cursor_vec = Vec3::new(cursor_tup[0]- win.width() / 2.0,cursor_tup[1]- win.height() / 2.0,0.0);

            for (transform,mut sprite,_target) in &mut query.iter() {

                //if collide(cursor_vec,Vec2::new(1.0,1.0), transform.translation, Vec2(256.0, 256.0)) {
                match collide(cursor_vec,Vec2::new(10.0,10.0), transform.translation, Vec2::new(256.0, 256.0)) {

                    Some(_) => println!(" cursor_tup {} translation.x {}   translation.y  {}",cursor_tup, transform.translation.x, transform.translation.y),
                    None => {} // { println!("\n    -   match collide is None ") },

                 }
             }

            //print_type_of()
        },
    }

    /*

    if let cursor_tup = translate_cursor ((cursor.0,cursor.1));


    for (translation,mut sprite,_target) in &mut query.iter() {
        let cursor_tup = translate_cursor ((cursor.0,cursor.1));
        let cursor_vec = Vec3::new(cursor_tup.0,cursor_tup.1,0.0);
        if collides(cursor_vec,Vec2::new(1.0,1.0),translation.0,sprite.size) {
            //println!("{}",name.0);
            println!(" cursor_tup {} translation.0 {}   translation.1  {}",cursor_tup, translation.0, translation.1);

        }
    }

    */
}



/*
fn mouse_system(
    
    //buttons: Res<Input<MouseButton>>,
    //mouse_pos: ResMut<MouseLoc>,




    mut egui_ctx: ResMut<EguiContext>,
    assets: Res<AssetServer>,

    mut commands: Commands,

    //q_names: Query<(&Naming, &Position, Option<&Health>)>,
    q_names: Query<(&Naming, &Transform)>,

    windows: Res<Windows>,
    mouse_input: Res<Input<MouseButton>>,



) {

    let win = windows.get_primary().expect("no primary window");

    if mouse_input.just_pressed(MouseButton::Left) {
        println!("click at {:?}", win.cursor_position());
    }

    //if buttons.just_pressed(MouseButton::Left) {
    if mouse_input.just_pressed(MouseButton::Left) {

        println!(" mouse_input  ");

        if ui_state.current_target_idx_MOUSE.is_none() {
            println!(" ui_state.current_target_idx_MOUSE.is_none() ");
            ui_state.current_target_idx_MOUSE = Some(0);
            println!(" ... NOW ui_state.current_target_idx_MOUSE = Some(0)");

            let current_usize: usize = ui_state.current_target_idx_MOUSE.unwrap();
            println!(" ui_state.vec_texturenames[current_usize] = {} ", ui_state.vec_texturenames[current_usize]);

            //let fresh_selected_texture_handle_ONE = assets.load(ui_state.vec_texturenames[current_usize]);
            //egui_ctx.set_egui_texture(BEVY_TEXTURE_ID_ONE, fresh_selected_texture_handle_ONE);
            //ui_state.current_png_String = ui_state.vec_texturenames[current_usize].to_string();


            //for (nam, tf) in q_names.iter() { println!(" {}  {} ", tf.translation.x, tf.translation.y) ; println!(" mouse_pos = {} ", mouse_pos); }

        }
    }
}
*/

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.8)))
        .insert_resource(CursorPosition { pos: Vec2::ZERO })

        .insert_resource(WindowDescriptor {
            title: "Mouse testing".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)


        
        .add_startup_system(setup)

        //.add_system(my_cursor_system)

        .add_system(mouse_move_system)

        .add_system(print_mouse_events_system)

        .add_system(cursor_position_system)


        

        

        .run();
}



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn print_type_of_mut_version<T>(_: &mut T) {
    println!("{}", std::any::type_name::<T>())
}


/*
fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

*/

// helper function to get cursor position
fn calculate_cursor_position(windows: Res<Windows>) -> Option<Vec2> {
    let window = windows.get_primary()?;
    let cursor_position = window.cursor_position()?;
    Some(Vec2::new(
        cursor_position.x - window.width() / 2.0,
        cursor_position.y - window.height() / 2.0,
    ))
}




/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,


    
    windows: Res<Windows>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    for event in mouse_button_input_events.iter() {
        //info!("{:?}", event);
    }
    for event in mouse_motion_events.iter() {
        //info!("{:?}", event);
    }
    for event in cursor_moved_events.iter() {
        //info!("{:?}", event);
        //print_type_of(event);        
    }
    for event in mouse_wheel_events.iter() {
        //info!("{:?}", event);
    }
}




fn cursor_position_system(windows: Res<Windows>, mut cursor_position: ResMut<CursorPosition>) {
    if let Some(cursor_pos) = calculate_cursor_position(windows) {
        cursor_position.pos = cursor_pos;

        //println!(" cursor_pos = {} ", cursor_pos);
    }
}