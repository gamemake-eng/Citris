pub struct Gpu {
    pub frontbuf: Vec<u32>,
    pub backbuf: Vec<u32>,
    pub texbuf: Vec<u32>,
    pub regs: Vec<i32>
}

/*
 * regs
#define DX 0
#define DY 1
#define SX 2
#define SY 3
#define W 4
#define H 5
#define CC 6
*/

pub mod GPU_REG {
    pub const DX: usize  = 0;
    pub const DY: usize  = 1;
    pub const SX: usize  = 2;
    pub const SY: usize  = 3;
    pub const W: usize   = 4;
    pub const H: usize   = 5;
    pub const CC: usize  = 6;
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu{
            frontbuf: vec![0u32; 320*240],
            backbuf: vec![0u32; 320*240],
            texbuf: vec![0u32; 1024*1024],
            regs: vec![0i32; 7]

        }
    }
    pub fn pixel(&mut self, x: i32, y: i32, col: u32){
        self.backbuf[(x + 320 * y) as usize] = col;
    }
    pub fn blit(&mut self){
        let width = self.regs[GPU_REG::W];
        let height = self.regs[GPU_REG::H];
        
        let destx = self.regs[GPU_REG::DX];
        let desty = self.regs[GPU_REG::DY];
        
        let srcx = self.regs[GPU_REG::SX];
        let srcy = self.regs[GPU_REG::SY];

        let mut buf = vec![0u32; (width*height) as usize];
        
        
        //First blit to the temp buffer
        for (index, pixel) in buf.iter_mut().enumerate() {
            let mut x = (index as i32) % 1024;
            let mut y = (index as i32) / 1024;
            x += srcx;
            y += srcy;
            *pixel = self.texbuf[(x + 1024 * y) as usize];
        }

        //Then, blit to the back buffer
        for (index, pixel) in buf.iter_mut().enumerate() {
            let i = index as i32;
            let dx = destx+(i%width);
            let dy = desty+(i/height);
            
            let cangox = (dx >= 0) && (dx <= 320);
            let cangoy = (dy >= 0) && (dy <= 320);
            
            if cangox && cangoy {
                self.pixel(dx, dy, *pixel);
            }
        }

    }
    pub fn clear(&mut self){
        //println!("Not implmented");
        for pixel in self.backbuf.iter_mut() {
            *pixel = self.regs[GPU_REG::CC] as u32;
        }
    }
    pub fn flip(&mut self){
        for (index, pixel) in self.backbuf.iter_mut().enumerate(){
            //println!("index {index} pixel {pixel}");
            self.frontbuf[index] = *pixel;
        }

        /*for (index, pixel) in self.frontbuf.iter_mut().enumerate(){     
            println!("index {index} pixel {pixel}");
        }*/ 



    }
}
