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
