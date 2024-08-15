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

pub trait Bounds<T>: Debug {
    fn get() -> T;
}

pub enum ExceededBound {
    Min,
    Max,
}

pub trait OutOfBoundsHandler<T> {
    fn handle(bound: T, value: T, exceeded_bound: ExceededBound);
}

impl<T: Debug> OutOfBoundsHandler<T> for () {
    fn handle(bound: T, value: T, exceeded_bound: ExceededBound) {
        println!(
            "Bound value: {:?} to bound: {:?} ({})",
            value,
            bound,
            match exceeded_bound {
                ExceededBound::Min => "MIN",
                ExceededBound::Max => "MAX",
            }
        );
    }
}

pub struct OOBPanicer;

impl<T: Debug> OutOfBoundsHandler<T> for OOBPanicer {
    fn handle(bound: T, value: T, exceeded_bound: ExceededBound) {
        panic!(
            "Value: {:?} exceeded bound: {:?} ({})",
            value,
            bound,
            match exceeded_bound {
                ExceededBound::Min => "MIN",
                ExceededBound::Max => "MAX",
            }
        )
    }
}

pub struct OOBIgnorer;

impl<T> OutOfBoundsHandler<T> for OOBIgnorer {
    fn handle(_: T, _: T, _: ExceededBound) {}
}

#[derive(Debug)]
pub struct BoundedValue<T, MIN, MAX, H = ()> {
    value: T,
    _min: PhantomData<MIN>,
    _max: PhantomData<MAX>,
    _out_of_bounds_handler: PhantomData<H>,
}

impl<T: Copy, MIN, MAX, H: Copy> Copy for BoundedValue<T, MIN, MAX, H> {}

impl<T: PartialOrd + Debug, MIN: Bounds<T>, MAX: Bounds<T>, H: OutOfBoundsHandler<T>>
    BoundedValue<T, MIN, MAX, H>
{
    pub fn create(val: T) -> Self {
        let v = if val < MIN::get() {
            H::handle(MIN::get(), val, ExceededBound::Min);
            MIN::get()
        } else if val > MAX::get() {
            H::handle(MAX::get(), val, ExceededBound::Max);
            MAX::get()
        } else {
            val
        };
        Self {
            value: v,
            _min: PhantomData,
            _max: PhantomData,
            _out_of_bounds_handler: PhantomData,
        }
    }

    pub fn take(self) -> T {
        self.value
    }

    pub fn once(val: T) -> T {
        Self::create(val).take()
    }
}

impl<T, MIN, MAX, H> Deref for BoundedValue<T, MIN, MAX, H> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, MIN, MAX, H> DerefMut for BoundedValue<T, MIN, MAX, H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<
        'de,
        T: PartialOrd + Debug + Deserialize<'de>,
        MIN: Bounds<T>,
        MAX: Bounds<T>,
        H: OutOfBoundsHandler<T>,
    > Deserialize<'de> for BoundedValue<T, MIN, MAX, H>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = T::deserialize(deserializer)?;
        Ok(Self::create(v))
    }
}

impl<T: PartialOrd + Debug + Serialize, MIN: Bounds<T>, MAX: Bounds<T>, H> Serialize
    for BoundedValue<T, MIN, MAX, H>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<T: Clone, MIN, MAX, H: Clone> Clone for BoundedValue<T, MIN, MAX, H> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _min: PhantomData,
            _max: PhantomData,
            _out_of_bounds_handler: PhantomData,
        }
    }
}

impl<T: PartialEq, MIN, MAX, H> PartialEq for BoundedValue<T, MIN, MAX, H> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: PartialEq, MIN, MAX, H> PartialEq<T> for BoundedValue<T, MIN, MAX, H> {
    fn eq(&self, other: &T) -> bool {
        self.value.eq(other)
    }
}

impl<T: PartialOrd, MIN, MAX, H> PartialOrd<T> for BoundedValue<T, MIN, MAX, H> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<T: get_size::GetSize, MIN, MAX, H> GetSize for BoundedValue<T, MIN, MAX, H> {
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

impl<T: JsonSchema + Display, MIN: Bounds<T>, MAX: Bounds<T>, H> JsonSchema
    for BoundedValue<T, MIN, MAX, H>
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

    macro_rules! impl_bounds {
        ($bound:ty, $val:stmt, $($ts:ty),+) => {
            $(
                impl Bounds<$ts> for $bound {
                    fn get() -> $ts {
                        $val
                    }
                }
            )*
        };
    }

    impl_bounds!(Zero, 0, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
    impl_bounds!(One, 1, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
    impl_bounds!(NegOne, -1, i8, i16, i32, i64, i128);
    impl_bounds!(Zero, 0.0, f32, f64);
    impl_bounds!(One, 1.0, f32, f64);
    impl_bounds!(NegOne, -1.0, f32, f64);

    macro_rules! impl_bounds_dynamic {
        ($bound:ident, $t:ty) => {
            #[derive(Debug)]
            pub struct $bound<const BOUND: $t>;

            impl<const BOUND: $t> Bounds<$t> for $bound<BOUND> {
                fn get() -> $t {
                    BOUND
                }
            }
        };
    }

    impl_bounds_dynamic!(DynamicU8, u8);
    impl_bounds_dynamic!(DynamicU16, u16);
    impl_bounds_dynamic!(DynamicU32, u32);
    impl_bounds_dynamic!(DynamicU64, u64);
    impl_bounds_dynamic!(DynamicU128, u128);
    impl_bounds_dynamic!(DynamicI8, i8);
    impl_bounds_dynamic!(DynamicI16, i16);
    impl_bounds_dynamic!(DynamicI32, i32);
    impl_bounds_dynamic!(DynamicI64, i64);
    impl_bounds_dynamic!(DynamicI128, i128);
}
