
use wasi_common::sync::WasiCtxBuilder;
use wasi_common::WasiCtx;
use wasmtime::*;

use minifb::{Key, Window, WindowOptions};

use std::sync::{Arc, Mutex};

mod gpu;

use gpu::{Gpu, gpu_reg};

struct Env<'a> {
    gpu:  &'a mut Gpu,
    wasi: WasiCtx
}

fn main() -> anyhow::Result<()> {
    //let args: Vec<String> = env::args().collect();    
    
    let mut gpu = Gpu::new();

    let wasm_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/c/build/game.wasm"
    );

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    
    linker.func_wrap("env", "gpu_cmd", |caller: Caller<'_, Env>, cmd: u32| {
        caller.data().gpu.cmd(cmd);
    })?;

    wasi_common::sync::add_to_linker(&mut linker, |s: &mut Env| &mut s.wasi)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    let  env = Env {
        gpu: &mut gpu,
        wasi: wasi
    };

    let mut store = Store::new(&engine, env);

    let module = Module::from_file(&engine, wasm_path)?;
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
    ).unwrap();
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        //x+=1;

        window.update_with_buffer(&gpu.frontbuf, 320, 240)?;
    }

    Ok(())
}
