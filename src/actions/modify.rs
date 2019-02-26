use crate::backup::backup;
use crate::input;
use crate::utils::fileutil::write_cal;
use crate::KhResult;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ModifyArgs {
  /// Rebuild index
  #[structopt(short = "n", long = "dry-run")]
  pub dry_run: bool,
  /// index path
  #[structopt(subcommand)]
  pub modify_cmd: ModifyCommand,
}

#[derive(Debug, StructOpt)]
pub enum ModifyCommand {
  /// Show agenda view
  #[structopt(name = "remove-xlicerror", author = "")]
  RemoveXlicerror,
}

pub fn do_modify(args: &ModifyArgs) -> KhResult<()> {
  info!("do_modify");

  match &args.modify_cmd {
    ModifyCommand::RemoveXlicerror => {
      let dry_run = args.dry_run;

      let khlines = input::default_input_khlines()?;
      for khline in khlines {
        let (cal, count_removed) = khline.to_cal()?.with_remove_property("X-LIC-ERROR");
        if count_removed > 0 {
          if !dry_run {
            info!("Modifying {}", cal.get_path_as_string().unwrap());

            let backup_path = backup(&khline).unwrap();
            info!("Backup written to {}", backup_path.display());
            write_cal(&cal)?
          } else {
            info!("Would modify {}", cal.get_path_as_string().unwrap());
          };
        }
      }
    }
  }

  Ok(())
}

#[cfg(test)]
mod integration {
  use super::*;

  use crate::testutils::*;
  use assert_fs::prelude::*;
  use predicates::prelude::*;

  use crate::cli::CommandLine;
  use crate::cli::Command::Modify;
  use structopt::StructOpt;

  #[test]
  fn test_do_modify() {
    let testdir = prepare_testdir("testdir_with_xlicerror");

    let args = CommandLine::from_iter(&["khaleesi", "modify", "remove-xlicerror"]);
    if let Modify(x) = args.cmd {
      do_modify(&x).unwrap();
    }

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

    let args = CommandLine::from_iter(&["khaleesi", "modify", "--dry-run", "remove-xlicerror"]);
    if let Modify(x) = args.cmd {
      do_modify(&x).unwrap();
    }

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

//  #[test]
//  fn test_do_modify_negative() {
//
//    let args = CommandLine::from_iter(&["khaleesi", "modify", "nonsense"]);
//    if let Modify(x) = args.cmd {
//      assert!(do_modify(&x).is_ok());
//    }
//  }
}
