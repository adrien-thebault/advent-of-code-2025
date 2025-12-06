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
                "{} : {}ms",
                self.name,
                duration.as_micros() as f64 / 1_000f64
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
