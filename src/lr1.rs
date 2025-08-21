fn main(){
    let mut x:String = String::from("value");
    let to_add: String = String::from(" ");
    for _ in 0..10{
        x.push_str(&to_add);
    }
    x.push_str("string");
    print!("{}", x);
    let mut x:i32 = 1;
    for _ in 1..10{
        x += 1;
        print!("{} ", x)

    }
    let mut st1 :String = String::from("ass");
    while x<3{
        st1.push_str("ingass");
        x += 1;

    }
    print!("{} ", st1);

    let s1: String = String::from("string1");
    let s2: String = String::from("string2");
    let combined: String = format!("{} {}", s1, s2);
    print!("{}", combined);

{    let s1: String = String::from("string1");
    println!("cap: {}, len: {}, ptr: {:?}", s1.capacity(), s1.len(), s1.as_ptr());

    pub fn do_smth(val: &String){
    print!("{}", val);
    let s1 = String::from("str1");
    do_smth(&s1);
    let s2 = s1;
    print!("{}", s2)
}}

{let s1 = String::from("hel");
let s2 = do_smth(s1);
print!("{}", s2);

pub fn do_smth(mut val: String) -> String{
    val.push_str("lo");
    val}

    // similar

    fn main() {
        let mut s1 = String::from("hel");
        do_smth(&mut s1);
        print!("{}", s1)
    }
    
    pub fn do_smth(val:&mut  String){
        val.push_str("lo");
    }

}


use std::fmt::Debug;

pub struct Rect{
    
    height: i32,
    weidth: i32,
    }

impl Rect{
    // pub fn area(&self) -> i32{
    //     self.weidth*self.height
    // }
    pub fn peremeter(&self) -> i32{
        2*(self.weidth+self.height)
    }
}

impl Debug for Rect{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!{f, "{}", self.weidth*self.height}
    }
}
fn main() {

    let rect = Rect{
        height: 2,
        weidth: 2,
    };
    let area = rect;
    // let perem = rect.peremeter();
    println!("{:?}", area);
    // print!("{}", perem);
}

#[derive(PartialEq, Eq)]
pub enum Direction{
    North,
    South,
    East,
    West,
}

fn main() {
    let my_dir1 = Direction::North;
    let my_dir2 = Direction::South;
    let my_dir3 = Direction::East;
    let my_dir4 = Direction::West;
    move_around(my_dir1);
    move_around(my_dir2);
    move_around(my_dir3);
    move_around(my_dir4);
}

pub fn move_around(direction: Direction){
    if direction == Direction::North{
        println!("Moving in North")
    }
    else {
        println!("Not moving in north")
    }
}


#[derive(PartialEq)]
pub enum Shape{
    Rect(f64, f64),
    Square(f64),
    Circle(f64),
}

fn main() {
    let areasquare = area(Shape::Square(2.0));
    let areacirc = area(Shape::Circle(7.0));
    let arearect = area(Shape::Rect(2.0, 3.0));
    println!("{}", areasquare);
    println!("{}", areacirc);
    println!("{}", arearect);
    
    }

pub fn area(shape: Shape) -> f64 {

    match shape {
        Shape::Circle(radius) => {radius*radius*std::f64::consts::PI},
        Shape::Rect(len, wid) => {len*wid},
        Shape::Square(side) => {side*side},
    }
}

let res = fs::read_to_string("C:/Users/DELL/Documents/chan/src/eg.txt");
    match res {
        Ok(content) => {print!("{}", content)},
        Err(err) => {println!("Error: {}", err)}
    }   

fn main() {
    let res = string_match(String::from("vfalue"));
    match res {
        Some(index) => print!("at index {:#?}", index),
        None => print!("Not found")
    }
}

pub fn string_match(a: String) -> Option<i32> {

    for (index, char) in a.chars().enumerate(){
        if char == 'a'{
            return Some(index as i32);
        }
        
}
        return None;
}

fn main(){
    let vac = vec![1,2,3,4,5,6,7,8,9,10];
    let vaace = eveners(vac);
    print!("{:?}", vaace)
}

pub fn eveners(vac: Vec<i32>) -> Vec<i32>{
    let mut vectr = Vec::new();
    for i in vac{
        if i%2 == 0{
            vectr.push(i);
        }
    }
    return vectr;   
}

use std::{cmp};

fn main(){
    let vac = vec![1,2,3,4,5,6,7,8,9,10];
    let vaace = eveners(vac);
    print!("{:?}", vaace)
}

pub fn kadane(vac: Vec<i32>) ->i32{
    let mut curr = vac[0];
    let mut max_vec = vac[0];
    for i in vac.into_iter().skip(1){  
            curr = cmp::max(curr+i, i);         
            max_vec = cmp::max(curr, max_vec)   
    };
    return max_vec;
}

//other possible is
pub fn kadane(vac: Vec<i32>) ->i32{
    let mut curr = vac[0];
    let mut max_vec = vac[0];
    for &i in vac.iter().skip(1){  
            curr = cmp::max(curr+i, i);         
            max_vec = cmp::max(curr, max_vec)   
    };
    return max_vec;
}

    let  vec= vec![1,2,3,4,5,6,7,8,9,10];
    let v1_iter = vec.iter();
    let vec2: Vec<i32> = v1_iter.filter(|x| *x % 2 == 1).collect();

    for i in vec2{
        println!("{}", i)
    }
use std::collections::HashMap;
fn main() {
    let s = String::from("abcdef");
    let t = String::from("acdef");
    let ans = is_ana(s.clone(), t);
    let ansa = veccer(s);
    println!("{ans}");
    println!("{ansa:?}");
}

pub fn is_ana(s: String, t: String) -> bool {
    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();
    
    for c in s.chars(){
        *map_s.entry(c).or_insert(0) += 1;
    }
    
    for c in t.chars(){
        *map_t.entry(c).or_insert(0) += 1;
    }
    
    if map_s == map_t {
        return true;
    }
    else {
        return false;
    }
}

pub fn veccer(s: String) -> Vec<i32>{
    let mut vec = vec![0;26];
    
    for i in s.chars() {
        vec[i as usize - 'a' as usize] += 1;
    }
    
    vec
}

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, Result};
use rand::Rng;
fn main(){
    let _ = creater();
    let _ = teller();
}

fn teller() -> Result<()>{
        let file = File::open("path.txt")?;
    let mut heads = 0;
    let mut tails = 0;
    let reader = BufReader::new(file);
    for line in reader.lines(){
        match line {
           Ok(val) => match val.as_str() {
               "H" => heads += 1,
               "T" => tails += 1,
               _ => println!("found {val}"),
           },
           Err(e) => println!("{e}"),
        }
    }
    let tot = heads + tails;
    println!("heads: {heads}, tails: {tails}, tot: {tot}");
    Ok(())
}

fn creater() -> Result<()>{
 let mut file = OpenOptions::new().append(true).create(true).open("path.txt")?;
 for _ in 0..100000{
     let num = h_o_t();
     if num == 1{
        writeln!(file, "H")?;
     }
     else {
        writeln!(file, "T")?;
     }
 }
 println!("created");
 Ok(())
}

fn h_o_t() -> i32{
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..=1);
    num
}

}


