use crate::input;
use crate::KhResult;
use crate::backup::backup;
use crate::utils::fileutil::write_cal;

pub fn do_modify(args: &[&str]) -> KhResult<()> {
  info!("do_modify");

  if args[0] == "removeprop" && args[1] == "xlicerror" {
    let dry_run = args.len() >= 3 && args[2] == "--dry-run";

    let lines = input::default_input_khlines()?;
    for khline in lines {
      let cal = khline.to_cal()?.with_remove_property("X-LIC-ERROR");
      if cal.1 > 0 {
        if !dry_run {
          println!("Modifying {}", cal.0.get_path_as_string().unwrap());

          let backup_path = backup(&khline).unwrap();
          info!("Backup written to {}", backup_path.display());
          write_cal(&cal.0)?
        } else {
          println!("Would modify {}", cal.0.get_path_as_string().unwrap());
        };
      }
    };
  } else {
    error!("not supported");
  }

  Ok(())
}
