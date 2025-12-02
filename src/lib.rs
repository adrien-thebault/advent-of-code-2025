pub struct Timer {
    name: &'static str,
    start: std::time::Instant,
}

impl Timer {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        use colored::Colorize;
        let duration = self.start.elapsed();
        println!(
            "{}",
            format!(
                "{} : {}.{:0>3}ms",
                self.name,
                duration.as_millis(),
                duration.subsec_millis()
            )
            .dimmed()
        );
    }
}

#[macro_export]
macro_rules! timer {
    ($name:expr) => {
        let _timer = advent_of_code_2025::Timer::new($name);
    };
}
