use wasmer::{Store, Module, Instance, Value, imports};
use minifb::{Key, Window, WindowOptions};

mod gpu;

use gpu::{Gpu, gpu_reg};

fn main() -> anyhow::Result<()> {
	
	let mut gpu = Gpu::new();

	let module_wat = r#"
	(module
	(type $t0 (func (param i32) (result i32)))
	(func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
			get_local $p0
			i32.const 1
			i32.add))
	"#;

	let mut store = Store::default();
	let module = Module::new(&store, &module_wat)?;
	// The module doesn't import anything, so we create an empty import object.
	let import_object = imports! {};
	let instance = Instance::new(&mut store, &module, &import_object)?;
	
        
        let mut x = -5;

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
            
            x+=1;

	    window.update_with_buffer(&gpu.frontbuf, 320, 240)?;
    	}



    	Ok(())
}
