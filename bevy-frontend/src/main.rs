use bevy::{prelude::*, utils::HashMap};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use lsystem::{LSystem, RuleSet, lsystem, vectorize};
use bevy_prototype_debug_lines::{ DebugLinesPlugin, DebugLines, Line };
use std::iter::{Iterator, IntoIterator};

struct LSystems {
	pub systems: Vec<(String, LSystem, f32)>,
	pub current_index: usize
}

struct LSystemEditorState {
	new_rule: String,
	new_rule_symbol: String,
	new_rule_name: String
}

struct OrbitCam;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
		.insert_resource(LSystems {
			systems: vec![
				("Plant 1".to_string(), lsystem!("+TT+F", F => "F[Fz[zFZXFZYF]Z[ZFxzFyzF]C+]"), 23.0),
				("Plant 2".to_string(), lsystem!("+TT+R", R => "FFF[FXYZ[FxRxF[zFRzXFC]R[ZFZyFC]]yFRyF]"), 23.0),
				("Sirpenski".to_string(), lsystem!("T", T => "FxTxF", F => "TXFXT"), 60.0),
				("Plant 3".to_string(), lsystem!("+TT+R", R => "F[[yyBBzB]XB]", B => "XXYYYYYYYYFRFzzFRRC"), 23.0),
				("Dragon Curve".to_string(), lsystem!("T", T => "TxF", F => "TXF"), 90.0),
				("Hilbert".to_string(), lsystem!("T", T => "YxTFYxTFTzFYXXTFTyFZXXTFTzFXTzX"), 90.0),
			],
			current_index: 0usize,
		})
		.insert_resource(LSystemEditorState{
			new_rule: "".to_string(),
			new_rule_symbol: "".to_string(),
			new_rule_name: "new-system".to_string(),
		})
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
		.add_plugin(EguiPlugin)
        .add_startup_system(setup.system())
		.add_system(orbit_system.system())
		.add_system(draw_lsystem_debug.system())
		.add_system(ui_systems.system())
        .run();
}

fn ui_systems(
	egui_context: ResMut<EguiContext>,
	mut systems: ResMut<LSystems>,
	mut editor_state: ResMut<LSystemEditorState>
) {
	egui::Window::new("Viewer Selection").show(egui_context.ctx(), |ui| {
		egui::ComboBox::from_label("Systems").selected_text(format!("{:?}", systems.systems[systems.current_index].0)).show_ui(ui, |ui| {
			for i in 0..systems.systems.len() {
				let name = systems.systems[i].0.clone();
				ui.selectable_value(&mut systems.current_index, i, format!("{:?}", name));
			}
		});

		let current_index = systems.current_index;
		ui.horizontal(|ui| {
			if ui.button("Iter").clicked() {
				systems.systems[current_index].1.next();
			}
			if ui.button("Reset").clicked() {
				systems.systems[current_index].1.reset();
			}
			ui.add(egui::Slider::new(&mut systems.systems[current_index].2, 0.0..=360.0).text("rotation"));
		});

		let mut axiom_str: String = systems.systems[current_index].1.get_axiom_str();
		let axiom_response = ui.text_edit_singleline(&mut axiom_str);
		if axiom_response.changed() {
			systems.systems[current_index].1.set_axiom(axiom_str.chars().collect());
			systems.systems[current_index].1.reset();
		}

		let mut updated_rules: RuleSet = RuleSet::new();
		for (key, value) in systems.systems[current_index].1.get_rules() {
			ui.horizontal(|ui| {
				ui.label(key.to_string());
				let mut rule_str: String = value.into_iter().collect();
				let rule_response = ui.text_edit_singleline(&mut rule_str);
				if rule_response.changed() {
					updated_rules.insert(*key, rule_str.chars().collect());
				}
			});
		}

        ui.separator();

		ui.label("Symbol");
		ui.text_edit_singleline(&mut editor_state.new_rule_symbol);
		ui.label("rule");
		ui.text_edit_singleline(&mut editor_state.new_rule);

		if ui.button("Add Rule").clicked() {
			systems.systems[current_index].1.add_rule(editor_state.new_rule_symbol.chars().next().unwrap(), editor_state.new_rule.chars().collect());
		}

		if updated_rules.len() > 0 {
			for (k, v) in updated_rules {
				systems.systems[current_index].1.add_rule(k, v);
			}
		}

        ui.separator();

		ui.text_edit_singleline(&mut editor_state.new_rule_name);

		if ui.button("New System").clicked() {
			systems.systems.push((editor_state.new_rule_name.clone(), lsystem::LSystem::new(), 0.0));
			systems.current_index = systems.systems.len() - 1;
		}

		if ui.button("DELETE SYSTEM").clicked() {
			systems.systems.remove(current_index);
			systems.current_index = systems.systems.len() - 1;
		}

		ui.text_edit_singleline(&mut format!("{:?}",systems.systems[current_index].1));
    });
}

fn orbit_system(
	time: Res<Time>,
	mut camera_query: Query<(&OrbitCam, &mut Transform)>
) {
	for (_, mut transform) in camera_query.iter_mut() {
		let x = time.time_since_startup().as_secs_f32().cos();
		let z = time.time_since_startup().as_secs_f32().sin();
		transform.translation.x = x * 100.0;
		transform.translation.z = z * 100.0;
		transform.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y)
	}
}

fn draw_lsystem_debug(
	mut lines: ResMut<DebugLines>,
	systems: Res<LSystems>
) {
	let mut current_position = Vec3::new(0.0, 0.0, 0.0);
	let mut direction = Vec3::new(0.0, 1.0, 0.0);
	let length = 1.0f32;
	let mut direction_stack: Vec<Vec3> = vec![];
	let mut position_stack: Vec<Vec3> = vec![];
	let angle = systems.systems[systems.current_index].2;

	for c in systems.systems[systems.current_index].1.get_current().iter() {
		match c {
			't' => {
				lines.lines.push(Line::new(current_position, current_position + (-direction * length), 0.0, Color::GREEN, Color::GREEN));
				current_position = current_position + (direction.normalize() * length);
			},
			'T' => {
				lines.lines.push(Line::new(current_position, current_position + (direction * length), 0.0, Color::GREEN, Color::GREEN));
				current_position = current_position + (direction.normalize() * length);
			},
			'F' => {
				lines.lines.push(Line::new(current_position, current_position + (direction * length), 0.0, Color::GREEN, Color::GREEN));
				current_position = current_position + (direction.normalize() * length);
			},
			'z' => {
				direction = Quat::from_rotation_z(angle.to_radians()).mul_vec3(direction);
			},
			'Z' => {
				direction = Quat::from_rotation_z(-angle.to_radians()).mul_vec3(direction);
			},
			'x' => {
				direction = Quat::from_rotation_x(angle.to_radians()).mul_vec3(direction);
			},
			'X' => {
				direction = Quat::from_rotation_x(-angle.to_radians()).mul_vec3(direction);
			},
			'y' => {
				direction = Quat::from_rotation_y(angle.to_radians()).mul_vec3(direction);
			},
			'Y' => {
				direction = Quat::from_rotation_y(-angle.to_radians()).mul_vec3(direction);
			},
			'|' => {
				direction = Quat::from_rotation_x(180.0f32.to_radians()).mul_vec3(direction);
			},
			'[' => {
				direction_stack.push(direction);
				position_stack.push(current_position);
			},
			']' => {
				direction = direction_stack.pop().unwrap();
				current_position = position_stack.pop().unwrap();
			},
			_ => {}
		}
	}
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(100.0, 50.0, 100.0)).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
	}).insert(OrbitCam);
}