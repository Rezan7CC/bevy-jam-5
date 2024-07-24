use crate::GameState;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.25, 0.5, 0.25);

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::srgba(0.1, 0.1, 0.1, 0.5);

// All actions that can be triggered from a button click
#[derive(Component)]
pub enum UIButtonAction {
    Play,
}

#[derive(Component)]
pub struct OnMainMenuScreen;

pub fn system_ui_actions(
    interaction_query: Query<(&Interaction, &UIButtonAction), (Changed<Interaction>, With<Button>)>,
    main_menu_screen: Query<Entity, With<OnMainMenuScreen>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for (interaction, ui_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match ui_button_action {
                UIButtonAction::Play => {
                    game_state.set(GameState::Running);
                    despawn_screen::<OnMainMenuScreen>(&main_menu_screen, &mut commands);
                }
            }
        }
    }
}

pub fn system_button_color(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        background_color.0 = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }
    }
}

fn despawn_screen<T: Component>(to_despawn: &Query<Entity, With<T>>, commands: &mut Commands) {
    for entity in to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn system_create_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Ducky Boids",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            UIButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("ui/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent
                                .spawn(TextBundle::from_section("Play", button_text_style.clone()));
                        });
                });
        });
}
