#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![feature(decl_macro)]

mod scan{
    //#[macro_export]
    pub macro splitter( $string:expr, $sep:expr) {
        {
            let mut iter:Vec<&str> = $string.split($sep).collect();
            iter
        }
    }

    pub macro sscanf ($buffer:expr,$sep:expr,[$($y:ty),+],$($x:expr),+){
        let res = splitter!($buffer,$sep);
        let mut i = 0;
        $(
            $x = res[i].parse::<$y>().unwrap_or_default();
            i+=1;
        )*             
    }
}

fn main() {
    let a :u8;   let b :i32;   let c :i16;   let d :f32;
    let buffer = "02:98;76,39.6";
    let sep = [':',';',','];
    scan::sscanf!(buffer,sep,[u8,i32,i16,f32],a,b,c,d);
    println!("{} {} {} {}",a,b,c,d);

    let a :u8;   let b :i32;   let c :i16;   let d :f32;   let e :String;
    let buffer = "02:98;abc,39.6";
    let sep = [':',';',',',' '];
    scan::sscanf!(buffer,sep,[u8,i32,String,f32],a,b,e,d);
    println!("{} {} {} {}",a,b,e,d);

    let buffer = "00:00:03,400 --> 00:00:06,177";
    let a :u8;   let b :u8;   let c :u8;   let d :i32;   let e :String;
    let f:u8; let g:u8; let h:u8; let i:i32;
    scan::sscanf!(buffer,sep,[u8,u8,u8,i32,String,u8,u8,u8,i32],a,b,c,d,e,f,g,h,i);
    println!("{:02}:{:02}:{:02},{:03} --> {:02}:{:02}:{:02},{:03}",a,b,c,d,f,g,h,i);
}