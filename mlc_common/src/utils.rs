use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use get_size::GetSize;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

impl IntRange for u64 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

impl IntRange for i32 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

impl IntRange for u32 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

impl IntRange for i16 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

impl IntRange for u16 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

impl IntRange for i8 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

impl IntRange for u8 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

impl IntRange for i128 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

impl IntRange for u128 {
    fn range(&self, min: Self, max: Self) -> Self {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

pub trait Bounds<T>: Debug {
    fn get() -> T;
}

#[derive(Debug)]
pub struct BoundedValue<T, MIN, MAX> {
    value: T,
    _min: PhantomData<MIN>,
    _max: PhantomData<MAX>,
}

impl<T: Copy, MIN, MAX> Copy for BoundedValue<T, MIN, MAX> {}

impl<T: PartialOrd + Debug, MIN: Bounds<T>, MAX: Bounds<T>> BoundedValue<T, MIN, MAX> {
    pub fn create(val: T) -> Self {
        let v = if val < MIN::get() {
            println!("Bound value: {val:?} to MIN: {:?}", MIN::get());
            MIN::get()
        } else if val > MAX::get() {
            println!("Bound value: {val:?} to MAX: {:?}", MAX::get());
            MAX::get()
        } else {
            val
        };
        Self {
            value: v,
            _min: PhantomData,
            _max: PhantomData,
        }
    }

    pub fn take(self) -> T {
        self.value
    }
}

impl<T, MIN, MAX> Deref for BoundedValue<T, MIN, MAX> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, MIN, MAX> DerefMut for BoundedValue<T, MIN, MAX> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<'de, T: PartialOrd + Debug + Deserialize<'de>, MIN: Bounds<T>, MAX: Bounds<T>> Deserialize<'de>
    for BoundedValue<T, MIN, MAX>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = T::deserialize(deserializer)?;
        Ok(Self::create(v))
    }
}

impl<'de, T: PartialOrd + Debug + Serialize, MIN: Bounds<T>, MAX: Bounds<T>> Serialize
    for BoundedValue<T, MIN, MAX>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<T: Clone, MIN, MAX> Clone for BoundedValue<T, MIN, MAX> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _min: PhantomData,
            _max: PhantomData,
        }
    }
}

impl<T: PartialEq, MIN, MAX> PartialEq for BoundedValue<T, MIN, MAX> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: PartialEq, MIN, MAX> PartialEq<T> for BoundedValue<T, MIN, MAX> {
    fn eq(&self, other: &T) -> bool {
        self.value.eq(other)
    }
}

impl<T: PartialOrd, MIN, MAX> PartialOrd<T> for BoundedValue<T, MIN, MAX> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<T: get_size::GetSize, MIN, MAX> GetSize for BoundedValue<T, MIN, MAX> {
    fn get_stack_size() -> usize {
        T::get_stack_size()
    }

    fn get_heap_size(&self) -> usize {
        self.value.get_heap_size()
    }

    fn get_size(&self) -> usize {
        self.value.get_size()
    }
}

impl<T: JsonSchema + Display, MIN: Bounds<T>, MAX: Bounds<T>> JsonSchema
    for BoundedValue<T, MIN, MAX>
{
    fn schema_name() -> String {
        format!(
            "BoundedValue<{},{},{}>",
            T::schema_name(),
            MIN::get(),
            MAX::get()
        )
    }

    fn json_schema(gen: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        T::json_schema(r#gen)
    }
}

pub mod bounds {
    use super::Bounds;

    #[derive(Debug)]
    pub struct Zero;
    #[derive(Debug)]
    pub struct One;
    #[derive(Debug)]
    pub struct NegOne;

    impl Bounds<f64> for Zero {
        fn get() -> f64 {
            0.0
        }
    }
    impl Bounds<f64> for One {
        fn get() -> f64 {
            1.0
        }
    }
    impl Bounds<f64> for NegOne {
        fn get() -> f64 {
            -1.0
        }
    }

    impl Bounds<f32> for Zero {
        fn get() -> f32 {
            0.0
        }
    }
    impl Bounds<f32> for One {
        fn get() -> f32 {
            1.0
        }
    }
    impl Bounds<f32> for NegOne {
        fn get() -> f32 {
            -1.0
        }
    }

    impl Bounds<u32> for Zero {
        fn get() -> u32 {
            0
        }
    }
    impl Bounds<u32> for One {
        fn get() -> u32 {
            1
        }
    }
}
