use crate::backup::backup;
use crate::input;
use crate::utils::fileutil::write_cal;
use crate::KhResult;

pub fn do_modify(args: &[&str]) -> KhResult<()> {
  info!("do_modify");

  if args.len() >= 2 && args[0] == "removeprop" && args[1] == "xlicerror" {
    let dry_run = args.len() >= 3 && args[2] == "--dry-run";

    let khlines = input::default_input_khlines()?;
    for khline in khlines {
      let cal = khline.to_cal()?.with_remove_property("X-LIC-ERROR");
      if cal.1 > 0 {
        if !dry_run {
          info!("Modifying {}", cal.0.get_path_as_string().unwrap());

          let backup_path = backup(&khline).unwrap();
          info!("Backup written to {}", backup_path.display());
          write_cal(&cal.0)?
        } else {
          info!("Would modify {}", cal.0.get_path_as_string().unwrap());
        };
      }
    }
  } else {
    error!("not supported");
  }

  Ok(())
}

#[cfg(test)]
mod integration {
  use super::*;

  use crate::testutils::*;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  #[test]
  fn test_do_modify() {
    let testdir = prepare_testdir("testdir_with_xlicerror");
    let args = ["removeprop", "xlicerror"];

    do_modify(&args).unwrap();

    let expected = indoc!(
      "
      BEGIN:VCALENDAR
      PRODID:CommuniGate Pro 6.2.5
      VERSION:2.0
      BEGIN:VEVENT
      DTSTAMP:20180813T160004Z
      UID:1c441c1b-8ca7-4898-b670-49ce30a7137b
      SEQUENCE:2
      SUMMARY:some summary
      DTSTART:20161007T073000Z
      DTEND:20161007T160000Z
      LAST-MODIFIED:20161018T095049Z
      CREATED:20161018T094913Z
      PRIORITY:5
      END:VEVENT
      END:VCALENDAR
    "
    )
    .replace("\n", "\r\n");
    let predicate = predicate::str::similar(expected);
    testdir
      .child(".khaleesi/cal/xlicerror.ics")
      .assert(predicate);
  }

  #[test]
  fn test_do_modify_dry_run() {
    let testdir = prepare_testdir("testdir_with_xlicerror");
    let args = ["removeprop", "xlicerror", "--dry-run"];

    do_modify(&args).unwrap();

    let expected = indoc!("
      BEGIN:VCALENDAR
      PRODID:CommuniGate Pro 6.2.5
      VERSION:2.0
      BEGIN:VEVENT
      DTSTAMP:20180813T160004Z
      UID:1c441c1b-8ca7-4898-b670-49ce30a7137b
      SEQUENCE:2
      SUMMARY:some summary
      DTSTART:20161007T073000Z
      DTEND:20161007T160000Z
      LAST-MODIFIED:20161018T095049Z
      CREATED:20161018T094913Z
      PRIORITY:5
      X-LIC-ERROR;X-LIC-ERRORTYPE=VALUE-PARSE-ERROR:No value for SUMMARY property. Removing entire property:
      END:VEVENT
      END:VCALENDAR
    ");
    let predicate = predicate::str::similar(expected);
    testdir
      .child(".khaleesi/cal/xlicerror.ics")
      .assert(predicate);
  }

  #[test]
  fn test_do_modify_negative() {
    let args = ["nonsense"];

    assert!(do_modify(&args).is_ok());
  }
}
