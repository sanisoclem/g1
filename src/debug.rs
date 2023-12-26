use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::{
  bevy_inspector::hierarchy::SelectedEntities, DefaultInspectorConfigPlugin,
};

pub fn inspector_ui(world: &mut World, mut selected_entities: Local<SelectedEntities>) {
  let mut egui_context = world
    .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
    .single(world)
    .clone();
  egui::SidePanel::left("hierarchy")
    .default_width(200.0)
    .show(egui_context.get_mut(), |ui| {
      egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("Hierarchy");

        bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
          world,
          ui,
          &mut selected_entities,
        );

        ui.label("Press escape to toggle UI");
        ui.allocate_space(ui.available_size());
      });
    });

  egui::SidePanel::right("inspector")
    .default_width(250.0)
    .show(egui_context.get_mut(), |ui| {
      egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("Inspector");

        match selected_entities.as_slice() {
          &[entity] => {
            bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
          }
          entities => {
            bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
              world, entities, ui,
            );
          }
        }

        ui.allocate_space(ui.available_size());
      });
    });
}
