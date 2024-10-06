use time::{Date, Month, Duration};

pub struct Holiday {
    name: String,
    date: Date,
}

impl Holiday {
    fn new(name: &str, date: Date) -> Self {
        Holiday {
            name: name.to_string(),
            date,
        }
    }

    pub fn get_from_year(year: i32) -> Vec<Holiday> {
        let lundi_paque = Self::get_lundi_paque(year);
        vec![
            Holiday::new("Jour de l'An", Date::from_calendar_date(year, Month::January, 1).unwrap()),
            Holiday::new("Noël", Date::from_calendar_date(year, Month::December, 25).unwrap()),
            Holiday::new("Victoire", Date::from_calendar_date(year, Month::May, 8).unwrap()),
            Holiday::new("Armistice", Date::from_calendar_date(year, Month::November, 11).unwrap()),
            Holiday::new("Fête du Travail", Date::from_calendar_date(year, Month::May, 1).unwrap()),
            Holiday::new("Fête Nationale", Date::from_calendar_date(year, Month::July, 14).unwrap()),
            Holiday::new("Assomption", Date::from_calendar_date(year, Month::August, 15).unwrap()),
            Holiday::new("Toussaint", Date::from_calendar_date(year, Month::November, 1).unwrap()),
            Holiday::new("Ascension", lundi_paque + Duration::days(39)),
            Holiday::new("Lundi de Pentecôte", lundi_paque + Duration::days(50)),
            Holiday::new("Lundi de Pâques", lundi_paque + Duration::days(1)),
        ]
    }

    fn get_lundi_paque(year: i32) -> Date {
        let a: f32 = (year % 19) as f32;
        let b: f32 = (year / 100) as f32;
        let c: f32 = (year % 100) as f32;
        let d: f32 = (19.0 * a + b - (b / 4.0).floor() - ((b - (b + 8.0) / 25.0 + 1.0) / 3.0).floor() + 15.0) % 30.0;
        let e: f32 = (32.0 + 2.0 * (b % 4.0) + 2.0 * (c / 4.0).floor() - d - (c % 4.0)) % 7.0;
        let f: f32 = d + e - 7.0 * ((a + 11.0 * d + 22.0 * e) / 451.0).floor() + 114.0;
        let month: u8 = (f / 31.0) as u8;
        let day: u8 = (f % 31.0 + 1.0) as u8;
        Date::from_calendar_date(year, Month::try_from(month).unwrap(), day).unwrap()
    }
}
impl std::fmt::Display for Holiday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:02}/{:02}/{}", self.name, self.date.day(), self.date.month() as u8, self.date.year())
    }
}

fn main() {
    let holidays = Holiday::get_from_year(2023);
    for holiday in holidays {
        println!("{}", holiday);
    }
    println!("Is holiday: {}", is_holiday(Date::from_calendar_date(2023, Month::January, 2).unwrap()));
}

fn is_holiday(date: Date) -> bool {
    let holidays = Holiday::get_from_year(date.year());
    for holiday in holidays {
        if holiday.date == date {
            return true;
        }
    }
    false
}

