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
    TRANSP:TRANSPARENT
    END:VEVENT
    END:VCALENDAR
");