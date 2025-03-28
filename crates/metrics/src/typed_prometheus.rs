use prometheus::core::{Metric, MetricVec, MetricVecBuilder};

#[macro_export]
macro_rules! metrics_group {
    ($(#[$attr:meta])* $type_vis:vis struct $type_name:ident {
        $(#[name = $name:ident] #[help = $help:expr] $(#[labels($($labels:ident: $labelty:ty),*)])? $(#[buckets($($bucket:literal),*)])? $vis:vis $field:ident: $ty:ident,)*
    }) => {
        $(#[$attr])*
        $type_vis struct $type_name {
            $($vis $field: $crate::metrics_group!(@fieldtype $field $ty $(($($labels)*))?),)*
        }
        $($crate::metrics_group!(@maketype $vis $field $ty $(($($labels: $labelty),*))? $(($($bucket)*))?);)*
        impl $type_name {
            #[allow(clippy::new_without_default)]
            pub fn new() -> Self {
                Self {
                    $($field: $crate::make_collector!($crate::metrics_group!(@fieldtype $field $ty $(($($labels)*))?), stringify!($name), $help),)*
                }
            }
        }

        impl prometheus::core::Collector for $type_name {
            fn desc(&self) -> Vec<&prometheus::core::Desc> {
                $crate::typed_prometheus::itertools::concat([ $(prometheus::core::Collector::desc(&self.$field)),* ])
            }

            fn collect(&self) -> Vec<prometheus::proto::MetricFamily> {
                $crate::typed_prometheus::itertools::concat([ $(prometheus::core::Collector::collect(&self.$field)),* ])
            }
        }
        impl prometheus::core::Collector for &$type_name {
            fn desc(&self) -> Vec<&prometheus::core::Desc> {
                (**self).desc()
            }

            fn collect(&self) -> Vec<prometheus::proto::MetricFamily> {
                (**self).collect()
            }
        }
    };
    (@fieldtype $field:ident $ty:ident ($($labels:tt)*)) => { $crate::typed_prometheus::paste! { [< $field:camel $ty >] } };
    (@fieldtype $field:ident $ty:ident) => { $ty };
    (@maketype $vis:vis $field:ident $ty:ident ($($labels:tt)*)) => {
        $crate::typed_prometheus::paste! {
            $crate::metrics_vec!($vis [< $field:camel $ty >]: $ty($($labels)*));
        }
    };
    (@maketype $vis:vis $field:ident $ty:ident ($($labels:tt)*) ($($bucket:literal)*)) => {
        $crate::typed_prometheus::paste! {
            $crate::metrics_histogram_vec!($vis [< $field:camel $ty >]: $ty($($labels)*) ($($bucket)*));
        }
    };
    (@maketype $vis:vis $field:ident $ty:ident) => {};
}
pub use metrics_group;
#[doc(hidden)]
pub use {itertools, paste::paste};

#[macro_export]
macro_rules! make_collector {
    ($ty:ty, $name:expr, $help:expr $(,)?) => {
        <$ty>::with_opts(prometheus::Opts::new($name, $help).into()).unwrap()
    };
    ($ty:ty, $name:expr, $help:expr, $labels:expr $(,)?) => {
        <$ty>::new(prometheus::Opts::new($name, $help).into(), $labels).unwrap()
    };
}
pub use make_collector;

#[macro_export]
macro_rules! metrics_histogram_vec {
    ($vis:vis $name:ident: $vecty:ident($($labels:ident: $labelty:ty),+ $(,)?) ($($bucket:literal)*)) => {
        #[derive(Clone)]
        $vis struct $name($vecty);
        impl $name {
            pub fn with_opts(opts: prometheus::Opts) -> prometheus::Result<Self> {
                let opts = prometheus::HistogramOpts::from(opts).buckets(vec![$(f64::from($bucket)),*]);
                $vecty::new(opts.into(), &[$(stringify!($labels)),+]).map(Self)
            }

            pub fn with_label_values(&self, $($labels: &$labelty),+) -> <$vecty as $crate::typed_prometheus::ExtractMetricVecT>::M {
                use $crate::typed_prometheus::AsPrometheusLabel as _;
                self.0.with_label_values(&[ $($labels.as_prometheus_str().as_ref()),+ ])
            }

            pub fn remove_label_values(&self, $($labels: &$labelty),+) -> prometheus::Result<()> {
                use $crate::typed_prometheus::AsPrometheusLabel as _;
                self.0.remove_label_values(&[ $($labels.as_prometheus_str().as_ref()),+ ])
            }
        }

        impl prometheus::core::Collector for $name {
            fn desc(&self) -> Vec<&prometheus::core::Desc> {
                prometheus::core::Collector::desc(&self.0)
            }

            fn collect(&self) -> Vec<prometheus::proto::MetricFamily> {
                prometheus::core::Collector::collect(&self.0)
            }
        }
    };
}
pub use metrics_histogram_vec;

#[macro_export]
macro_rules! metrics_vec {
    ($vis:vis $name:ident: $vecty:ident($($labels:ident: $labelty:ty),+ $(,)?)) => {
        #[derive(Clone)]
        $vis struct $name($vecty);
        impl $name {
            pub fn with_opts(opts: prometheus::Opts) -> prometheus::Result<Self> {
                $vecty::new(opts.into(), &[$(stringify!($labels)),+]).map(Self)
            }

            pub fn with_label_values(&self, $($labels: &$labelty),+) -> <$vecty as $crate::typed_prometheus::ExtractMetricVecT>::M {
                use $crate::typed_prometheus::AsPrometheusLabel as _;
                self.0.with_label_values(&[ $($labels.as_prometheus_str().as_ref()),+ ])
            }
            pub fn remove_label_values(&self, $($labels: &$labelty),+) -> prometheus::Result<()> {
                use $crate::typed_prometheus::AsPrometheusLabel as _;
                self.0.remove_label_values(&[ $($labels.as_prometheus_str().as_ref()),+ ])
            }
        }

        impl prometheus::core::Collector for $name {
            fn desc(&self) -> Vec<&prometheus::core::Desc> {
                prometheus::core::Collector::desc(&self.0)
            }

            fn collect(&self) -> Vec<prometheus::proto::MetricFamily> {
                prometheus::core::Collector::collect(&self.0)
            }
        }
    };
}
pub use metrics_vec;

pub trait AsPrometheusLabel {
    fn as_prometheus_str(&self) -> impl AsRef<str> + '_;
}
impl<T: AsRef<str> + ?Sized> AsPrometheusLabel for &T {
    fn as_prometheus_str(&self) -> impl AsRef<str> + '_ {
        self
    }
}

impl AsPrometheusLabel for bool {
    fn as_prometheus_str(&self) -> impl AsRef<str> + '_ {
        match *self {
            true => "true",
            false => "false",
        }
    }
}

macro_rules! impl_prometheusvalue_itoa {
    ($($ty:ident),*) => {
        $(impl $crate::typed_prometheus::AsPrometheusLabel for $ty {
            fn as_prometheus_str(&self) -> impl AsRef<str> + '_ {
                use std::fmt::Write;
                // max # of chars = log10 of MAX, rounded up (std ilog10 rounds down),
                // + 1 if signed (for the `-`)
                const CAP: usize = ($ty::MAX.ilog10() as usize + 1) + ($ty::MIN != 0) as usize;
                let mut buf = arrayvec::ArrayString::<CAP>::new();
                write!(buf, "{self}").unwrap();
                buf
            }
        })*
    }
}

impl_prometheusvalue_itoa!(u8, u16, u32, u64, i8, i16, i32, i64);

#[doc(hidden)]
pub trait ExtractMetricVecT {
    type M: Metric;
}

impl<T: MetricVecBuilder> ExtractMetricVecT for MetricVec<T> {
    type M = T::M;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arraystring_fmt() {
        macro_rules! tst {
            ($($ty:ident),*) => {
                $(
                    assert_eq!($ty::MIN.as_prometheus_str().as_ref(), $ty::MIN.to_string().as_str());
                    assert_eq!($ty::as_prometheus_str(&0).as_ref(), $ty::to_string(&0).as_str());
                    assert_eq!($ty::MAX.as_prometheus_str().as_ref(), $ty::MAX.to_string().as_str());
                )*
            }
        }
        tst!(u8, u16, u32, u64, i8, i16, i32, i64);
    }
}
