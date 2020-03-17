use libc::{c_char};
use std::ffi::CString;
use std::ffi::CStr;
use std::cmp::{max,min};


#[link(name = "flintfac")]
#[link(name = "flint")]
#[link(name = "mpfr")]
#[link(name = "mpir")]
extern {
    fn factorize(poly_str: *mut c_char, length: *mut i8) -> *const *const c_char;
    fn heya() -> *const c_char;
    fn mem_test1() -> *const c_char;
    fn mem_test2(places: *const usize) -> *const c_char;
    fn get_multiplicity(places: *const usize, nb: usize, ring_n: usize) -> f64;
    fn get_char(places: *const usize, nb: usize, ring_n: usize) -> *const c_char;
    fn get_gen(places: *const usize, nb: usize, ring_n: usize) -> *const c_char;
    fn get_prefactor(places: *const usize, nb: usize, ring_n: usize) -> *const c_char;
    fn get_k(places: *const usize, nb: usize, ring_n: usize) -> usize;
    fn get_quantum_k(places_a: *const usize, nb_a: usize, places_b: *const usize, nb_b: usize, ring_n: usize) -> usize; 
    fn get_pf_and_char(places: *const usize, nb: usize, ring_n: usize) -> *const *const c_char;
    fn deallocate(factors_raw: *const *const c_char);
    fn deallocate_str(string: *const c_char);
}
//int multiplicity(int* places, int nb);
pub fn poly_to_str(poly: Vec<usize>) -> String{
    let length = poly[poly.len()-1] + 1;
    let mut pol_str = String::from(length.to_string());
    pol_str.push_str(" 2 "); 

    let mut counter = 0;

    for i in 0..poly.len() {

        for j in counter..poly[i] {
            pol_str.push_str(" 0");
        }

        pol_str.push_str(" 1");

        counter = poly[i] + 1;

    }

    pol_str


}

pub fn rs_heya() -> String {

   
    let c_buf: *const c_char = unsafe {heya()};
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned(); 
    unsafe{ deallocate_str(c_buf) };

    str_buf

}

pub fn rs_mem_test2(poly: &Vec<usize>) -> String {

    let poly_ptr = poly.as_ptr();

    let c_buf: *const c_char = unsafe {mem_test2(poly_ptr)};
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned(); 
    unsafe{ deallocate_str(c_buf) };

    str_buf

}


pub fn multiplicity(poly: &Vec<usize>, ring_n: usize) -> f64{
    let len = poly.len();
    let poly_ptr = poly.as_ptr();

    let mul = unsafe { get_multiplicity(poly_ptr, len, ring_n)};

    mul

}



pub fn char_str(poly: &[usize], ring_n: usize) -> String{
    let len = poly.len();
    let poly_ptr = poly.as_ptr();

    let c_buf: *const c_char = unsafe { get_char(poly_ptr, len, ring_n)};
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned(); 
    unsafe{ deallocate_str(c_buf) };

    str_buf

}

pub fn gen_str(poly: &Vec<usize>, ring_n: usize) -> String{
    let len = poly.len();
    let poly_ptr = poly.as_ptr();

    let c_buf: *const c_char = unsafe { get_gen(poly_ptr, len, ring_n)};
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned(); 
    unsafe{ deallocate_str(c_buf) };

    str_buf

}

pub fn code_k(poly: &[usize], ring_n: usize) -> usize{
    let len = poly.len();
    let poly_ptr = poly.as_ptr();

    let k: usize = unsafe { get_k(poly_ptr, len, ring_n)};
    
    k

}

pub fn quantum_code_k(poly_a: &[usize], poly_b: &[usize], ring_n: usize) -> usize{
    let len_a = poly_a.len();
    let poly_ptr_a = poly_a.as_ptr();

    let len_b = poly_b.len();
    let poly_ptr_b = poly_b.as_ptr();

    let k: usize = unsafe { get_quantum_k(poly_ptr_a, len_a, poly_ptr_b, len_b, ring_n)};
    
    k

}

pub fn prefactor_str(poly: &Vec<usize>, ring_n: usize) -> String{
    let len = poly.len();
    let poly_ptr = poly.as_ptr();

    let c_buf: *const c_char = unsafe { get_prefactor(poly_ptr, len, ring_n) };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned(); 
    unsafe{ deallocate_str(c_buf) };

    str_buf

}

pub fn pf_and_char_str(poly: &Vec<usize>, ring_n: usize) -> (String, String){
    let len = poly.len();
    let poly_ptr = poly.as_ptr();

    let (pf_buf,g_buf): (*const c_char, *const c_char) = unsafe { 
        let tuple = get_pf_and_char(poly_ptr, len, ring_n);

        let pf_buf = *tuple;
        let g_buf = *tuple.offset(1);

        (pf_buf,g_buf)
        
        };

    let pf_str: &CStr = unsafe { CStr::from_ptr(pf_buf) };
    let pf_slice: &str = pf_str.to_str().unwrap();
    let pf_str: String = pf_slice.to_owned(); 
    unsafe{ deallocate_str(pf_buf) };

    let g_str: &CStr = unsafe { CStr::from_ptr(g_buf) };
    let g_slice: &str = g_str.to_str().unwrap();
    let g_str: String = g_slice.to_owned(); 
    unsafe{ deallocate_str(g_buf) };


    (pf_str,g_str)

}





pub fn get_factors(poly_str: String) -> Vec<String> {

    let poly = CString::new(poly_str).expect("CString::new failed").into_raw();

    let mut length = 0;

    let v = unsafe {
        let raw_factors = factorize(poly, &mut length);

        assert!(!raw_factors.is_null());
        assert!(length >= 0);

        let _ = CString::from_raw(poly);
        //println!("Length:{}",length);
        let v: Vec<String> = (0..length as isize).map(|i| CStr::from_ptr(*raw_factors.offset(i)).to_string_lossy().into_owned()).collect();
        deallocate(raw_factors);

        v
    };

    //println!("{:?}", v);
    v
}

pub fn factorize_ring(l: usize){
    // the string sent to get_factors will have length floor(log10( number )) + 1 (degree) + 1 (space) + 1 (mod) + 2 (spaces) + l+1 (exponents) + l (spaces) = floor(log10( number )) + 6 + 2*l

}