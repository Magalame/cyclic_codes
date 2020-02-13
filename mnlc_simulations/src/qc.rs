
use libc::{c_int};

#[link(name = "qc")]
#[link(name = "m")]

extern {
    fn get_cr(list_v1: *const c_int, list_v2: *const c_int, length: c_int) -> c_int;
}

pub fn cr(list_v1: &[usize], list_v2: &[usize]) -> usize {

    assert!(list_v1.len()==list_v2.len());

    let length = list_v1.len();

    let tmp_v1:Vec<i32> = list_v1.iter().map(|x| *x as i32).collect();
    let tmp_v2:Vec<i32> = list_v2.iter().map(|x| *x as i32).collect();

    let v1_ptr = (&tmp_v1[..]).as_ptr();
    let v2_ptr = (&tmp_v2[..]).as_ptr();

    // println!("sub hell");
    let crossing_number = unsafe {
        
        get_cr(v1_ptr as *const c_int, v2_ptr as *const c_int, length as c_int)

    } as usize;
    // println!("outta sub hell");

    crossing_number 
}

