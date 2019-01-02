// from https://tools.ietf.org/html/rfc5545#section-3.6.1
pub static TEST_EVENT_MULTIDAY: &str = indoc!("
    BEGIN:VCALENDAR
    VERSION:2.0
    BEGIN:VEVENT
    UID:20070423T123432Z-541111@example.com
    DTSTAMP:20070423T123432Z
    DTSTART;VALUE=DATE:20070628
    DTEND;VALUE=DATE:20070709
    SUMMARY:Festival International de Jazz de Montreal
    LOCATION:LDB Lobby
    TRANSP:TRANSPARENT
    END:VEVENT
    END:VCALENDAR
");

// from https://tools.ietf.org/html/rfc5545#section-4
pub static TEST_EVENT_ONE_MEETING: &str = indoc!("
    BEGIN:VCALENDAR
    METHOD:xyz
    VERSION:2.0
    PRODID:-//ABC Corporation//NONSGML My Product//EN
    BEGIN:VEVENT
    DTSTAMP:19970324T120000Z
    SEQUENCE:0
    UID:uid3@example.com
    ORGANIZER:mailto:jdoe@example.com
    ATTENDEE;RSVP=TRUE:mailto:jsmith@example.com
    DTSTART:19970324T123000Z
    DTEND:19970324T210000Z
    CATEGORIES:MEETING,PROJECT
    CLASS:PUBLIC
    SUMMARY:Calendaring Interoperability Planning Meeting
    DESCRIPTION:Discuss how we can test c&s interoperability\n
    using iCalendar and other IETF standards.
    LOCATION:LDB Lobby
    ATTACH;FMTTYPE=application/postscript:ftp://example.com/pub/conf/bkgrnd.ps
    END:VEVENT
    END:VCALENDAR

");

pub static TEST_EVENT_RECUR: &str = indoc!("
    BEGIN:VCALENDAR
    VERSION:2.0
    BEGIN:VEVENT
    UID:autocryptthursday
    DTSTART;VALUE=DATE:20181011
    DURATION:P2D
    SUMMARY:Autocrypt Thursdays
    RRULE:FREQ=WEEKLY;COUNT=10
    END:VEVENT
    END:VCALENDAR

");


pub static TEST_EVENT_WITH_TIMEZONE_COMPONENT: &str = indoc!("
    BEGIN:VCALENDAR
    VERSION:2.0
    PRODID:-//PIMUTILS.ORG//NONSGML khal / icalendar //EN
    BEGIN:VTIMEZONE
    TZID:Europe/Berlin
    BEGIN:DAYLIGHT
    DTSTART;VALUE=DATE-TIME:20180325T030000
    TZNAME:CEST
    TZOFFSETFROM:+0100
    TZOFFSETTO:+0200
    END:DAYLIGHT
    BEGIN:STANDARD
    DTSTART;VALUE=DATE-TIME:20181028T020000
    TZNAME:CET
    TZOFFSETFROM:+0200
    TZOFFSETTO:+0100
    END:STANDARD
    END:VTIMEZONE
    BEGIN:VEVENT
    SUMMARY:Some Event
    DTSTART;TZID=Europe/Berlin;VALUE=DATE-TIME:20181026T133000
    DTEND;TZID=Europe/Berlin;VALUE=DATE-TIME:20181026T160000
    DTSTAMP;VALUE=DATE-TIME:20181022T145405Z
    UID:O2G1SKNFDGC1OZ1675I1A9OFQOFZXTNONYNO
    SEQUENCE:1
    LOCATION:Some Location
    END:VEVENT
    END:VCALENDAR
");

pub static TEST_MULTIPLE_EVENTS: &str = indoc!("
    BEGIN:VCALENDAR
    VERSION:2.0
    BEGIN:VEVENT
    UID:uid1
    DTSTAMP:20070423T123432Z
    DTSTART;VALUE=DATE:20070628
    SUMMARY:First Event
    END:VEVENT
    BEGIN:VEVENT
    UID:uid2
    DTSTAMP:20070423T123432Z
    DTSTART;VALUE=DATE:20070628
    SUMMARY:Second Event
    END:VEVENT
    END:VCALENDAR
");

#[cfg(test)]
use icalwrap::{IcalVCalendar,IcalVEvent};
#[cfg(test)]
use std::path::PathBuf;
#[cfg(test)]
pub fn get_test_event(str: &str, path: Option<PathBuf>) -> IcalVEvent {
  IcalVCalendar::from_str(str, path)
    .map(|cal| cal.get_principal_event())
    .unwrap()
}

