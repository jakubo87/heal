use std::fs;

pub struct Score{
    totdist: f64,
    tottime: u32
}

impl Score{
    pub fn addev(&mut self, e: &Event){
    // dist
        let N = e.locX.len();
        for n in 0..N{
            self.totdist += (e.locX[n]*e.locX[n] + e.locY[n]*e.locY[n] + e.locZ[n]*e.locZ[n]).sqrt();
        }
        self.tottime = self.tottime + e.timest[N-1] - e.timest[0];
        println!("Updated totals.")
    }

    pub fn avgspd(& self) -> f64 {
        self.totdist / self.tottime as f64
    }
}

#[derive(Default)]
pub struct Event{
    //ID: ,
    locX: Vec<f64>,
    locY: Vec<f64>,
    locZ: Vec<f64>,
    timest: Vec<u32>,
    beats: Vec<u32>
}


fn readjson(e: &mut Event, path: &str){
    let cont = fs::read_to_string(path).expect("Couldn't read. Too bad!");
    println!("{cont}"); 


    e.locX.push(1.0);
    e.locX.push(1.2);
    e.locY.push(1.1);
    e.locY.push(1.2);
    e.locZ.push(1.1);
    e.locZ.push(1.2);
    e.timest.push(1);
    e.timest.push(2);
    e.beats.push(5);
}



fn main() {
    println!("Hello, world!");
    let PATH: String = String::from("./jsondata/data1.json");
    let mut e : Event = Default::default();
    readjson(&mut e,&PATH); 
    let mut s : Score = Score{totdist:0.0, tottime:0};
    s.addev(&e);
    let avg = s.avgspd();
    println!("Average Speed: {avg}");
}
