use regex::Regex;

#[derive(Debug, Default, PartialEq)]
pub struct RoutineStep {
    pub title: String,
    pub image_url: String,
    pub duration: u16,
    pub rest: Option<u16>,
}

impl RoutineStep {
    pub fn new(title: String, image_url: String, duration: u16, rest: Option<u16>) -> Self {
        Self {
            title,
            image_url,
            duration,
            rest,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Routine {
    pub steps: Vec<RoutineStep>,
}

impl Routine {
    pub fn get(&self, idx: u16) -> &RoutineStep {
        &self.steps[idx as usize]
    }

    pub fn steps(&self) -> Vec<&RoutineStep> {
        self.steps.iter().map(|x| x).collect()
    }

    pub fn nsteps(&self) -> usize {
        self.steps.len()
    }

    pub fn has_steps(&self) -> bool {
        self.nsteps() > 0
    }
}

impl From<&str> for Routine {
    fn from(text: &str) -> Self {
        let rx = Regex::new(r"([^;]+);([^;]+);(\d+)(?:;(\d+))?").unwrap();
        let steps: Vec<RoutineStep> = text
            .lines()
            .filter_map(|line| {
                let c = rx.captures(line)?;
                let title = c.get(1).map(|x| x.as_str().to_string())?;
                let image_url = c.get(2).map(|x| x.as_str().to_string())?;
                let duration = c.get(3).map(|x| x.as_str().parse::<u16>())?.ok()?;
                let rest = match c.get(4).map(|x| x.as_str().parse::<u16>()) {
                    Some(Ok(x)) => Some(x),
                    _ => None,
                };

                Some(RoutineStep::new(title, image_url, duration, rest))
            })
            .collect();
        Self { steps }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<(&str, &str, u16, Option<u16>)> for RoutineStep {
        fn from((title, image_url, duration, rest): (&str, &str, u16, Option<u16>)) -> Self {
            Self::new(title.to_string(), image_url.to_string(), duration, rest)
        }
    }

    #[test]
    fn parse_single_step() {
        let content = "Russian Twist;\
                       https://sample/illustration.gif;\
                       60;15";
        assert_eq!(
            Routine::from(content),
            Routine {
                steps: vec![(
                    "Russian Twist",
                    "https://sample/illustration.gif",
                    60,
                    Some(15)
                )
                    .into()]
            }
        );
        let content = "Russian Twist;\
                       https://sample/illustration.gif;\
                       60";
        assert_eq!(
            Routine::from(content),
            Routine {
                steps: vec![("Russian Twist", "https://sample/illustration.gif", 60, None).into()]
            }
        );
        let content = "Russian Twist;\
                       https://sample/illustration.gif;";
        assert_eq!(Routine::from(content), Routine { steps: vec![] });
    }

    #[test]
    fn parse_multi_steps() {
        let content = "Russian Twist;\
                       https://sample/illustration.gif;\
                       60;15\n\
                       Crunch Right;\
                       https://sample/crunch-right.jpeg;\
                       60;5\n\
                       Crunch Left;\
                       https://sample/crunch-left.jpeg;\
                       30\n\
                       ";
        assert_eq!(
            Routine::from(content),
            Routine {
                steps: vec![
                    (
                        "Russian Twist",
                        "https://sample/illustration.gif",
                        60,
                        Some(15)
                    )
                        .into(),
                    (
                        "Crunch Right",
                        "https://sample/crunch-right.jpeg",
                        60,
                        Some(5)
                    )
                        .into(),
                    ("Crunch Left", "https://sample/crunch-left.jpeg", 30, None).into()
                ]
            }
        );
    }
}
