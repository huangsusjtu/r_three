use crate::wgpu::WgpuRenderer;
use crate::{RendererInterface, Scene};
use std::sync::{Arc, RwLock};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{WindowAttributes, WindowId};

pub struct App {
    renderer: Option<WgpuRenderer>,
    scene: Arc<RwLock<Scene>>,
}

impl App {
    pub fn new() -> App {
        init_log();

        App {
            renderer: None,
            scene: Arc::new(RwLock::new(Scene::new())),
        }
    }

    pub fn scene(&self) -> Arc<RwLock<Scene>> {
        self.scene.clone()
    }

    fn render(&mut self) {
        if let Some(renderer) = self.renderer.as_mut() {
            let scene = self.scene.read().unwrap();
            renderer.render(&scene).expect("renderer panic message");
        }
    }
}

impl ApplicationHandler<crate::UserEvent> for App {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, _cause: StartCause) {
        tracing::trace!("new_events");
        if self.renderer.is_none() {
            let window = event_loop.create_window(WindowAttributes::default()).unwrap();
            self.renderer = Some(pollster::block_on(WgpuRenderer::new(Arc::new(window))));
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        tracing::trace!("resumed");
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: crate::UserEvent) {
        tracing::trace!("user_event");
        let _ = (event_loop, event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        tracing::trace!("window_event");
        match event {
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(physical_size) => {
                if physical_size.width == 0 || physical_size.height == 0 {
                    // 处理最小化窗口的事件
                    log::info!("Window minimized!");
                } else {
                    self.renderer.as_mut().unwrap().resize(physical_size);
                    // window.request_redraw();
                }
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {
                event_loop.exit();
                log::info!("Window CloseRequested");
            }
            WindowEvent::Destroyed => {
                log::info!("Window destroyed");
            }
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::PinchGesture { .. } => {}
            WindowEvent::PanGesture { .. } => {}
            WindowEvent::DoubleTapGesture { .. } => {}
            WindowEvent::RotationGesture { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {
                self.render();
                // context.update();
                // match context.renderer() {
                //     Ok(_) => {}
                //     // 当展示平面的上下文丢失，就需重新配置
                //     Err(wgpu::SurfaceError::Lost) => context.resize(context.size),
                //     // 所有其他错误（过期、超时等）应在下一帧解决
                //     Err(e) => log::error!("{e:?}"),
                // }
                // 除非我们手动请求，RedrawRequested 将只会触发一次。
                // window.request_redraw();
            }
        }
    }
}

fn start_event_loop(mut app: App, event_loop: EventLoop<crate::UserEvent>) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::EventLoopExtWebSys;
            let event_loop_function = EventLoop::spawn;
        } else {
            let event_loop_function = EventLoop::run_app::<App>;
        }
    }
    let _ = event_loop_function(event_loop, &mut app);
}

fn init_log() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            let log_level = tracing_subscriber::EnvFilter::builder().with_default_directive(tracing_core::LevelFilter::DEBUG.into()).from_env_lossy();
             let subscriber = fmt::Subscriber::builder()
                     .with_writer(std::io::stdout)
                     .with_env_filter(log_level)//.with_env_filter(EnvFilter::from_default_env())
                     .with_thread_ids(true).with_ansi(true).with_line_number(true)
                     .finish();
                   tracing::subscriber::set_global_default(subscriber).unwrap();

        }
    }
}

//
// #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
// pub async fn run() {
//     cfg_if::cfg_if! {
//         if #[cfg(target_arch = "wasm32")] {
//             std::panic::set_hook(Box::new(console_error_panic_hook::hook));
//             console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
//         } else {
//              tracing_subscriber::registry()
//                     .with(fmt::layer())
//                     .init();
//         }
//     }
//
//     #[cfg(target_arch = "wasm32")]
//     {
//         // 在网页中，需要先添加 canvas 再初始化 Context
//         use winit::platform::web::WindowExtWebSys;
//         web_sys::window()
//             .and_then(|win| win.document())
//             .map(|doc| {
//                 let canvas = window.canvas().unwrap();
//                 let mut web_width = 800.0f32;
//                 let ratio = 1.0;
//                 match doc.get_element_by_id("wasm-example") {
//                     Some(dst) => {
//                         web_width = dst.client_width() as f32;
//                         let _ = dst.append_child(&web_sys::Element::from(canvas));
//                     }
//                     None => {
//                         canvas.style().set_css_text(
//                             "background-color: black; display: block; margin: 20px auto;",
//                         );
//                         doc.body()
//                             .map(|body| body.append_child(&web_sys::Element::from(canvas)));
//                     }
//                 };
//                 // winit 0.29 开始，通过 request_inner_size, canvas.set_width 都无法设置 canvas 的大小
//                 let canvas = window.canvas().unwrap();
//                 let web_height = web_width / ratio;
//                 let scale_factor = window.scale_factor() as f32;
//                 canvas.set_width((web_width * scale_factor) as u32);
//                 canvas.set_height((web_height * scale_factor) as u32);
//                 canvas.style().set_css_text(
//                     &(canvas.style().css_text()
//                         + &format!("width: {}px; height: {}px", web_width, web_height)),
//                 );
//             })
//             .expect("Couldn't append canvas to document body.");
//
//         // 创建 Context 实例
//         let context = crate::context::Context::new(window.clone()).await;
//
//         wasm_bindgen_futures::spawn_local(async move {
//             let run_closure = Closure::once_into_js(move || {
//                 start_event_loop(context, window.clone(), event_loop)
//             });
//
//             // 处理运行过程中抛出的 JS 异常。
//             // 否则 wasm_bindgen_futures 队列将中断，且不再处理任何任务。
//             if let Err(error) = call_catch(&run_closure) {
//                 let is_control_flow_exception =
//                     error.dyn_ref::<js_sys::Error>().map_or(false, |e| {
//                         e.message().includes("Using exceptions for control flow", 0)
//                     });
//
//                 if !is_control_flow_exception {
//                     web_sys::console::error_1(&error);
//                 }
//             }
//
//             #[wasm_bindgen]
//             extern "C" {
//                 #[wasm_bindgen(catch, js_namespace = Function, js_name = "prototype.call.call")]
//                 fn call_catch(this: &JsValue) -> Result<(), JsValue>;
//             }
//         });
//     }
//
//     #[cfg(not(target_arch = "wasm32"))]
//     {
//         // 创建 app 实例
//         let mut app = App::new();
//         let event_loop = EventLoop::with_user_event().build().unwrap();
//         start_event_loop(app, event_loop);
//     }
// }
