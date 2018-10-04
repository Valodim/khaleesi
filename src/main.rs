use std::env;
//use std::fs::{File, read_dir};
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::prelude::*;
use libc::{c_void, c_int, time_t, c_char};
use std::ffi::CStr;
use chrono::{NaiveDate, Datelike, Duration};
use std::collections::HashMap;

extern crate libc;
extern crate chrono;

pub struct Icalcomponent {
  iterating: bool,
  pub ptr: *const c_void,
}

impl Drop for Icalcomponent {
  fn drop (&mut self) {
    unsafe {
      icalcomponent_free(self.ptr);
    }
  }
}

impl Icalcomponent {
  fn from_ptr(ptr: *const c_void) -> Icalcomponent {
    Icalcomponent { ptr: ptr, iterating: false }   
  }     
}

impl Iterator for Icalcomponent {
  type Item = Icalcomponent;

  fn next (&mut self) -> Option< Icalcomponent > {
    unsafe {
      let ptr = if !self.iterating {
        self.iterating = true;
        icalcomponent_get_first_component(self.ptr, ICAL_VEVENT_COMPONENT)
      } else {
        icalcomponent_get_next_component(self.ptr, ICAL_VEVENT_COMPONENT)
      };
      if ptr.is_null() {
        None
      } else {
        let comp = Icalcomponent::from_ptr ( ptr );        
        Some(comp)
      }
    }
  }    
}

pub fn parse_component(str: &String) -> Icalcomponent {
  unsafe {
    let parsed_event = icalparser_parse_string(str.as_ptr());
    Icalcomponent::from_ptr ( parsed_event )
  }
}

pub fn get_dtstart_unix(comp: &Icalcomponent) -> i64 {
  unsafe {
    let dtstart = icalcomponent_get_dtstart(comp.ptr);
    icaltime_as_timet(dtstart)
  }
} 

pub fn get_dtend(comp: &Icalcomponent) -> NaiveDate {
  unsafe {
    let dtend = icalcomponent_get_dtend(comp.ptr);
    NaiveDate::from_ymd(dtend.year, dtend.month as u32, dtend.day as u32)
  }
}

pub fn get_dtstart(comp: &Icalcomponent) -> NaiveDate {
  unsafe {
    let dtstart = icalcomponent_get_dtstart(comp.ptr);
    NaiveDate::from_ymd(dtstart.year, dtstart.month as u32, dtstart.day as u32)
  }
}

pub fn get_uid(comp: &Icalcomponent) -> String {
  unsafe {
    let foo = CStr::from_ptr(icalcomponent_get_uid(comp.ptr));
    foo.to_string_lossy().into_owned()
  }
}

pub fn get_buckets(comp: &mut Icalcomponent) -> Vec<String> {
  let mut buckets: Vec<String> = comp.map( |x| {
    let mut start_date = get_dtstart(&x);
    let end_date = get_dtend(&x);
    let mut buckets = Vec::new();
    while start_date.iso_week() <= end_date.iso_week() {
      let bucket = format!("{}-{:02}", start_date.iso_week().year(), start_date.iso_week().week());
      buckets.push(bucket);
      start_date = start_date.checked_add_signed(Duration::days(7)).unwrap();
      }
      buckets
      } ).flatten().collect();
  buckets.sort();
  buckets.dedup();
  buckets
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

pub type IcalcomponentKind = u32;

pub const ICAL_NO_COMPONENT : IcalcomponentKind = 0 ; pub const ICAL_ANY_COMPONENT : IcalcomponentKind = 1 ; pub const ICAL_XROOT_COMPONENT : IcalcomponentKind = 2 ; pub const ICAL_XATTACH_COMPONENT : IcalcomponentKind = 3 ; pub const ICAL_VEVENT_COMPONENT : IcalcomponentKind = 4 ; pub const ICAL_VTODO_COMPONENT : IcalcomponentKind = 5 ; pub const ICAL_VJOURNAL_COMPONENT : IcalcomponentKind = 6 ; pub const ICAL_VCALENDAR_COMPONENT : IcalcomponentKind = 7 ; pub const ICAL_VAGENDA_COMPONENT : IcalcomponentKind = 8 ; pub const ICAL_VFREEBUSY_COMPONENT : IcalcomponentKind = 9 ; pub const ICAL_VALARM_COMPONENT : IcalcomponentKind = 10 ; pub const ICAL_XAUDIOALARM_COMPONENT : IcalcomponentKind = 11 ; pub const ICAL_XDISPLAYALARM_COMPONENT : IcalcomponentKind = 12 ; pub const ICAL_XEMAILALARM_COMPONENT : IcalcomponentKind = 13 ; pub const ICAL_XPROCEDUREALARM_COMPONENT : IcalcomponentKind = 14 ; pub const ICAL_VTIMEZONE_COMPONENT : IcalcomponentKind = 15 ; pub const ICAL_XSTANDARD_COMPONENT : IcalcomponentKind = 16 ; pub const ICAL_XDAYLIGHT_COMPONENT : IcalcomponentKind = 17 ; pub const ICAL_X_COMPONENT : IcalcomponentKind = 18 ; pub const ICAL_VSCHEDULE_COMPONENT : IcalcomponentKind = 19 ; pub const ICAL_VQUERY_COMPONENT : IcalcomponentKind = 20 ; pub const ICAL_VREPLY_COMPONENT : IcalcomponentKind = 21 ; pub const ICAL_VCAR_COMPONENT : IcalcomponentKind = 22 ; pub const ICAL_VCOMMAND_COMPONENT : IcalcomponentKind = 23 ; pub const ICAL_XLICINVALID_COMPONENT : IcalcomponentKind = 24 ; pub const ICAL_XLICMIMEPART_COMPONENT : IcalcomponentKind = 25 ; pub const ICAL_VAVAILABILITY_COMPONENT : IcalcomponentKind = 26 ; pub const ICAL_XAVAILABLE_COMPONENT : IcalcomponentKind = 27 ; pub const ICAL_VPOLL_COMPONENT : IcalcomponentKind = 28 ; pub const ICAL_VVOTER_COMPONENT : IcalcomponentKind = 29 ; pub const ICAL_XVOTE_COMPONENT : IcalcomponentKind = 30 ; pub const ICAL_VPATCH_COMPONENT : IcalcomponentKind = 31 ; pub const ICAL_XPATCH_COMPONENT : IcalcomponentKind = 32 ;


#[link(name = "ical")]
extern {
  //  icalcomponent* icalparser_parse_string  (  const char *   str )  
  pub fn icalparser_parse_string(str: *const u8) -> *const c_void;

  pub fn icalcomponent_free(component: *const c_void);

  //LIBICAL_ICAL_EXPORT struct icaltimetype icalcomponent_get_dtstart(icalcomponent *comp);
  pub fn icalcomponent_get_dtstart(icalcomponent: *const c_void) -> icaltimetype;
  pub fn icalcomponent_get_dtend(icalcomponent: *const c_void) -> icaltimetype;

  //LIBICAL_ICAL_EXPORT const char *icalcomponent_get_uid(icalcomponent *comp);
  pub fn icalcomponent_get_uid(comp: *const c_void) -> *const c_char;

  //LIBICAL_ICAL_EXPORT icalcomponent *icalcomponent_get_first_component(icalcomponent *component,
  //                                                                     IcalcomponentKind kind);
  pub fn icalcomponent_get_first_component(comp: *const c_void, kind: IcalcomponentKind) -> *const c_void;

  //LIBICAL_ICAL_EXPORT icalcomponent *icalcomponent_get_next_component(icalcomponent *component,
  //                                                                    IcalcomponentKind kind);
  pub fn icalcomponent_get_next_component(comp: *const c_void, kind: IcalcomponentKind) -> *const c_void;

  //time_t icaltime_as_timet  (  const struct icaltimetype    )  
  pub fn icaltime_as_timet(foo: icaltimetype) -> time_t;
}

fn read_file_to_string(path: &PathBuf) -> Result<String, String> {
  if let Ok(mut file) = File::open(&path) {
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_ok() {
      Ok(contents)
    } else {
      //println!("something went wrong reading the file");
      Err("something went wrong reading the file".to_string())
    }
  } else {
    //println!("could not open {} for reading", path.display());
    Err(format!("could not open {} for reading", path.display()))
  }
}

fn vec_from_string(str: String) -> Vec<String> {
  let mut vec: Vec<String> = Vec::new();
  vec.push(str);
  vec
}

fn write_file(filename: &String, contents: String) -> std::io::Result<()> {
    let mut filepath: String = "Index/".to_owned();
    filepath.push_str(&filename);
    let mut file = File::create(filepath)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() {
  let args: Vec<String> = env::args().collect();

  //let filename = &args[1];
  let dir = &args[1];
  let mut buckets: HashMap<String, Vec<String>> = HashMap::new();

  if let Ok(entries) = fs::read_dir(dir) {
    for entry in entries {
      if let Ok(entry) = entry {
        // Here, `entry` is a `DirEntry`.
        if entry.path().is_file() {
          if entry.path().extension().map_or(false, |extension| extension == "ics") { 
            if let Ok(contents) = read_file_to_string(&entry.path()) {
              let mut comp = parse_component(&contents);//
              let comp_buckets = get_buckets(&mut comp);
              for bucketid in comp_buckets {
                buckets.entry(bucketid)
                  .and_modify( |items| items.push(get_uid(&comp)))
                  .or_insert( vec_from_string(get_uid(&comp)) );
              }
            }
          }
        }
      }
    } 
  }
  for (key, val) in buckets.iter() {
    write_file(key, val.join("\n"));
  }


//  //println!("Searching for {}", query);
//  println!("In file {}", filename);
//
//  let mut f = File::open(filename).expect("file not found");
//
//  let mut contents = String::new();
//  f.read_to_string(&mut contents)
//    .expect("something went wrong reading the file");
//
//  println!("With text:\n{}", contents);
//
//  let mut comp = parse_component(&contents);
//
//  let mut foo = get_buckets(&mut comp);
//  println!("{}", foo.join("\n"));
}

