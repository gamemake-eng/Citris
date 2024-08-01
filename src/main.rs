use wasi_common::sync::WasiCtxBuilder;
use wasmtime::*;
use minifb::{Key, Window, WindowOptions};

mod gpu;

use gpu::{Gpu, gpu_reg};

fn main() -> anyhow::Result<()> {
    //let args: Vec<String> = env::args().collect();    


    let mut gpu = Gpu::new();

    let wasm_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/c/build/game.wasm"
    );

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    let module = Module::from_file(&engine, &wasm_path)?;
    linker.module(&mut store, "", &module)?;
    linker
        .get_default(&mut store, "")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;


    let mut window = Window::new(
        "Test",
        320,
        240,
        WindowOptions::default()
    )?;
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        gpu.regs[6] = 0xff00fff0u32 as i32;
        gpu.clear();
        gpu.regs[gpu_reg::W] = 32;
        gpu.regs[gpu_reg::H] = 32;
        gpu.regs[gpu_reg::DX] = 10;
        gpu.regs[gpu_reg::DY] = 10;
        gpu.blit();

        gpu.regs[gpu_reg::W] = 32;
        gpu.regs[gpu_reg::H] = 32;
        gpu.regs[gpu_reg::DX] = 25;
        gpu.regs[gpu_reg::DY] = 25;
        gpu.blit();

        gpu.flip();

        //x+=1;

        window.update_with_buffer(&gpu.frontbuf, 320, 240)?;
    }



    Ok(())
}
