use bevy::{asset::ReflectAsset, prelude::*, reflect::TypeRegistry, window::PrimaryWindow};
use bevy_egui::{EguiContext, EguiPostUpdateSet, egui};
use bevy_inspector_egui::bevy_inspector::{
    self,
    hierarchy::{SelectedEntities, hierarchy_ui},
    ui_for_entities_shared_components, ui_for_entity_with_children,
};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

use super::components::*;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiState::new()).add_systems(
            PostUpdate,
            show_ui_system
                .before(EguiPostUpdateSet::ProcessOutput)
                .before(bevy_egui::end_pass_system)
                .before(bevy::transform::TransformSystem::TransformPropagate),
        );
    }
}

impl UiState {
    pub fn new() -> Self {
        let mut state = DockState::new(vec![EguiWindow::GameView]);
        let tree = state.main_surface_mut();
        let [game, _inspector] =
            tree.split_right(NodeIndex::root(), 0.8, vec![EguiWindow::Inspector]);
        let [game, _hierarchy] = tree.split_left(game, 0.2, vec![EguiWindow::Hierarchy]);
        let [_game, _bottom] =
            tree.split_below(game, 0.8, vec![EguiWindow::Resources, EguiWindow::Assets]);

        Self {
            state,
            selected_entities: SelectedEntities::default(),
            selection: InspectorSelection::Entities,
            viewport_rect: egui::Rect::NOTHING,
        }
    }

    pub fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
            selection: &mut self.selection,
        };
        DockArea::new(&mut self.state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        match window {
            EguiWindow::GameView => {
                *self.viewport_rect = ui.clip_rect();
            }
            EguiWindow::Hierarchy => {
                let selected = hierarchy_ui(self.world, ui, self.selected_entities);
                if selected {
                    *self.selection = InspectorSelection::Entities;
                }
            }
            EguiWindow::Resources => select_resource(ui, &type_registry, self.selection),
            EguiWindow::Assets => select_asset(ui, &type_registry, self.world, self.selection),
            EguiWindow::Inspector => match *self.selection {
                InspectorSelection::Entities => match self.selected_entities.as_slice() {
                    &[entity] => ui_for_entity_with_children(self.world, entity, ui),
                    entities => ui_for_entities_shared_components(self.world, entities, ui),
                },
                InspectorSelection::Resource(type_id, ref name) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_resource(
                        self.world,
                        type_id,
                        ui,
                        name,
                        &type_registry,
                    )
                }
                InspectorSelection::Asset(type_id, ref name, handle) => {
                    ui.label(name);
                    bevy_inspector::by_type_id::ui_for_asset(
                        self.world,
                        type_id,
                        handle,
                        ui,
                        &type_registry,
                    );
                }
            },
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, EguiWindow::GameView)
    }
}

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

fn select_resource(
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

fn select_asset(
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
                    .selectable_label(selected, format!("{:?}", handle))
                    .clicked()
                {
                    *selection =
                        InspectorSelection::Asset(asset_type_id, asset_name.to_string(), handle);
                }
            }
        });
    }
}
