// use bev

// pub fn update_visualizer_system(
//     mut egui_contexts: EguiContexts,
//     mut visualizer: ResMut<RenetClientVisualizer<200>>,
//     client: Res<RenetClient>,
//     mut show_visualizer: Local<bool>,
//     keyboard_input: Res<Input<KeyCode>>,
// ) {
//     visualizer.add_network_info(client.network_info());
//     if keyboard_input.just_pressed(KeyCode::F1) {
//         *show_visualizer = !*show_visualizer;
//     }
//     if *show_visualizer {
//         visualizer.show_window(egui_contexts.ctx_mut());
//     }
// }
