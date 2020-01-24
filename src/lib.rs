use chrono::Duration;
use std::fmt;

pub trait ChronoHumanDuration {
    type Displayer: fmt::Display;
    fn format_human(&self) -> Self::Displayer;
}

impl ChronoHumanDuration for Duration {
    type Displayer = Displayer;
    fn format_human(&self) -> Self::Displayer {
        Displayer { d: *self }
    }
}

pub struct Displayer {
    d: Duration,
}

impl fmt::Display for Displayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut wrote = false;
        let d = self.d;

        let weeks = d.num_weeks();
        if weeks > 0 {
            write!(f, "{} weeks", weeks)?;
            wrote = true;
        } else {
            let days = d.num_days();
            if days > 0 {
                write!(f, "{}{} days", if wrote { ", " } else { "" }, days)?;
                wrote = true;
            } else {
                let hours = d.num_hours();
                if hours > 0 {
                    write!(f, "{}{} hours", if wrote { ", " } else { "" }, hours)?;
                    wrote = true;
                }
            }
        }

        if wrote {
            write!(f, " ago")
        } else {
            write!(f, "just now")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChronoHumanDuration;
    use chrono::Duration;

    #[test]
    fn it_works() {
        let d = Duration::weeks(2) + Duration::days(3) + Duration::hours(2) + Duration::minutes(20);
        assert_eq!(d.format_human().to_string(), "2 weeks ago");
        let d = Duration::minutes(20);
        assert_eq!(d.format_human().to_string(), "just now");
    }
}
