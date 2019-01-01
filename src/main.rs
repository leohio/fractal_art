extern crate num;
use std::fs::File;
use num::Complex;

use std::io::prelude::*;

use std::f64;
const PI:f64 = f64::consts::PI;



//Iterate over a range of c values
fn main(){

    //Specify image width and height
    let w = 640.0;
    let h = 480.0;
    let w_int = 640;
    let h_int = 480;

    //Specify real and imaginary range of image
    let re_max = 2.0;
    let re_min = -2.0;
    let im_min = -1.5;
    let im_max = 1.5;
    
    //# Generate evenly spaced values over real and imaginary ranges
    let real_interval = (re_max - re_min) / w;
    let imag_interval = (im_min - im_max) / h;
    //Frame counter
    let frame = 0;
    
    for frame in 0..200 {
        //numpy.arange(0.0, 2 * math.pi, 0.01 * math.pi):
        let angle = PI*0.01*frame as f64;
        // Increment frame counter
    
        //Open file and write PGM header info
        let mut filename = format!("output/{fr: >03}.ppm",fr= frame+1);
        println!("{}",filename);
        let mut fout = File::create(&filename).unwrap();
        {
            let s1 = concat!("P3\n# Julia Set image\n", 640," " , 480, "\n255\n");
            fout.write(s1.as_bytes()).unwrap();
            //Generate pixel values
            for h_index in 0..h_int{

                let imv = im_max + imag_interval*h_index as f64;

                for w_index in 0..w_int{
                    let rev = re_min+real_interval*w_index as f64;

                    let mut z = Complex { re:rev,im :imv};
                    let mut c = Complex { re:-1.0 + 0.3*angle.sin(),im: 0.3*angle.cos()};
                    let mut n:f64 =255.0;
                    let mut n2:f64 = 255.0;
                    let mut n3:f64 = 255.0;
                    while z.norm() < 10.0 && n >= 5.0{
                        z = z*z + c;
                        n = n - 5.0
                    }
                    if(n<=127.5) {
                        n2 = 1.5*n as f64;
                        n3 = 2.0*n/3.0 as f64;
                    }else {
                        n2 = 0.5*n + 127.5 as f64;
                        n3 = 4.0*n/3.0 +85.0 as f64;
                    }
                    //Write pixel value to file
                    //let s2 = format!("{}{}{}{}{}",n1, " ",n2," ",n3);
                    let s2 = format!("{}{}{}{}{}",n.round(), " ",n2.round()," ",n3.round());
                    fout.write(s2.as_bytes());
                    fout.write(b"\n");
                }
            }
        } 
    }
}
