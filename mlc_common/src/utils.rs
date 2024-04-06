pub trait FormatEffectDuration {
    fn effect_format(&self) -> String;
}

impl FormatEffectDuration for chrono::Duration {
    fn effect_format(&self) -> String {
        let minutes = self.num_minutes();
        let secs = self.num_seconds() % 60;
        let millis = self.num_milliseconds() % 1000;

        format!("{minutes:02}:{secs:02}.{millis}")
    }
}

