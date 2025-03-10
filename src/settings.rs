use crate::{
    gui::{BuildContext, Ui, UiMessage, UiNode},
    scene::EditorScene,
    GameEngine, Message,
};
use rg3d::{
    core::{pool::Handle, scope_profile},
    gui::{
        button::ButtonBuilder,
        check_box::CheckBoxBuilder,
        color::ColorFieldBuilder,
        grid::{Column, GridBuilder, Row},
        message::{
            ButtonMessage, CheckBoxMessage, ColorFieldMessage, MessageDirection,
            NumericUpDownMessage, UiMessageData, WindowMessage,
        },
        numeric::NumericUpDownBuilder,
        stack_panel::StackPanelBuilder,
        text::TextBuilder,
        widget::WidgetBuilder,
        window::{WindowBuilder, WindowTitle},
        HorizontalAlignment, Orientation, Thickness, VerticalAlignment,
    },
    scene::camera::Camera,
};
use std::sync::mpsc::Sender;

pub struct Settings {
    window: Handle<UiNode>,
    ssao: Handle<UiNode>,
    point_shadows: Handle<UiNode>,
    spot_shadows: Handle<UiNode>,
    ok: Handle<UiNode>,
    default: Handle<UiNode>,
    sender: Sender<Message>,
    ambient_color: Handle<UiNode>,
    light_scatter: Handle<UiNode>,
    near_plane: Handle<UiNode>,
    far_plane: Handle<UiNode>,
}

fn make_text_mark(ctx: &mut BuildContext, text: &str, row: usize) -> Handle<UiNode> {
    TextBuilder::new(
        WidgetBuilder::new()
            .with_vertical_alignment(VerticalAlignment::Center)
            .with_margin(Thickness::left(4.0))
            .on_row(row)
            .on_column(0),
    )
    .with_text(text)
    .build(ctx)
}

fn make_bool_input_field(ctx: &mut BuildContext, row: usize, value: bool) -> Handle<UiNode> {
    CheckBoxBuilder::new(
        WidgetBuilder::new()
            .on_row(row)
            .with_margin(Thickness::uniform(1.0))
            .on_column(1),
    )
    .checked(Some(value))
    .build(ctx)
}

impl Settings {
    pub fn new(engine: &mut GameEngine, sender: Sender<Message>) -> Self {
        let ssao;
        let ok;
        let default;
        let ambient_color;
        let point_shadows;
        let spot_shadows;
        let light_scatter;
        let near_plane;
        let far_plane;
        let ctx = &mut engine.user_interface.build_ctx();
        let settings = engine.renderer.get_quality_settings();
        let text =
            "Here you can select graphics settings to improve performance and/or to understand how \
            you scene will look like with different graphics settings. Please note that these settings won't be saved \
            with scene!";
        let window = WindowBuilder::new(WidgetBuilder::new().with_width(300.0).with_height(400.0))
            .open(false)
            .with_title(WindowTitle::Text("Settings".to_owned()))
            .with_content(
                GridBuilder::new(
                    WidgetBuilder::new()
                        .with_child(
                            TextBuilder::new(
                                WidgetBuilder::new()
                                    .on_row(0)
                                    .with_margin(Thickness::uniform(1.0)),
                            )
                            .with_text(text)
                            .with_wrap(true)
                            .build(ctx),
                        )
                        .with_child(
                            GridBuilder::new(
                                WidgetBuilder::new()
                                    .on_row(1)
                                    .with_child(make_text_mark(ctx, "SSAO", 0))
                                    .with_child({
                                        ssao = make_bool_input_field(ctx, 0, settings.use_ssao);
                                        ssao
                                    })
                                    .with_child(make_text_mark(ctx, "Ambient Color", 1))
                                    .with_child({
                                        ambient_color = ColorFieldBuilder::new(
                                            WidgetBuilder::new().on_column(1).on_row(1),
                                        )
                                        .build(ctx);
                                        ambient_color
                                    })
                                    .with_child(make_text_mark(ctx, "Point Shadows", 2))
                                    .with_child({
                                        point_shadows = make_bool_input_field(
                                            ctx,
                                            2,
                                            settings.point_shadows_enabled,
                                        );
                                        point_shadows
                                    })
                                    .with_child(make_text_mark(ctx, "Spot Shadows", 3))
                                    .with_child({
                                        spot_shadows = make_bool_input_field(
                                            ctx,
                                            3,
                                            settings.spot_shadows_enabled,
                                        );
                                        spot_shadows
                                    })
                                    .with_child(make_text_mark(ctx, "Light Scatter", 4))
                                    .with_child({
                                        light_scatter = make_bool_input_field(
                                            ctx,
                                            4,
                                            settings.light_scatter_enabled,
                                        );
                                        light_scatter
                                    })
                                    .with_child(make_text_mark(ctx, "Near Plane", 5))
                                    .with_child({
                                        near_plane = NumericUpDownBuilder::new(
                                            WidgetBuilder::new()
                                                .on_column(1)
                                                .on_row(5)
                                                .with_margin(Thickness::uniform(1.0)),
                                        )
                                        .build(ctx);
                                        near_plane
                                    })
                                    .with_child(make_text_mark(ctx, "Far Plane", 6))
                                    .with_child({
                                        far_plane = NumericUpDownBuilder::new(
                                            WidgetBuilder::new()
                                                .on_column(1)
                                                .on_row(6)
                                                .with_margin(Thickness::uniform(1.0)),
                                        )
                                        .build(ctx);
                                        far_plane
                                    }),
                            )
                            .add_row(Row::strict(25.0))
                            .add_row(Row::strict(25.0))
                            .add_row(Row::strict(25.0))
                            .add_row(Row::strict(25.0))
                            .add_row(Row::strict(25.0))
                            .add_row(Row::strict(25.0))
                            .add_row(Row::strict(25.0))
                            .add_row(Row::stretch())
                            .add_row(Row::stretch())
                            .add_column(Column::strict(100.0))
                            .add_column(Column::stretch())
                            .build(ctx),
                        )
                        .with_child(
                            StackPanelBuilder::new(
                                WidgetBuilder::new()
                                    .on_row(2)
                                    .with_horizontal_alignment(HorizontalAlignment::Right)
                                    .with_child({
                                        default = ButtonBuilder::new(
                                            WidgetBuilder::new()
                                                .with_width(80.0)
                                                .with_margin(Thickness::uniform(1.0)),
                                        )
                                        .with_text("Default")
                                        .build(ctx);
                                        default
                                    })
                                    .with_child({
                                        ok = ButtonBuilder::new(
                                            WidgetBuilder::new()
                                                .with_width(80.0)
                                                .with_margin(Thickness::uniform(1.0)),
                                        )
                                        .with_text("OK")
                                        .build(ctx);
                                        ok
                                    }),
                            )
                            .with_orientation(Orientation::Horizontal)
                            .build(ctx),
                        ),
                )
                .add_row(Row::auto())
                .add_row(Row::stretch())
                .add_row(Row::strict(25.0))
                .add_column(Column::stretch())
                .build(ctx),
            )
            .build(ctx);

        Self {
            window,
            ssao,
            sender,
            ok,
            default,
            ambient_color,
            point_shadows,
            spot_shadows,
            light_scatter,
            near_plane,
            far_plane,
        }
    }

    pub fn open(&self, ui: &Ui, camera: &Camera) {
        ui.send_message(WindowMessage::open(
            self.window,
            MessageDirection::ToWidget,
            true,
        ));
        ui.send_message(NumericUpDownMessage::value(
            self.near_plane,
            MessageDirection::ToWidget,
            camera.z_near(),
        ));
        ui.send_message(NumericUpDownMessage::value(
            self.far_plane,
            MessageDirection::ToWidget,
            camera.z_far(),
        ));
    }

    pub fn handle_message(
        &mut self,
        message: &UiMessage,
        editor_scene: &EditorScene,
        engine: &mut GameEngine,
    ) {
        scope_profile!();

        let mut settings = engine.renderer.get_quality_settings();

        match message.data() {
            UiMessageData::CheckBox(CheckBoxMessage::Check(check)) => {
                let value = check.unwrap_or(false);
                if message.destination() == self.ssao {
                    settings.use_ssao = value;
                } else if message.destination() == self.point_shadows {
                    settings.point_shadows_enabled = value;
                } else if message.destination() == self.spot_shadows {
                    settings.spot_shadows_enabled = value;
                } else if message.destination() == self.light_scatter {
                    settings.light_scatter_enabled = value;
                }
            }
            UiMessageData::ColorField(msg)
                if message.direction() == MessageDirection::FromWidget =>
            {
                if message.destination() == self.ambient_color {
                    if let ColorFieldMessage::Color(color) = *msg {
                        engine.scenes[editor_scene.scene].ambient_lighting_color = color;
                    }
                }
            }
            UiMessageData::Button(ButtonMessage::Click) => {
                if message.destination() == self.ok {
                    engine.user_interface.send_message(WindowMessage::close(
                        self.window,
                        MessageDirection::ToWidget,
                    ));
                } else if message.destination() == self.default {
                    settings = Default::default();

                    let sync_check_box = |handle: Handle<UiNode>, value: bool| {
                        engine.user_interface.send_message(CheckBoxMessage::checked(
                            handle,
                            MessageDirection::ToWidget,
                            Some(value),
                        ));
                    };

                    sync_check_box(self.ssao, settings.use_ssao);
                    sync_check_box(self.point_shadows, settings.point_shadows_enabled);
                    sync_check_box(self.spot_shadows, settings.spot_shadows_enabled);
                    sync_check_box(self.light_scatter, settings.light_scatter_enabled);
                }
            }
            UiMessageData::NumericUpDown(NumericUpDownMessage::Value(value)) => {
                let camera = engine.scenes[editor_scene.scene].graph
                    [editor_scene.camera_controller.camera]
                    .as_camera_mut();
                if message.destination() == self.near_plane {
                    camera.set_z_near(*value);
                } else if message.destination() == self.far_plane {
                    camera.set_z_far(*value);
                }
            }
            _ => {}
        }

        if settings != engine.renderer.get_quality_settings() {
            if let Err(e) = engine.renderer.set_quality_settings(&settings) {
                self.sender
                    .send(Message::Log(format!(
                        "An error occurred at attempt to set new graphics settings: {:?}",
                        e
                    )))
                    .unwrap();
            } else {
                self.sender
                    .send(Message::Log(
                        "New graphics quality settings were successfully set!".to_owned(),
                    ))
                    .unwrap();
            }
        }
    }
}
