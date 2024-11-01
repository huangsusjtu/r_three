use glam::Vec3;
use r_three::color::Color;
use r_three::Object3D;
use std::cell::RefCell;
use std::rc::Rc;
use winit::event_loop::EventLoop;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建 app 实例
    let mut app = r_three::app::App::new();
    let scene = app.scene();
    tokio::spawn(async move {
        let mut scene = scene.write().unwrap();
        scene.set_clear_color(Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        });

        let line_obj: Rc<RefCell<Box<dyn Object3D>>> = {
            let mut geo = r_three::BufferGeometry::new();
            geo.set_from_points(vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(0.0, 2.0, 0.0),
            ]);
            let material = r_three::materials::LineBasicMaterial {
                color: r_three::color::Color::from([1.0, 1.0, 1.0]),
                line_width: 1.0,
                linecap: "round",
                linejoin: "round",
                alpha: 1.0,
            };
            r_three::objects::Line::new(geo, material).as_object()
        };
        scene.add(line_obj);

        let group_obj: Rc<RefCell<Box<dyn Object3D>>> = {
            let mut geo = r_three::geometries::BoxGeometry::new(1.0, 1.0, 1.0);
            let material = r_three::materials::MeshBasicMaterial {
                color: r_three::color::Color::from([1.0, 0.0, 0.0]),
            };
            r_three::objects::Mesh::new(Box::new(geo), material).as_object()
        };
        scene.add(group_obj);
    });

    // 主线程事件循环
    let event_loop = EventLoop::with_user_event().build()?;
    let _ = EventLoop::run_app::<r_three::app::App>(event_loop, &mut app);
    Ok(())
}
