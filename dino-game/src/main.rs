use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::render::settings::{RenderCreation, WgpuSettings, Backends};
use bevy::render::RenderPlugin;

#[derive(Resource)]
struct DinoList {
    dinos: Vec<DinoData>,
    current_index: usize,
    score: u32,
    answered: bool,
}

#[derive(Clone)]
struct DinoData {
    filename: String,
    full_name: String,
    two_pt: Vec<String>,
    one_pt: Vec<String>,
}

#[derive(Component)]
struct DinoImage;

#[derive(Component)]
struct InputText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct FeedbackText;

#[derive(Component)]
struct PromptText;

#[derive(Resource, Default)]
struct TypingBuffer(String);

fn make_dino_list() -> Vec<DinoData> {
    vec![
        DinoData {
            filename: "Ankylosaurus.png".into(),
            full_name: "ankylosaurus".into(),
            two_pt: vec!["anky".into()],
            one_pt: vec!["ornithischian".into()],
        },
        DinoData {
            filename: "Carcharodontosaurus.png".into(),
            full_name: "carcharodontosaurus".into(),
            two_pt: vec!["carchar".into()],
            one_pt: vec!["theropod".into()],
        },
        DinoData {
            filename: "Carnotaurus.png".into(),
            full_name: "carnotaurus".into(),
            two_pt: vec!["carno".into()],
            one_pt: vec!["theropod".into()],
        },
        DinoData {
            filename: "Deinosuchus.png".into(),
            full_name: "deinosuchus".into(),
            two_pt: vec!["deino".into()],
            one_pt: vec!["croc".into(), "crocodylia".into()],
        },
        DinoData {
            filename: "Dilophosaurus.png".into(),
            full_name: "dilophosaurus".into(),
            two_pt: vec!["dilo".into()],
            one_pt: vec!["theropod".into()],
        },
        DinoData {
            filename: "Giganotosaurus.png".into(),
            full_name: "giganotosaurus".into(),
            two_pt: vec!["giga".into()],
            one_pt: vec!["theropod".into()],
        },
        DinoData {
            filename: "Hatzegopteryx.png".into(),
            full_name: "hatzegopteryx".into(),
            two_pt: vec!["hatz".into()],
            one_pt: vec!["pterosaur".into(), "pterosauria".into()],
        },
        DinoData {
            filename: "Kentrosaurus.png".into(),
            full_name: "kentrosaurus".into(),
            two_pt: vec!["kentro".into()],
            one_pt: vec!["ornithischian".into()],
        },
        DinoData {
            filename: "Mosasaurus.png".into(),
            full_name: "mosasaurus".into(),
            two_pt: vec!["mosa".into()],
            one_pt: vec!["marine reptile".into(), "squamata".into()],
        },
        DinoData {
            filename: "Quetzalcoatlus.png".into(),
            full_name: "quetzalcoatlus".into(),
            two_pt: vec!["quetz".into()],
            one_pt: vec!["pterosaur".into(), "pterosauria".into()],
        },
        DinoData {
            filename: "Spinosaurus.png".into(),
            full_name: "spinosaurus".into(),
            two_pt: vec!["spino".into()],
            one_pt: vec!["theropod".into()],
        },
        DinoData {
            filename: "Stegosaurus.png".into(),
            full_name: "stegosaurus".into(),
            two_pt: vec!["stego".into()],
            one_pt: vec!["ornithischian".into()],
        },
        DinoData {
            filename: "Therizinosaurus.png".into(),
            full_name: "therizinosaurus".into(),
            two_pt: vec!["theri".into()],
            one_pt: vec!["theropod".into()],
        },
        DinoData {
            filename: "Triceratops.png".into(),
            full_name: "triceratops".into(),
            two_pt: vec!["trike".into()],
            one_pt: vec!["ornithischian".into()],
        },
        DinoData {
            filename: "TyrannosaurusRex.png".into(),
            full_name: "tyrannosaurus rex".into(),
            two_pt: vec!["trex".into(), "t-rex".into(), "t rex".into()],
            one_pt: vec!["theropod".into()],
        },
        DinoData {
            filename: "Utahraptor.png".into(),
            full_name: "utahraptor".into(),
            two_pt: vec!["raptor".into()],
            one_pt: vec!["theropod".into()],
        },
        DinoData {
            filename: "Velociraptor.png".into(),
            full_name: "velociraptor".into(),
            two_pt: vec!["raptor".into()],
            one_pt: vec!["theropod".into()],
        },
        DinoData {
            filename: "Yutyrannus.png".into(),
            full_name: "yutyrannus".into(),
            two_pt: vec!["yuty".into()],
            one_pt: vec!["theropod".into()],
        },
    ]
}

fn score_answer(answer: &str, dino: &DinoData) -> u32 {
    let answer = answer.trim().to_lowercase();
    if answer == dino.full_name {
        return 3;
    }
    for two in &dino.two_pt {
        if answer.contains(two.as_str()) {
            return 2;
        }
    }
    for one in &dino.one_pt {
        if answer.contains(one.as_str()) {
            return 1;
        }
    }
    0
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Dino Quiz!".into(),
                    resolution: (800., 600.).into(),
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::GL),
                    ..default()
                }),
                ..default()
            })
        )
        .insert_resource(DinoList {
            dinos: make_dino_list(),
            current_index: 0,
            score: 0,
            answered: false,
        })
        .insert_resource(TypingBuffer::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input, update_ui))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, dino_list: Res<DinoList>) {
    commands.spawn(Camera2d);

    let first = &dino_list.dinos[0];

    commands.spawn((
        Sprite {
            image: asset_server.load(format!("images/{}", first.filename)),
            custom_size: Some(Vec2::new(400., 350.)),
            ..default()
        },
        Transform::from_xyz(0., 80., 1.),
        DinoImage,
    ));

    commands.spawn((
        Text::new("Score: 0"),
        TextFont { font_size: 28.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.),
            left: Val::Px(10.),
            ..default()
        },
        ScoreText,
    ));

    commands.spawn((
        Text::new("What dinosaur is this? Press Enter to submit."),
        TextFont { font_size: 20.0, ..default() },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(90.),
            left: Val::Px(20.),
            ..default()
        },
        PromptText,
    ));

    commands.spawn((
        Text::new("> "),
        TextFont { font_size: 26.0, ..default() },
        TextColor(Color::srgb(0.2, 1.0, 0.4)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(55.),
            left: Val::Px(20.),
            ..default()
        },
        InputText,
    ));

    commands.spawn((
        Text::new(""),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::srgb(1.0, 1.0, 0.3)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.),
            left: Val::Px(20.),
            ..default()
        },
        FeedbackText,
    ));
}

fn keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut evr_char: EventReader<KeyboardInput>,
    mut buffer: ResMut<TypingBuffer>,
    mut dino_list: ResMut<DinoList>,
) {
    use bevy::input::keyboard::Key;

    if dino_list.answered {
        if keyboard.just_pressed(KeyCode::Enter) {
            dino_list.answered = false;
            dino_list.current_index += 1;
            buffer.0.clear();
        }
        return;
    }

    for ev in evr_char.read() {
        if ev.state.is_pressed() {
            match &ev.logical_key {
                Key::Character(c) => {
                    buffer.0.push_str(c.as_str());
                }
                Key::Space => {
                    buffer.0.push(' ');
                }
                Key::Backspace => {
                    buffer.0.pop();
                }
                Key::Enter => {
                    if dino_list.current_index < dino_list.dinos.len() {
                        let pts = score_answer(
                            &buffer.0,
                            &dino_list.dinos[dino_list.current_index].clone(),
                        );
                        dino_list.score += pts;
                        dino_list.answered = true;
                    }
                }
                _ => {}
            }
        }
    }
}

fn update_ui(
    mut image_query: Query<&mut Sprite, With<DinoImage>>,
    mut input_query: Query<&mut Text, (With<InputText>, Without<ScoreText>, Without<FeedbackText>, Without<PromptText>)>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<InputText>, Without<FeedbackText>, Without<PromptText>)>,
    mut feedback_query: Query<&mut Text, (With<FeedbackText>, Without<InputText>, Without<ScoreText>, Without<PromptText>)>,
    mut prompt_query: Query<&mut Text, (With<PromptText>, Without<InputText>, Without<ScoreText>, Without<FeedbackText>)>,
    buffer: Res<TypingBuffer>,
    dino_list: Res<DinoList>,
    asset_server: Res<AssetServer>,
) {
    let total = dino_list.dinos.len();

    if dino_list.current_index >= total {
        if let Ok(mut text) = prompt_query.get_single_mut() {
            **text = format!("Game over! Final score: {}/{}", dino_list.score, total * 3);
        }
        if let Ok(mut text) = input_query.get_single_mut() {
            **text = "Thanks for playing!".into();
        }
        if let Ok(mut text) = feedback_query.get_single_mut() {
            **text = "".into();
        }
        return;
    }

    let dino = &dino_list.dinos[dino_list.current_index];

    if let Ok(mut sprite) = image_query.get_single_mut() {
        sprite.image = asset_server.load(format!("images/{}", dino.filename));
    }

    if let Ok(mut text) = score_query.get_single_mut() {
        **text = format!("Score: {} | Dino {}/{}", dino_list.score, dino_list.current_index + 1, total);
    }

    if let Ok(mut text) = input_query.get_single_mut() {
        **text = format!("> {}", buffer.0);
    }

    if let Ok(mut text) = feedback_query.get_single_mut() {
        if dino_list.answered {
            let pts = score_answer(&buffer.0, dino);
            let msg = match pts {
                3 => format!("✓ Perfect! +3 pts — it was {}", dino.full_name),
                2 => format!("~ Close! +2 pts — it was {}", dino.full_name),
                1 => format!("~ Partial! +1 pt — it was {}", dino.full_name),
                _ => format!("✗ Nope! +0 pts — it was {}", dino.full_name),
            };
            **text = format!("{} | Press Enter for next", msg);
        } else {
            **text = "".into();
        }
    }

    if let Ok(mut text) = prompt_query.get_single_mut() {
        if dino_list.answered {
            **text = "".into();
        } else {
            **text = "What dinosaur is this? Press Enter to submit.".into();
        }
    }
}