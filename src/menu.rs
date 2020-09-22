use crate::{
    gui::{BuildContext, Ui, UiMessage, UiNode},
    make_save_file_selector, make_scene_file_filter,
    scene::{AddNodeCommand, EditorScene, SceneCommand},
    GameEngine, Message,
};
use rg3d::{
    core::{math::vec2::Vec2, pool::Handle},
    gui::{
        file_browser::FileSelectorBuilder,
        menu::{MenuBuilder, MenuItemBuilder, MenuItemContent},
        message::{
            FileSelectorMessage, MenuItemMessage, MessageDirection, UiMessageData, WindowMessage,
        },
        widget::WidgetBuilder,
        window::{WindowBuilder, WindowTitle},
        Thickness,
    },
    renderer::surface::{Surface, SurfaceSharedData},
    scene::{
        base::BaseBuilder,
        camera::CameraBuilder,
        light::{BaseLightBuilder, PointLightBuilder, SpotLightBuilder},
        mesh::{Mesh, MeshBuilder},
        node::Node,
        particle_system::{BaseEmitterBuilder, ParticleSystemBuilder, SphereEmitterBuilder},
        sprite::SpriteBuilder,
    },
};
use std::sync::{mpsc::Sender, Arc, Mutex};

pub struct Menu {
    pub menu: Handle<UiNode>,
    new_scene: Handle<UiNode>,
    save: Handle<UiNode>,
    save_as: Handle<UiNode>,
    load: Handle<UiNode>,
    close_scene: Handle<UiNode>,
    undo: Handle<UiNode>,
    redo: Handle<UiNode>,
    create_cube: Handle<UiNode>,
    create_cone: Handle<UiNode>,
    create_sphere: Handle<UiNode>,
    create_cylinder: Handle<UiNode>,
    create_point_light: Handle<UiNode>,
    create_spot_light: Handle<UiNode>,
    exit: Handle<UiNode>,
    message_sender: Sender<Message>,
    save_file_selector: Handle<UiNode>,
    load_file_selector: Handle<UiNode>,
    create_camera: Handle<UiNode>,
    create_sprite: Handle<UiNode>,
    create_particle_system: Handle<UiNode>,
    sidebar: Handle<UiNode>,
    world_outliner: Handle<UiNode>,
    asset_browser: Handle<UiNode>,
}

pub struct MenuContext<'a, 'b> {
    pub engine: &'a mut GameEngine,
    pub editor_scene: &'b Option<EditorScene>,
    pub sidebar_window: Handle<UiNode>,
    pub world_outliner_window: Handle<UiNode>,
    pub asset_window: Handle<UiNode>,
}

fn switch_window_state(window: Handle<UiNode>, ui: &mut Ui) {
    let current_state = ui.node(window).visibility();
    ui.send_message(if current_state {
        WindowMessage::close(window, MessageDirection::ToWidget)
    } else {
        WindowMessage::open(window, MessageDirection::ToWidget)
    })
}

impl Menu {
    pub fn new(ctx: &mut BuildContext, message_sender: Sender<Message>) -> Self {
        let min_size = Vec2::new(120.0, 20.0);
        let new_scene;
        let save;
        let save_as;
        let close_scene;
        let load;
        let redo;
        let undo;
        let create_cube;
        let create_cone;
        let create_sphere;
        let create_cylinder;
        let create_point_light;
        let create_spot_light;
        let exit;
        let create_camera;
        let create_sprite;
        let create_particle_system;
        let sidebar;
        let asset_browser;
        let world_outliner;
        let menu = MenuBuilder::new(WidgetBuilder::new().on_row(0))
            .with_items(vec![
                MenuItemBuilder::new(WidgetBuilder::new().with_margin(Thickness::right(10.0)))
                    .with_content(MenuItemContent::text("File"))
                    .with_items(vec![
                        {
                            new_scene =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text_with_shortcut(
                                        "New Scene",
                                        "Ctrl+N",
                                    ))
                                    .build(ctx);
                            new_scene
                        },
                        {
                            save =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text_with_shortcut(
                                        "Save Scene",
                                        "Ctrl+S",
                                    ))
                                    .build(ctx);
                            save
                        },
                        {
                            save_as =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text_with_shortcut(
                                        "Save Scene As...",
                                        "Ctrl+Shift+S",
                                    ))
                                    .build(ctx);
                            save_as
                        },
                        {
                            load =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text_with_shortcut(
                                        "Load Scene...",
                                        "Ctrl+L",
                                    ))
                                    .build(ctx);
                            load
                        },
                        {
                            close_scene =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text_with_shortcut(
                                        "Close Scene",
                                        "Ctrl+Q",
                                    ))
                                    .build(ctx);
                            close_scene
                        },
                        {
                            exit =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text_with_shortcut(
                                        "Exit", "Alt+F4",
                                    ))
                                    .build(ctx);
                            exit
                        },
                    ])
                    .build(ctx),
                MenuItemBuilder::new(WidgetBuilder::new().with_margin(Thickness::right(10.0)))
                    .with_content(MenuItemContent::text_with_shortcut("Edit", ""))
                    .with_items(vec![
                        {
                            undo =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text_with_shortcut(
                                        "Undo", "Ctrl+Z",
                                    ))
                                    .build(ctx);
                            undo
                        },
                        {
                            redo =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text_with_shortcut(
                                        "Redo", "Ctrl+Y",
                                    ))
                                    .build(ctx);
                            redo
                        },
                    ])
                    .build(ctx),
                MenuItemBuilder::new(WidgetBuilder::new().with_margin(Thickness::right(10.0)))
                    .with_content(MenuItemContent::text_with_shortcut("Create", ""))
                    .with_items(vec![
                        MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                            .with_content(MenuItemContent::text("Mesh"))
                            .with_items(vec![
                                {
                                    create_cube = MenuItemBuilder::new(
                                        WidgetBuilder::new().with_min_size(min_size),
                                    )
                                    .with_content(MenuItemContent::text("Cube"))
                                    .build(ctx);
                                    create_cube
                                },
                                {
                                    create_sphere = MenuItemBuilder::new(
                                        WidgetBuilder::new().with_min_size(min_size),
                                    )
                                    .with_content(MenuItemContent::text("Sphere"))
                                    .build(ctx);
                                    create_sphere
                                },
                                {
                                    create_cylinder = MenuItemBuilder::new(
                                        WidgetBuilder::new().with_min_size(min_size),
                                    )
                                    .with_content(MenuItemContent::text("Cylinder"))
                                    .build(ctx);
                                    create_cylinder
                                },
                                {
                                    create_cone = MenuItemBuilder::new(
                                        WidgetBuilder::new().with_min_size(min_size),
                                    )
                                    .with_content(MenuItemContent::text("Cone"))
                                    .build(ctx);
                                    create_cone
                                },
                            ])
                            .build(ctx),
                        MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                            .with_content(MenuItemContent::text("Light"))
                            .with_items(vec![
                                {
                                    create_spot_light = MenuItemBuilder::new(
                                        WidgetBuilder::new().with_min_size(min_size),
                                    )
                                    .with_content(MenuItemContent::text("Spot Light"))
                                    .build(ctx);
                                    create_spot_light
                                },
                                {
                                    create_point_light = MenuItemBuilder::new(
                                        WidgetBuilder::new().with_min_size(min_size),
                                    )
                                    .with_content(MenuItemContent::text("Point Light"))
                                    .build(ctx);
                                    create_point_light
                                },
                            ])
                            .build(ctx),
                        {
                            create_camera =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text("Camera"))
                                    .build(ctx);
                            create_camera
                        },
                        {
                            create_sprite =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text("Sprite"))
                                    .build(ctx);
                            create_sprite
                        },
                        {
                            create_particle_system =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text("Particle System"))
                                    .build(ctx);
                            create_particle_system
                        },
                    ])
                    .build(ctx),
                MenuItemBuilder::new(WidgetBuilder::new().with_margin(Thickness::right(10.0)))
                    .with_content(MenuItemContent::text_with_shortcut("View", ""))
                    .with_items(vec![
                        {
                            sidebar =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text("Sidebar"))
                                    .build(ctx);
                            sidebar
                        },
                        {
                            asset_browser =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text("Asset Browser"))
                                    .build(ctx);
                            asset_browser
                        },
                        {
                            world_outliner =
                                MenuItemBuilder::new(WidgetBuilder::new().with_min_size(min_size))
                                    .with_content(MenuItemContent::text("World Outliner"))
                                    .build(ctx);
                            world_outliner
                        },
                    ])
                    .build(ctx),
            ])
            .build(ctx);

        let save_file_selector = make_save_file_selector(ctx);

        let load_file_selector = FileSelectorBuilder::new(
            WindowBuilder::new(WidgetBuilder::new().with_width(300.0).with_height(400.0))
                .with_title(WindowTitle::Text("Select a Scene to Load".into()))
                .open(false),
        )
        .with_path("./")
        .with_filter(make_scene_file_filter())
        .build(ctx);

        Self {
            menu,
            new_scene,
            save,
            save_as,
            close_scene,
            load,
            undo,
            redo,
            create_cube,
            create_cone,
            create_sphere,
            create_cylinder,
            create_point_light,
            create_spot_light,
            exit,
            message_sender,
            save_file_selector,
            load_file_selector,
            create_camera,
            create_sprite,
            create_particle_system,
            sidebar,
            world_outliner,
            asset_browser,
        }
    }

    pub fn handle_message(&mut self, message: &UiMessage, ctx: MenuContext) {
        match &message.data() {
            UiMessageData::FileSelector(msg) => match msg {
                FileSelectorMessage::Commit(path) => {
                    if message.destination() == self.save_file_selector {
                        self.message_sender
                            .send(Message::SaveScene(path.to_owned()))
                            .unwrap();
                    } else {
                        self.message_sender
                            .send(Message::LoadScene(path.to_owned()))
                            .unwrap();
                    }
                }
                _ => (),
            },
            UiMessageData::MenuItem(msg) => {
                if let MenuItemMessage::Click = msg {
                    if message.destination() == self.create_cube {
                        let mut mesh = Mesh::default();
                        mesh.set_name("Cube");
                        mesh.add_surface(Surface::new(Arc::new(Mutex::new(
                            SurfaceSharedData::make_cube(Default::default()),
                        ))));
                        let node = Node::Mesh(mesh);
                        self.message_sender
                            .send(Message::DoSceneCommand(SceneCommand::AddNode(
                                AddNodeCommand::new(node),
                            )))
                            .unwrap();
                    } else if message.destination() == self.create_spot_light {
                        let node = SpotLightBuilder::new(BaseLightBuilder::new(
                            BaseBuilder::new().with_name("SpotLight"),
                        ))
                        .with_distance(10.0)
                        .with_hotspot_cone_angle(45.0f32.to_radians())
                        .with_falloff_angle_delta(2.0f32.to_radians())
                        .build_node();

                        self.message_sender
                            .send(Message::DoSceneCommand(SceneCommand::AddNode(
                                AddNodeCommand::new(node),
                            )))
                            .unwrap();
                    } else if message.destination() == self.create_point_light {
                        let node = PointLightBuilder::new(BaseLightBuilder::new(
                            BaseBuilder::new().with_name("PointLight"),
                        ))
                        .with_radius(10.0)
                        .build_node();

                        self.message_sender
                            .send(Message::DoSceneCommand(SceneCommand::AddNode(
                                AddNodeCommand::new(node),
                            )))
                            .unwrap();
                    } else if message.destination() == self.create_cone {
                        let mesh = MeshBuilder::new(BaseBuilder::new().with_name("Cone"))
                            .with_surfaces(vec![Surface::new(Arc::new(Mutex::new(
                                SurfaceSharedData::make_cone(16, 1.0, 1.0, Default::default()),
                            )))])
                            .build_node();
                        self.message_sender
                            .send(Message::DoSceneCommand(SceneCommand::AddNode(
                                AddNodeCommand::new(mesh),
                            )))
                            .unwrap();
                    } else if message.destination() == self.create_cylinder {
                        let mesh = MeshBuilder::new(BaseBuilder::new().with_name("Cylinder"))
                            .with_surfaces(vec![Surface::new(Arc::new(Mutex::new(
                                SurfaceSharedData::make_cylinder(
                                    16,
                                    1.0,
                                    1.0,
                                    true,
                                    Default::default(),
                                ),
                            )))])
                            .build_node();
                        self.message_sender
                            .send(Message::DoSceneCommand(SceneCommand::AddNode(
                                AddNodeCommand::new(mesh),
                            )))
                            .unwrap();
                    } else if message.destination() == self.create_sphere {
                        let mesh = MeshBuilder::new(BaseBuilder::new().with_name("Sphere"))
                            .with_surfaces(vec![Surface::new(Arc::new(Mutex::new(
                                SurfaceSharedData::make_sphere(16, 16, 1.0),
                            )))])
                            .build_node();
                        self.message_sender
                            .send(Message::DoSceneCommand(SceneCommand::AddNode(
                                AddNodeCommand::new(mesh),
                            )))
                            .unwrap();
                    } else if message.destination() == self.create_camera {
                        let node =
                            CameraBuilder::new(BaseBuilder::new().with_name("Camera")).build_node();

                        self.message_sender
                            .send(Message::DoSceneCommand(SceneCommand::AddNode(
                                AddNodeCommand::new(node),
                            )))
                            .unwrap();
                    } else if message.destination() == self.create_sprite {
                        let node =
                            SpriteBuilder::new(BaseBuilder::new().with_name("Sprite")).build_node();

                        self.message_sender
                            .send(Message::DoSceneCommand(SceneCommand::AddNode(
                                AddNodeCommand::new(node),
                            )))
                            .unwrap();
                    } else if message.destination() == self.create_particle_system {
                        let node = ParticleSystemBuilder::new(
                            BaseBuilder::new().with_name("ParticleSystem"),
                        )
                        .with_emitters(vec![SphereEmitterBuilder::new(
                            BaseEmitterBuilder::new()
                                .with_max_particles(100)
                                .resurrect_particles(true),
                        )
                        .with_radius(1.0)
                        .build()])
                        .build_node();

                        self.message_sender
                            .send(Message::DoSceneCommand(SceneCommand::AddNode(
                                AddNodeCommand::new(node),
                            )))
                            .unwrap();
                    } else if message.destination() == self.save {
                        if let Some(scene_path) =
                            ctx.editor_scene.as_ref().map(|s| s.path.as_ref()).flatten()
                        {
                            self.message_sender
                                .send(Message::SaveScene(scene_path.clone()))
                                .unwrap();
                        } else {
                            // If scene wasn't saved yet - open Save As window.
                            ctx.engine
                                .user_interface
                                .send_message(WindowMessage::open_modal(
                                    self.save_file_selector,
                                    MessageDirection::ToWidget,
                                ));
                        }
                    } else if message.destination() == self.save_as {
                        ctx.engine
                            .user_interface
                            .send_message(WindowMessage::open_modal(
                                self.save_file_selector,
                                MessageDirection::ToWidget,
                            ));
                    } else if message.destination() == self.load {
                        ctx.engine
                            .user_interface
                            .send_message(WindowMessage::open_modal(
                                self.load_file_selector,
                                MessageDirection::ToWidget,
                            ));
                    } else if message.destination() == self.close_scene {
                        self.message_sender.send(Message::CloseScene).unwrap();
                    } else if message.destination() == self.undo {
                        self.message_sender.send(Message::UndoSceneCommand).unwrap();
                    } else if message.destination() == self.redo {
                        self.message_sender.send(Message::RedoSceneCommand).unwrap();
                    } else if message.destination() == self.exit {
                        self.message_sender
                            .send(Message::Exit { force: false })
                            .unwrap();
                    } else if message.destination() == self.new_scene {
                        self.message_sender.send(Message::NewScene).unwrap();
                    } else if message.destination() == self.asset_browser {
                        switch_window_state(ctx.asset_window, &mut ctx.engine.user_interface);
                    } else if message.destination() == self.world_outliner {
                        switch_window_state(
                            ctx.world_outliner_window,
                            &mut ctx.engine.user_interface,
                        );
                    } else if message.destination() == self.sidebar {
                        switch_window_state(ctx.sidebar_window, &mut ctx.engine.user_interface);
                    }
                }
            }
            _ => (),
        }
    }
}