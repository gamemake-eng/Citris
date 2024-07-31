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

pub enum GPU_REG {
    DX = 0,
    DY = 1,
    SX = 2,
    SY = 3,
    W  = 4,
    H  = 5,
    CC = 6
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
        let width = self.regs[GPU_REG::W as usize];
        let height = self.regs[GPU_REG::H as usize];
        
        let destx = self.regs[GPU_REG::DX as usize];
        let desty = self.regs[GPU_REG::DY as usize];
        
        let srcx = self.regs[GPU_REG::SX as usize];
        let srcy = self.regs[GPU_REG::SY as usize];

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
            self.pixel(destx+(i%width), desty+(i/height), *pixel);
        }

    }
    pub fn clear(&mut self){
        //println!("Not implmented");
        for pixel in self.backbuf.iter_mut() {
            *pixel = self.regs[GPU_REG::CC as usize] as u32;
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
