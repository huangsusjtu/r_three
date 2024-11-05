use glam::Vec3;
use r_three::camera::OrthographicCamera;
use r_three::color::Color;
use r_three::{Object3D, Scene};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use winit::event_loop::EventLoop;
pub mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let scene = Arc::new(RwLock::new(Scene::new()));
    let d_scene = scene.clone();
    tokio::spawn(async move { init_scene(d_scene) });

    // 主线程事件循环
    // 创建 app 实例
    let mut app = app::App::new();
    app.attach_scene(
        scene,
        Box::new(OrthographicCamera::new(
            Vec3::new(0.0, 0.0, 10.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.1,
            1000.0,
            30.0,
        )),
    );

    let event_loop = EventLoop::with_user_event().build()?;
    let _ = EventLoop::run_app::<app::App>(event_loop, &mut app);
    Ok(())
}


fn init_scene(scene_mut_ref: Arc<RwLock<Scene>>)  {
    let mut scene = scene_mut_ref.write().unwrap();
    scene.set_clear_color(Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 0.0,
    });

    let triangle_obj: Rc<RefCell<Box<dyn Object3D>>> = {
        let mut geo = r_three::geometries::ShapeGeometry::new();
        geo.add(Vec3::new(0.0, 0.49240386, 0.0));
        geo.add(Vec3::new(-0.49513406, 0., 0.0));
        geo.add(Vec3::new(0.21918549, 0.0, 0.0));


        let material = r_three::materials::MeshBasicMaterial {
            color: r_three::color::Color::from([0.9, 0.0, 0.0]),
        };
        r_three::objects::Mesh::new(Box::new(geo), material).to_object()
    };
    scene.add(triangle_obj);
}


// fn init_scene(scene_mut_ref: Arc<RwLock<Scene>>)  {
//     let mut scene = scene_mut_ref.write().unwrap();
//     scene.set_clear_color(Color {
//         r: 0.1,
//         g: 0.2,
//         b: 0.3,
//         a: 1.0,
//     });
//
//     let line_obj: Rc<RefCell<Box<dyn Object3D>>> = {
//         let mut geo = r_three::BufferGeometry::new();
//         geo.set_from_points(vec![
//             Vec3::new(0.0, 0.0, 0.0),
//             Vec3::new(1.0, 1.0, 0.0),
//             Vec3::new(0.0, 2.0, 0.0),
//         ]);
//         let material = r_three::materials::LineBasicMaterial {
//             color: r_three::color::Color::from([1.0, 1.0, 1.0]),
//             line_width: 1.0,
//             linecap: "round",
//             linejoin: "round",
//             alpha: 1.0,
//         };
//         r_three::objects::Line::new(geo, material).to_object()
//     };
//     scene.add(line_obj);
//
//     let group_obj: Rc<RefCell<Box<dyn Object3D>>> = {
//         let geo = r_three::geometries::BoxGeometry::new(1.0, 1.0, 1.0);
//         let material = r_three::materials::MeshBasicMaterial {
//             color: r_three::color::Color::from([1.0, 0.0, 0.0]),
//         };
//         r_three::objects::Mesh::new(Box::new(geo), material).to_object()
//     };
//     scene.add(group_obj);
// }
