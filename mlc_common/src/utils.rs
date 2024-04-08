pub trait FormatEffectDuration {
    fn effect_format(&self) -> String;
}

impl FormatEffectDuration for chrono::Duration {
    fn effect_format(&self) -> String {
        let minutes = self.num_minutes();
        let secs = self.num_seconds() % 60;
        let millis = self.num_milliseconds() % 1000;

        format!("{minutes:02}:{secs:02}.{millis:03}")
    }
}


pub trait IntRange {
    fn range(&self, min: Self, max: Self) -> Self;
}

impl IntRange for i64 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}

impl IntRange for u64 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}

impl IntRange for i32 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}

impl IntRange for u32 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}

impl IntRange for i16 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}

impl IntRange for u16 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}

impl IntRange for i8 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}

impl IntRange for u8 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}

impl IntRange for i128 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}

impl IntRange for u128 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}
