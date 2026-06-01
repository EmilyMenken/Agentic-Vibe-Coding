use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::render::settings::{RenderCreation, WgpuSettings, Backends};
use bevy::render::RenderPlugin;

#[derive(Resource)]
struct DinoList {
    dinos: Vec<DinoData>,
    current_index: usize,
    score: u32,
}

#[derive(Clone)]
struct DinoData {
    filename: String,
    full_name: String,
    two_pt: Vec<String>,
    one_pt: Vec<String>,
    hint: String,
    user_answer: Option<String>,
    points_earned: Option<u32>,
    hint_used: bool,
}

#[derive(Resource, Default)]
struct TypingBuffer(String);

#[derive(Resource, Default)]
struct SkipConfirm(bool);

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

#[derive(Component)]
struct HintText;

fn make_dino_list() -> Vec<DinoData> {
    vec![
        DinoData {
            filename: "Ankylosaurus.png".into(),
            full_name: "ankylosaurus".into(),
            two_pt: vec!["anky".into()],
            one_pt: vec!["ornithischian".into()],
            hint: "The most heavily armored dinosaur.".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Carcharodontosaurus.png".into(),
            full_name: "carcharodontosaurus".into(),
            two_pt: vec!["carchar".into()],
            one_pt: vec!["theropod".into()],
            hint: "This guy's name means Shark-Toothed/Jagged-Toothed Lizard.".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Carnotaurus.png".into(),
            full_name: "carnotaurus".into(),
            two_pt: vec!["carno".into()],
            one_pt: vec!["theropod".into()],
            hint: "Loved this guy in Disney's Dinosaur, and in the Jurassic Park novel.".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Deinosuchus.png".into(),
            full_name: "deinosuchus".into(),
            two_pt: vec!["deino".into()],
            one_pt: vec!["croc".into(), "crocodylia".into()],
            hint: "One of the largest crocodilians of all time, starts with a D!".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Dilophosaurus.png".into(),
            full_name: "dilophosaurus".into(),
            two_pt: vec!["dilo".into()],
            one_pt: vec!["theropod".into()],
            hint: "This guy has a frill in Jurassic Park.".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Giganotosaurus.png".into(),
            full_name: "giganotosaurus".into(),
            two_pt: vec!["giga".into()],
            one_pt: vec!["theropod".into()],
            hint: "Loved this guy in Jurassic World Dominion!".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Hatzegopteryx.png".into(),
            full_name: "hatzegopteryx".into(),
            two_pt: vec!["hatz".into()],
            one_pt: vec!["pterosaur".into(), "pterosauria".into()],
            hint: "The heaviest flying reptile.".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Kentrosaurus.png".into(),
            full_name: "kentrosaurus".into(),
            two_pt: vec!["kentro".into()],
            one_pt: vec!["ornithischian".into()],
            hint: "This guy's name includes the city Kent.".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Mosasaurus.png".into(),
            full_name: "mosasaurus".into(),
            two_pt: vec!["mosa".into()],
            one_pt: vec!["marine reptile".into(), "squamata".into()],
            hint: "Loved this guy in Jurassic World - a great marine reptile.".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Quetzalcoatlus.png".into(),
            full_name: "quetzalcoatlus".into(),
            two_pt: vec!["quetz".into()],
            one_pt: vec!["pterosaur".into(), "pterosauria".into()],
            hint: "The tallest flying reptile - as tall as a giraffe.".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Spinosaurus.png".into(),
            full_name: "spinosaurus".into(),
            two_pt: vec!["spino".into()],
            one_pt: vec!["theropod".into()],
            hint: "Loved this guy in Jurassic Park 3!".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Stegosaurus.png".into(),
            full_name: "stegosaurus".into(),
            two_pt: vec!["stego".into()],
            one_pt: vec!["ornithischian".into()],
            hint: "Loved this guy in Jurassic Park 2!".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Therizinosaurus.png".into(),
            full_name: "therizinosaurus".into(),
            two_pt: vec!["theri".into()],
            one_pt: vec!["theropod".into()],
            hint: "The weirdest theropod... a herbivore with long Freddy Krueger-esque claws.".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Triceratops.png".into(),
            full_name: "triceratops".into(),
            two_pt: vec!["trike".into()],
            one_pt: vec!["ornithischian".into()],
            hint: "This guy was sick in Jurassic Park! Literally!".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "TyrannosaurusRex.png".into(),
            full_name: "tyrannosaurus rex".into(),
            two_pt: vec!["trex".into(), "t-rex".into(), "t rex".into()],
            one_pt: vec!["theropod".into()],
            hint: "Oh, you know who this is. Take a wild guess...".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Utahraptor.png".into(),
            full_name: "utahraptor".into(),
            two_pt: vec!["raptor".into()],
            one_pt: vec!["theropod".into()],
            hint: "The largest known raptor!".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Velociraptor.png".into(),
            full_name: "velociraptor".into(),
            two_pt: vec!["raptor".into()],
            one_pt: vec!["theropod".into()],
            hint: "This little guy is a lot smaller IRL than in the movies...".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
        },
        DinoData {
            filename: "Yutyrannus.png".into(),
            full_name: "yutyrannus".into(),
            two_pt: vec!["yuty".into()],
            one_pt: vec!["theropod".into()],
            hint: "Largest dinosaur known to have feathers!".into(),
            user_answer: None,
            points_earned: None,
            hint_used: false,
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
        })
        .insert_resource(TypingBuffer::default())
        .insert_resource(SkipConfirm::default())
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
            custom_size: Some(Vec2::new(500., 500.)),
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
        Text::new("What dinosaur/reptile is this? Type your answer. Enter to submit, type 'skip' to skip."),
        TextFont { font_size: 16.0, ..default() },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(110.),
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
            bottom: Val::Px(75.),
            left: Val::Px(20.),
            ..default()
        },
        InputText,
    ));

    commands.spawn((
        Text::new("Down Arrow for a hint"),
        TextFont { font_size: 16.0, ..default() },
        TextColor(Color::srgb(0.5, 0.8, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.),
            left: Val::Px(20.),
            ..default()
        },
        HintText,
    ));

    commands.spawn((
        Text::new(""),
        TextFont { font_size: 22.0, ..default() },
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
    mut skip_confirm: ResMut<SkipConfirm>,
) {
    use bevy::input::keyboard::Key;

    let total = dino_list.dinos.len();
    let idx = dino_list.current_index;

    if idx >= total {
        return;
    }

    let already_answered = dino_list.dinos[idx].user_answer.is_some();

    // Left arrow — go back (view only)
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        if idx > 0 {
            dino_list.current_index -= 1;
            buffer.0.clear();
            skip_confirm.0 = false;
        }
        return;
    }

    // Right arrow — move forward (only after answered)
    if keyboard.just_pressed(KeyCode::ArrowRight) {
        if already_answered && idx + 1 <= total {
            dino_list.current_index += 1;
            buffer.0.clear();
            skip_confirm.0 = false;
        }
        return;
    }

    // Down arrow — show hint (only if not already answered)
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        if !already_answered {
            dino_list.dinos[idx].hint_used = true;
        }
        return;
    }

    // Enter — submit (only if not already answered)
    if keyboard.just_pressed(KeyCode::Enter) {
        if !already_answered {
            let trimmed = buffer.0.trim().to_lowercase();

            if skip_confirm.0 {
                if trimmed == "yes" {
                    dino_list.dinos[idx].user_answer = Some("skip".into());
                    dino_list.dinos[idx].points_earned = Some(0);
                    buffer.0.clear();
                    skip_confirm.0 = false;
                } else {
                    buffer.0.clear();
                    skip_confirm.0 = false;
                }
            } else if trimmed == "skip" {
                skip_confirm.0 = true;
                buffer.0.clear();
            } else if !trimmed.is_empty() {
                let pts = score_answer(&trimmed, &dino_list.dinos[idx].clone());
                dino_list.score += pts;
                dino_list.dinos[idx].user_answer = Some(trimmed.clone());
                dino_list.dinos[idx].points_earned = Some(pts);
                buffer.0.clear();
                skip_confirm.0 = false;
            }
        }
        return;
    }

    // Typing — only allowed if not already answered
    if !already_answered {
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
                    _ => {}
                }
            }
        }
    } else {
        evr_char.clear();
    }
}

fn update_ui(
    mut image_query: Query<&mut Sprite, With<DinoImage>>,
    mut input_query: Query<&mut Text, (With<InputText>, Without<ScoreText>, Without<FeedbackText>, Without<PromptText>, Without<HintText>)>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<InputText>, Without<FeedbackText>, Without<PromptText>, Without<HintText>)>,
    mut feedback_query: Query<&mut Text, (With<FeedbackText>, Without<InputText>, Without<ScoreText>, Without<PromptText>, Without<HintText>)>,
    mut prompt_query: Query<&mut Text, (With<PromptText>, Without<InputText>, Without<ScoreText>, Without<FeedbackText>, Without<HintText>)>,
    mut hint_query: Query<&mut Text, (With<HintText>, Without<InputText>, Without<ScoreText>, Without<FeedbackText>, Without<PromptText>)>,
    buffer: Res<TypingBuffer>,
    dino_list: Res<DinoList>,
    skip_confirm: Res<SkipConfirm>,
    asset_server: Res<AssetServer>,
) {
    let total = dino_list.dinos.len();
    let idx = dino_list.current_index;

    if let Ok(mut text) = score_query.get_single_mut() {
        **text = format!("Score: {} | Dino {}/{}", dino_list.score, (idx + 1).min(total), total);
    }

    // Game over screen
    if idx >= total {
        if let Ok(mut text) = prompt_query.get_single_mut() {
            **text = format!("Game over! Final score: {}/{}", dino_list.score, total * 3);
        }
        if let Ok(mut text) = input_query.get_single_mut() {
            **text = "Thanks for playing!".into();
        }
        if let Ok(mut text) = feedback_query.get_single_mut() {
            **text = "Left Arrow to review your answers".into();
        }
        if let Ok(mut text) = hint_query.get_single_mut() {
            **text = "".into();
        }
        return;
    }

    let dino = &dino_list.dinos[idx];
    let already_answered = dino.user_answer.is_some();

    // Update image
    if let Ok(mut sprite) = image_query.get_single_mut() {
        sprite.image = asset_server.load(format!("images/{}", dino.filename));
    }

    // Update prompt
    if let Ok(mut text) = prompt_query.get_single_mut() {
        if skip_confirm.0 {
            **text = "Are you sure you want to skip? Type 'yes' + Enter to confirm, or just Enter to cancel.".into();
        } else if already_answered {
            **text = "Left Arrow back  |  Right Arrow next".into();
        } else {
            **text = "What dinosaur/reptile is this? Type your answer. Enter to submit, type 'skip' to skip.".into();
        }
    }

    // Update input line
    if let Ok(mut text) = input_query.get_single_mut() {
        if already_answered {
            let answer = dino.user_answer.as_deref().unwrap_or("");
            let pts = dino.points_earned.unwrap_or(0);
            if answer == "skip" {
                **text = "You skipped this one.".into();
            } else {
                **text = format!("Your answer: {} (+{} pts)", answer, pts);
            }
        } else {
            **text = format!("> {}", buffer.0);
        }
    }

    // Update hint
    if let Ok(mut text) = hint_query.get_single_mut() {
        if already_answered {
            **text = "".into();
        } else if dino.hint_used {
            **text = format!("Hint: {}", dino.hint);
        } else {
            **text = "Down Arrow for a hint".into();
        }
    }

    // Update feedback
    if let Ok(mut text) = feedback_query.get_single_mut() {
        if already_answered {
            let pts = dino.points_earned.unwrap_or(0);
            let msg = match pts {
                3 => format!("✓ Perfect! — it was {}", dino.full_name),
                2 => format!("~ Close! — it was {}", dino.full_name),
                1 => format!("~ Partial! — it was {}", dino.full_name),
                _ => format!("✗ — it was {}", dino.full_name),
            };
            **text = msg;
        } else {
            **text = "".into();
        }
    }
}