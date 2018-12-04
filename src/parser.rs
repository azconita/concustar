use std::fs;

pub fn get_speeds() -> Vec<f64>{
  let data = fs::read_to_string("src/conf.txt").expect("Unable to read file");
  let v: Vec<&str> = data.split('\n').collect();
  let str_speeds: Vec<&str> = v[0].split(',').collect();
  let mut speeds: Vec<f64> = Vec::new();
  for i in str_speeds{
    speeds.push(i.parse().unwrap());
  }
  return speeds;
}

pub fn get_observatories() -> Vec<Vec<f64>>{
  let data = fs::read_to_string("src/conf.txt").expect("Unable to read file");
  let v: Vec<&str> = data.split('\n').collect();
  let mut obs: Vec<Vec<f64>> = Vec::new();
  let mut i = 0;
  for line in v{
    if i==1 && line.len() != 0{
      let str_ob: Vec<&str> = line.split(',').collect();
      let mut ob: Vec<f64> = Vec::new();
      for item in str_ob{
        ob.push(item.parse().unwrap());
      }
      obs.push(ob);
    }
    i = 1;
  }
  return obs;
}

//fn main (){
//  let _v: Vec<f64> = get_speeds();
//  let _obs: Vec<Vec<f64>> = get_observatories();
//}
