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

pub mod gpu_reg {
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
            texbuf: vec![0x50ff0000u32; 1024*1024],
            regs: vec![0i32; 7]

        }
    }
    pub fn pixel(&mut self, x: i32, y: i32, col: u32){
        self.backbuf[(x + 320 * y) as usize] = col;
    }
    pub fn get_pixel(&mut self, x: i32, y: i32) -> u32 {
        let cangox = (x >= 0) && (x <= 320);
        let cangoy = (y >= 0) && (y <= 320);
        let mut col = 0;

        if cangox && cangoy {
            col = self.backbuf[(x + 320 * y) as usize];
        }

        col
    }

    pub fn calc_alpha(
        &mut self, 
        a: u32,
        v1: u32, v2: u32
    ) -> u32{
        let fin = ((255-a)*v1+a*v2)/255;
        fin
    }

    pub fn blit(&mut self){
        let width = self.regs[gpu_reg::W];
        let height = self.regs[gpu_reg::H];
        
        let destx = self.regs[gpu_reg::DX];
        let desty = self.regs[gpu_reg::DY];
        
        let srcx = self.regs[gpu_reg::SX];
        let srcy = self.regs[gpu_reg::SY];

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
            

            //checks if pixel is outside screen
            let cangox = (dx >= 0) && (dx <= 320);
            let cangoy = (dy >= 0) && (dy <= 320);
            
            //Only draw pixel if it's visible
            if cangox && cangoy {
                 //calculating alpha
                 
                 //Getting rgb of texture
                 let sr = (*pixel & 0x00ff0000)>>16;
                 let sg = (*pixel & 0x0000ff00)>>8;
                 let sb = *pixel & 0x000000ff;
                 let sa = *pixel>>24;
                 
                 //getting rgb from framebuffer
                 let dpix = self.get_pixel(dx,dy);
                
                 let dr = (dpix & 0x00ff0000)>>16;
                 let dg = (dpix & 0x0000ff00)>>8;
                 let db = dpix & 0x000000ff;
                 let da = dpix>>24;
                 
                 //Get the alpha
                 let fr = self.calc_alpha(sa, sr, dr);
                 let fg = self.calc_alpha(sa, sg, dg);
                 let fb = self.calc_alpha(sa, sb, db);
                 
                 //Combine into DWORD
                 let fp: u32 = (0xff << 24 | fr << 16 | fg << 8 | fb);


        
                 self.pixel(dx, dy, fp);
            }
        }

    }
    pub fn clear(&mut self){
        //println!("Not implmented");
        for pixel in self.backbuf.iter_mut() {
            *pixel = self.regs[gpu_reg::CC] as u32;
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
