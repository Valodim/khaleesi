use std::env;
use std::fs::File;
use std::io::prelude::*;
use libc::c_void;
use libc::c_int;
use libc::time_t;

extern crate libc;

pub struct Icalcomponent {
  pub ptr: *const c_void
}

impl Drop for Icalcomponent {
  fn drop (&mut self) {
    unsafe {
      icalcomponent_free(self.ptr);
    }
  }
}

pub fn parse_component(str: &String) -> Icalcomponent {
  unsafe {
    let parsed_event = icalparser_parse_string(str.as_ptr());
    Icalcomponent { ptr: parsed_event }        
  }
}

pub fn get_dtstart_unix(comp: &Icalcomponent) -> i64 {
  unsafe {
    let dtstart = icalcomponent_get_dtstart(comp.ptr);
    icaltime_as_timet(dtstart)
  }
} 

pub fn get_dtstart(comp: &Icalcomponent) -> String {
  unsafe {
    let dtstart = icalcomponent_get_dtstart(comp.ptr);
    format!("{}-{:02}-{:02}", dtstart.year, dtstart.month, dtstart.day)
  }
} 

#[repr(C)]
pub struct icaltimetype { 
  // Actual year, e.g. 2001. 
  pub year: c_int, 
      // 1 (Jan) to 12 (Dec). 
      pub month : c_int , 
      pub day : c_int , 
      pub hour : c_int , 
      pub minute : c_int , 
      pub second : c_int , 
      //< 1 -> interpret this as date. 
      pub is_date : c_int , 
      //< 1 -> time is in daylight savings time. 
      pub is_daylight : c_int , 
      //< timezone 
      pub zone : *const c_void ,
}

#[link(name = "ical")]
extern {
  //  icalcomponent* icalparser_parse_string  (  const char *   str )  
  pub fn icalparser_parse_string(str: *const u8) -> *const c_void;

  pub fn icalcomponent_free(component: *const c_void);

  //LIBICAL_ICAL_EXPORT struct icaltimetype icalcomponent_get_dtstart(icalcomponent *comp);
  pub fn icalcomponent_get_dtstart(icalcomponent: *const c_void) -> icaltimetype;

  //time_t icaltime_as_timet  (  const struct icaltimetype    )  
  pub fn icaltime_as_timet(foo: icaltimetype) -> time_t;
}

fn main() {
  let args: Vec<String> = env::args().collect();

  //let query = &args[1];
  let filename = &args[1];

  //println!("Searching for {}", query);
  println!("In file {}", filename);

  let mut f = File::open(filename).expect("file not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

  println!("With text:\n{}", contents);

  let comp = parse_component(&contents);
  let time = get_dtstart(&comp);

  println!("{}", time);
}

