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

pub enum GPU_CMD {
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
    pub fn blit(&mut self){
        
    }
    pub fn clear(&mut self){
        //println!("Not implmented");
        for pixel in self.backbuf.iter_mut() {
            *pixel = self.regs[GPU_CMD::CC as usize] as u32;
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
