use bevy::prelude::*;
use bevy::ui::DefaultCameraView;
use bevy::utils::HashSet;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};
use bevy_rapier2d::prelude::*;

use crate::bullet::AtkNormal;
use crate::enemy::Enemy;
use crate::enemy::Patrol;
use crate::player::Player;
use crate::wall::Collidable;

pub fn scene_plugin(app: &mut App) {
    app
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (
            camera_fit_inside_current_level,
            update_level_selection,
            on_spawn_ground_sensor,
            ground_detection,
            update_on_ground,
            check_door_color,
        ),
    );
}

#[derive(Component)]
enum ColorItem {
    Yellow,
    Red,
}

fn setup(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
) {
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., -1.0).with_scale(Vec3 { x: 0.4, y: 0.4, z: 0. }),
            texture: asset_server.load("images/bg.png"),
            ..Default::default()
        },
        Name::new("bg"),
    ));

    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3 { x: 0.3, y: 0.3, z: 0. }),
            texture: asset_server.load("images/door.png"),
            ..Default::default()
        },
        Name::new("door"),
    ));
    
    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., -305.3, 0.),
            ..Default::default()
        },
        Name::new("floor"),
        Collider::cuboid(1000., 50.),
    ));

    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..Default::default()
            },
            transform: Transform::from_xyz(-250., 100., 0.).with_scale(Vec3 { x: 0.3, y: 0.3, z: 0. }),
            texture: asset_server.load("images/yellow.png"),
            ..Default::default()
        },
        ColorItem::Yellow,
        Name::new("yellow_item"),
        Sensor,
    ));

    cmds.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..Default::default()
            },
            transform: Transform::from_xyz(200., 200., 0.).with_scale(Vec3 { x: 0.3, y: 0.3, z: 0. }),
            texture: asset_server.load("images/red.png"),
            ..Default::default()
        },
        ColorItem::Red,
        Name::new("red_item"),
        Sensor,
    ));
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PumpkinsBundle {
    #[sprite_sheet_bundle(no_grid)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ChestBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct MobBundle {
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub enemy: Enemy,
    #[ldtk_entity]
    pub patrol: Patrol,
}

#[derive(Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Items(Vec<String>);

impl From<&EntityInstance> for Items {
    fn from(entity_instance: &EntityInstance) -> Self {
        Items(
            entity_instance
                .iter_enums_field("items")
                .expect("items field should be correctly typed")
                .cloned()
                .collect(),
        )
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(6., 14.),
                rigid_body: RigidBody::KinematicVelocityBased,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..Default::default()
            },
            "Mob" => ColliderBundle {
                collider: Collider::cuboid(5., 5.),
                rigid_body: RigidBody::KinematicVelocityBased,
                rotation_constraints,
                ..Default::default()
            },
            "Chest" => ColliderBundle {
                collider: Collider::cuboid(8., 8.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale: GravityScale(1.0),
                friction: Friction::new(0.5),
                density: ColliderMassProperties::Density(15.0),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Climbable;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub rotation_constraints: LockedAxes,
}

impl From<IntGridCell> for SensorBundle {
    fn from(int_grid_cell: IntGridCell) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        // ladder
        if int_grid_cell.value == 2 {
            SensorBundle {
                collider: Collider::cuboid(8., 8.),
                sensor: Sensor,
                rotation_constraints,
            }
        } else {
            SensorBundle::default()
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct LadderBundle {
    #[from_int_grid_cell]
    pub sensor_bundle: SensorBundle,
    pub climbable: Climbable,
}

#[derive(Clone, Default, Component, Debug)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
    pub on_ground: bool,
}

pub fn on_spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &Collider), Added<GroundDetection>>,
) {
    for (entity, shape) in &detect_ground_for {
        if let Some(cuboid) = shape.as_cuboid() {
            let Vec2 {
                x: half_extents_x,
                y: half_extents_y,
            } = cuboid.half_extents();

            let detector_shape = Collider::cuboid(half_extents_x / 2.0, 2.);

            let sensor_translation = Vec3::new(0., -half_extents_y, 0.);

            commands.entity(entity).with_children(|builder| {
                builder
                    .spawn_empty()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(detector_shape)
                    .insert(Sensor)
                    .insert(Transform::from_translation(sensor_translation))
                    .insert(GlobalTransform::default());
            });
        }
    }
}

pub fn ground_detection(
    mut ground_sensors: Query<&mut GroundSensor>,
    mut collisions: EventReader<CollisionEvent>,
    collidables: Query<Entity, With<Collider>>,
) {
    for collision_event in collisions.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.insert(*e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.insert(*e2);
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if collidables.contains(*e1) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e2) {
                        sensor.intersecting_ground_entities.remove(e1);
                    }
                } else if collidables.contains(*e2) {
                    if let Ok(mut sensor) = ground_sensors.get_mut(*e1) {
                        sensor.intersecting_ground_entities.remove(e2);
                    }
                }
            }
        }
    }
}

pub fn update_on_ground(mut ground_sensors: Query<&mut GroundSensor>) {
    for mut sensor in ground_sensors.iter_mut() {
        sensor.on_ground = !sensor.intersecting_ground_entities.is_empty();
    }
}

pub fn update_level_selection(
    level_query: Query<(&LevelIid, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for (level_iid, level_transform) in &level_query {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        let level_bounds = Rect {
            min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
            max: Vec2::new(
                level_transform.translation.x + level.px_wid as f32,
                level_transform.translation.y + level.px_hei as f32,
            ),
        };

        for player_transform in &player_query {
            if player_transform.translation.x < level_bounds.max.x
                && player_transform.translation.x > level_bounds.min.x
                && player_transform.translation.y < level_bounds.max.y
                && player_transform.translation.y > level_bounds.min.y
                && !level_selection.is_match(&LevelIndices::default(), level)
            {
                *level_selection = LevelSelection::iid(level.iid.clone());
            }
        }
    }
}

const ASPECT_RATIO: f32 = 16. / 9.;

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        (With<IsDefaultUiCamera>, Without<Player>),
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_iid) in &level_query {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_projects.single())
                .expect("Project should be loaded if level has spawned");

            let level = ldtk_project
                .get_raw_level_by_iid(&level_iid.to_string())
                .expect("Spawned level should exist in LDtk project");

            if level_selection.is_match(&LevelIndices::default(), level) {
                let level_ratio = level.px_wid as f32 / level.px_hei as f32;
                orthographic_projection.viewport_origin = Vec2::ZERO;
                if level_ratio > ASPECT_RATIO {
                    // level is wider than the screen
                    let height = (level.px_hei as f32 / 9.).round() * 9.;
                    let width = height * ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.x =
                        (player_translation.x - level_transform.translation.x - width / 2.)
                            .clamp(0., level.px_wid as f32 - width);
                    camera_transform.translation.y = 0.;
                } else {
                    // level is taller than the screen
                    let width = (level.px_wid as f32 / 16.).round() * 16.;
                    let height = width / ASPECT_RATIO;
                    orthographic_projection.scaling_mode =
                        bevy::render::camera::ScalingMode::Fixed { width, height };
                    camera_transform.translation.y =
                        (player_translation.y - level_transform.translation.y - height / 2.)
                            .clamp(0., level.px_hei as f32 - height);
                    camera_transform.translation.x = 0.;
                }

                camera_transform.translation.x += level_transform.translation.x;
                camera_transform.translation.y += level_transform.translation.y;
            }
        }
    }
}


fn check_door_color(
    mut collisions: EventReader<CollisionEvent>,
    query_color_item: Query<Entity, (With<ColorItem>, Without<AtkNormal>)>,
    query_bullet: Query<Entity, (With<AtkNormal>, Without<ColorItem>)>,
) 
{
    for collision_event in collisions.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if (query_color_item.contains(*e1) && query_bullet.contains(*e2)) || (query_color_item.contains(*e2) && query_bullet.contains(*e1)) {
                    println!("Door open");
                }
            }
            _ => {},
        }
    }
}