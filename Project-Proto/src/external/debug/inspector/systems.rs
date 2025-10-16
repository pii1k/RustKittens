use bevy::{
    asset::ReflectAsset,
    prelude::*,
    reflect::TypeRegistry,
    window::PrimaryWindow,
    winit::cursor::{CursorIcon, CustomCursor},
};
use bevy_egui::{EguiContext, egui};

use crate::common::cursor::components::CustomCursorIcon;

use super::components::*;

pub fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}

pub fn select_resource(
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    selection: &mut InspectorSelection,
) {
    let mut resources: Vec<_> = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectResource>().is_some())
        .map(|registration| {
            (
                registration.type_info().type_path_table().short_path(),
                registration.type_id(),
            )
        })
        .collect();
    resources.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

    for (resource_name, type_id) in resources {
        let selected = match *selection {
            InspectorSelection::Resource(selected, _) => selected == type_id,
            _ => false,
        };

        if ui.selectable_label(selected, resource_name).clicked() {
            *selection = InspectorSelection::Resource(type_id, resource_name.to_string());
        }
    }
}

pub fn select_asset(
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    world: &World,
    selection: &mut InspectorSelection,
) {
    let mut assets: Vec<_> = type_registry
        .iter()
        .filter_map(|registration| {
            let reflect_asset = registration.data::<ReflectAsset>()?;
            Some((
                registration.type_info().type_path_table().short_path(),
                registration.type_id(),
                reflect_asset,
            ))
        })
        .collect();
    assets.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));

    for (asset_name, asset_type_id, reflect_asset) in assets {
        let handles: Vec<_> = reflect_asset.ids(world).collect();

        ui.collapsing(format!("{asset_name} ({})", handles.len()), |ui| {
            for handle in handles {
                let selected = match *selection {
                    InspectorSelection::Asset(_, _, selected_id) => selected_id == handle,
                    _ => false,
                };

                if ui
                    .selectable_label(selected, format!("{handle:?}"))
                    .clicked()
                {
                    *selection =
                        InspectorSelection::Asset(asset_type_id, asset_name.to_string(), handle);
                }
            }
        });
    }
}

pub fn toggle_inspector(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inspector: ResMut<InspectorEnabled>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        inspector.enabled = !inspector.enabled;
    }
}

pub fn should_show_inspector(show: Res<InspectorEnabled>) -> bool {
    show.enabled
}

pub fn force_custom_cursor_silent(
    custom_cursor_icon: Res<CustomCursorIcon>,
    mut windows: Query<&mut CursorIcon, With<Window>>,
) {
    for mut cursor_icon in windows.iter_mut() {
        // bypass_change_detection으로 Change 이벤트 없이 설정
        if !matches!(*cursor_icon.as_ref(), CursorIcon::Custom(_)) {
            *cursor_icon.bypass_change_detection() = CursorIcon::Custom(CustomCursor::Image {
                handle: custom_cursor_icon.icon.clone(),
                hotspot: (64, 64),
            });
        }
    }
}
