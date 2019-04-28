use direct2d::render_target::RenderTarget;
use kurbo::Rect;
use std::path::Path;

use piet::RenderContext;

use winit::os::windows::WindowExt;

fn main() {
    let (width, height) = (1164, 853);
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_dimensions((width, height).into())
        .build(&events_loop)
        .unwrap();

    let d2d_factory = direct2d::Factory::new().unwrap();
    let dw_factory = directwrite::Factory::new().unwrap();
    let (_feature_level, d3d11_device, d3d11_context) = direct3d11::device::Device::create()
        .with_flags(direct3d11::flags::CreateDeviceFlags::BGRA_SUPPORT)
        .build()
        .unwrap();

    let d2d_device = direct2d::Device::create(&d2d_factory, &d3d11_device.as_dxgi()).unwrap();
    let mut d2d_context = direct2d::DeviceContext::create(&d2d_device, false).unwrap();

    let dxgi_factory = d3d11_device.as_dxgi().get_adapter().unwrap().get_factory();
    let swapchain = dxgi_factory
        .create_swapchain_for_hwnd(&d3d11_device.as_dxgi())
        .hwnd(window.get_hwnd() as *mut _)
        .build()
        .unwrap();

    let backbuffer: direct3d11::Texture2D = swapchain.get_buffer(0).unwrap();
    let target = direct2d::image::Bitmap::create(&d2d_context)
        .with_dxgi_surface(&backbuffer.as_dxgi())
        .with_options(direct2d::enums::BitmapOptions::NONE)
        .build()
        .unwrap();
    d2d_context.set_target(&target);
    let mut rt = d2d_context.clone();
    let mut ctxt = piet_common::Piet::new(&d2d_factory, &dw_factory, &mut d2d_context);

    let img = image::open(&Path::new("background.png")).unwrap().to_rgba();
    let img_width = img.width();
    let img_height = img.height();
    let img_data = img.into_raw();

    let sample_bitmap = ctxt
        .make_image(
            img_width as _,
            img_height as _,
            &img_data,
            piet::ImageFormat::RgbaSeparate,
        )
        .unwrap();

    let mut stop = false;
    while !stop {
        events_loop.poll_events(|event| match event {
            winit::Event::WindowEvent { event, .. } => match event {
                winit::WindowEvent::CloseRequested => stop = true,
                _ => (),
            },
            _ => (),
        });

        rt.begin_draw();

        {
            ctxt.draw_image(
                &sample_bitmap,
                Rect::new(0.0, 0.0, width as _, height as _),
                piet::InterpolationMode::Bilinear,
            )
        }

        rt.end_draw();

        swapchain.present(1, dxgi::PresentFlags::NONE);
    }
}
