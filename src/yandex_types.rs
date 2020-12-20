use serde::Deserialize;
use chrono::{prelude as ch, Datelike, Timelike};
use crate::errors::{Error, Result};

#[derive(Clone, Copy, Default,Debug,Deserialize)]
pub struct YandexDateTime {
    pub year: Option<i32>,
    pub year_is_relative: bool,
    pub month: Option<i32>,
    pub month_is_relative: bool,
    pub day: Option<i32>,
    pub day_is_relative: bool,
    pub hour: Option<i32>,
    pub hour_is_relative: bool,
    pub minute: Option<i32>,
    pub minute_is_relative: bool,
}

impl YandexDateTime{
    pub fn date_time<Tz: ch::TimeZone>(&self, now: ch::DateTime<Tz>)->Result<ch::DateTime<Tz>>{
        let mut res = now;
        if let Some(year) = self.year {
            let year = if self.year_is_relative { res.year() + year } else { year };
            res = res.with_year(year).ok_or(Error::BadDateCalc("year".into()))?;
        }
        if let Some(month) = self.month {
            let month = if self.month_is_relative{ res.month() as i32 + month } else { month } as u32;
            res = res.with_month(month).ok_or(Error::BadDateCalc("month".into()))?;
        }
        if let Some(day) = self.day {
            let day = if self.day_is_relative { res.day() as i32 + day } else { day } as u32;
            res = res.with_day(day).ok_or(Error::BadDateCalc("day".into()))?;
        }
        if let Some(hour) = self.hour {
            let hour = if self.hour_is_relative{ res.hour() as i32 + hour } else { hour } as u32;
            res = res.with_hour(hour).ok_or(Error::BadDateCalc("hour".into()))?;
        }
        if let Some(minute) = self.minute {
            let minute = if self.minute_is_relative{ res.minute() as i32 + minute } else { minute } as u32;
            res = res.with_minute(minute).ok_or(Error::BadDateCalc("minute".into()))?;
        }
        return Ok(res);
    }
    pub fn has_date(&self)->bool{ self.year.is_some() || self.month.is_some() || self.day.is_some() }
    pub fn has_time(&self)->bool{ self.hour.is_some() || self.minute.is_some() }
}

#[cfg(test)]
mod tests {
    use crate::yandex_types::YandexDateTime;
    use chrono::{prelude as ch, Datelike};
    use ch::TimeZone;

    #[test]
    fn ydt_empty(){
        let ydt = YandexDateTime::default();
        assert!(!ydt.has_date());
        assert!(!ydt.has_time());
    }

    #[test]
    fn ydt_date(){
        let now = ch::Utc.ymd(2010, 3, 1).and_hms(0,0,0);

        let mut ydt = YandexDateTime::default();
        ydt.year = Some(2000);

        assert!(ydt.has_date());
        assert!(!ydt.has_time());

        let ydt_date = ydt.date_time(now).expect("date time");
        assert_eq!(ydt_date.date(), now.with_year(2000).expect("").date());

        ydt.year = Some(1);
        ydt.year_is_relative = true;
        let ydt_date = ydt.date_time(now).expect("date time");
        assert_eq!(ydt_date.date(), now.with_year(2011).expect("").date());
    }
}