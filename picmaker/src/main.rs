use std::fs::File;
use std::io::Write;

fn main() {
    let mut data = "P3 \n500 500 \n255\n".to_string();
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for y in 0..500 {
    	for x in 0..500 {
              let pixel = String::new();
              let pixel = r.to_string() + " " + &g.to_string() + " " + &b.to_string();
              data.push_str(&pixel);
              r += 1;
              g += 1;
              b += 1;
              if r == 256 {
       	      	 r = 0;
	      }
	      if g == 256 {
	      	 g = 0;
	      }
	      if b == 256 {
	      	 b = 0;
	      }
	      data.push_str("  ");
       }
       data.push_str("\n");
    }

   let mut pic = File::create("black.ppm").expect("File could not be created");
   pic.write(data.as_bytes()).expect("Could not write header"); 
}
