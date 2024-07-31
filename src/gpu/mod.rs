pub struct Gpu {
    pub frontbuf: Vec<u32>,
    pub backbuf: Vec<u32>,
    pub texbuf: Vec<u32>,
    pub regs: Vec<f32>
}

/*trait Gpu {
    fn new() -> Gpu;
    fn blit(&self);
    fn clear(&self);
    fn flip(&self);
}*/

impl Gpu {
    pub fn new() -> Gpu {
        Gpu{
            frontbuf: vec![0u32; 320*240],
            backbuf: vec![0u32; 320*240],
            texbuf: vec![0u32; 1024*1024],
            regs: vec![0f32; 7]

        }
    }
    pub fn blit(&self){
        println!("Not implmented");
    }
    pub fn clear(&self){
        println!("Not implmented");
    }
    pub fn flip(&self){
        println!("Not implmented");
    }
}
