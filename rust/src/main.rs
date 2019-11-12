extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::env;
use std::thread;

fn find_path( _key:&str, _x:i32, _y:i32 ) -> Option<String> {
    if _x == 3 && _y == 3 {
        Some("".to_string())
    } else {
        let mut md5 = Md5::new();
        md5.input( _key.as_bytes() );
        let is_lock = md5.result_str()[0..4].to_string() ;
        let mut lock_list : Vec<i32> = Vec::new() ;
        for i in is_lock.as_str().chars() {
            if i < 'b' {
                lock_list.push(1)
            } else {
                lock_list.push(0)
            }
        }

        if _x == 0 {
            lock_list[2] = 1 ;
        } else if _x == 3 {
            lock_list[3] = 1 ;
        }
        if _y == 0 {
            lock_list[0] = 1 ;
        } else if _y == 3 {
            lock_list[1] = 1 ;
        }

        let mut _path : Vec<String> = Vec::new() ;
        let _add_val_x = vec![ 0,  0, -1,  1] ;
        let _add_val_y = vec![-1,  1,  0,  0] ;
        let _add_str = vec!["U", "D", "L", "R"] ;

        for i in 0..lock_list.len() {
            if lock_list[i] == 0 {
                match find_path( &(_key.to_owned() + _add_str[i]), _x + _add_val_x[i], _y + _add_val_y[i] ) {
                    Some(ss) => _path.push(ss),
                    _ => _path.push("NG".to_string()),
                }
            } else {
                _path.push("NG".to_string()) ;
            }
        }

        let mut min_path_num = 0 ;
        let mut min_path = "NG".to_string() ;

        for i in 0.._path.len() {
            if _path[i] != "NG" {
                if min_path == "NG" || min_path_num > _path[i].len() {
                    min_path_num = _path[i].len() ;
                    min_path = _add_str[i].to_owned() + &_path[i] ;
                }
            }
        }

        if min_path == "NG" {
            None
        } else {
            Some(min_path.to_string())
        }
   }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("input key!!") ;
    } else {
        let builder = thread::Builder::new();
        let th = builder.stack_size(10000000);

        let handle = th.spawn( move || {
            match find_path( &args[1], 0, 0 ) {
                Some(s) => println!("ans: {}", s),
                None => println!("This case is impossible."),
            }
        }).unwrap();
        let _ = handle.join();
    }
}
