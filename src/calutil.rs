use chrono::prelude::*;
use chrono::MAX_DATE;
use chrono::IsoWeek;
use chrono::Duration;


// from https://github.com/chronotope/chrono/pull/209/files

pub fn iter_days<T: TimeZone>(date: &Date<T>) -> DateDaysIterator<T> {
    DateDaysIterator { value: date.clone() }
}

/// Iterator over `NaiveDate` with a step size of one day.
#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct DateDaysIterator<T: TimeZone> {
    value: Date<T>,
}

impl<T: TimeZone> Iterator for DateDaysIterator<T> {
    type Item = Date<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.value.clone();
        let next = current.succ_opt();
        if let Some(cur) = next {
            self.value = cur;
            Some(self.value.clone())
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact_size = MAX_DATE.signed_duration_since(self.value.clone()).num_days();
        (exact_size as usize, Some(exact_size as usize))
    }
}

impl <T: TimeZone> ExactSizeIterator for DateDaysIterator<T> {}

// TODO: NaiveDateDaysIterator should implement FusedIterator, TrustedLen, and
// Step once they becomes stable: https://github.com/chronotope/chrono/issues/208

fn _week_pred(week: IsoWeek) -> IsoWeek {
    let monday = NaiveDate::from_isoywd(week.year(), week.week(), Weekday::Mon);
    (monday - Duration::weeks(1)).iso_week()
}

fn _week_succ(week: IsoWeek) -> IsoWeek {
    let monday = NaiveDate::from_isoywd(week.year(), week.week(), Weekday::Mon);
    (monday + Duration::weeks(1)).iso_week()
}
