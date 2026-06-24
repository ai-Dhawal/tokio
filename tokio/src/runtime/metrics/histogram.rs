mod h2_histogram;
pub use h2_histogram::{InvalidHistogramConfiguration, LogHistogram, LogHistogramBuilder};
use crate::util::metric_atomics::MetricAtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use crate::runtime::metrics::batch::duration_as_u64;
use std::cmp;
use std::ops::Range;
use std::time::Duration;
#[derive(Debug)]
pub(crate) struct Histogram {
    /// The histogram buckets
    buckets: Box<[MetricAtomicU64]>,
    /// The type of the histogram
    ///
    /// This handles `fn(bucket) -> Range` and `fn(value) -> bucket`
    histogram_type: HistogramType,
}
#[derive(Debug, Clone)]
pub(crate) struct HistogramBuilder {
    pub(crate) histogram_type: HistogramType,
    pub(crate) legacy: Option<LegacyBuilder>,
}
#[derive(Debug, Clone)]
pub(crate) struct LegacyBuilder {
    pub(crate) resolution: u64,
    pub(crate) scale: HistogramScale,
    pub(crate) num_buckets: usize,
}
impl Default for LegacyBuilder {
    fn default() -> Self {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
pub(crate) struct HistogramBatch {
    buckets: Box<[u64]>,
    configuration: HistogramType,
}
cfg_unstable! {
    #[doc = " Whether the histogram used to aggregate a metric uses a linear or"] #[doc =
    " logarithmic scale."] #[derive(Debug, Copy, Clone, Eq, PartialEq)] #[non_exhaustive]
    pub enum HistogramScale { #[doc = " Linear bucket scale"] Linear, #[doc =
    " Logarithmic bucket scale"] Log, } #[doc =
    " Configuration for the poll count histogram"] #[derive(Debug, Clone)] pub struct
    HistogramConfiguration { pub (crate) inner : HistogramType } impl
    HistogramConfiguration { #[doc = " Create a linear bucketed histogram"] #[doc = ""]
    #[doc = " # Arguments"] #[doc = ""] #[doc =
    " * `bucket_width`: The width of each bucket"] #[doc =
    " * `num_buckets`: The number of buckets"] pub fn linear(bucket_width : Duration,
    num_buckets : usize) -> Self { Self { inner : HistogramType::Linear(LinearHistogram {
    num_buckets, bucket_width : duration_as_u64(bucket_width), }), } } #[doc =
    " Creates a log-scaled bucketed histogram"] #[doc = ""] #[doc =
    " See [`LogHistogramBuilder`] for information about configuration & defaults"] pub fn
    log(configuration : impl Into < LogHistogram >) -> Self { Self { inner :
    HistogramType::H2(configuration.into()), } } }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum HistogramType {
    /// Linear histogram with fixed width buckets
    Linear(LinearHistogram),
    /// Old log histogram where each bucket doubles in size
    LogLegacy(LegacyLogHistogram),
    /// Log histogram implementation based on H2 Histograms
    H2(LogHistogram),
}
impl HistogramType {
    pub(crate) fn num_buckets(&self) -> usize {
        panic!("STUB: not implemented");
    }
    fn value_to_bucket(&self, value: u64) -> usize {
        panic!("STUB: not implemented");
    }
    fn bucket_range(&self, bucket: usize) -> Range<u64> {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct LinearHistogram {
    num_buckets: usize,
    bucket_width: u64,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct LegacyLogHistogram {
    num_buckets: usize,
    first_bucket_width: u64,
}
impl Histogram {
    pub(crate) fn num_buckets(&self) -> usize {
        panic!("STUB: not implemented");
    }
    cfg_64bit_metrics! {
        pub (crate) fn get(& self, bucket : usize) -> u64 { self.buckets[bucket]
        .load(Relaxed) }
    }
    pub(crate) fn bucket_range(&self, bucket: usize) -> Range<u64> {
        panic!("STUB: not implemented");
    }
}
impl HistogramBatch {
    pub(crate) fn from_histogram(histogram: &Histogram) -> HistogramBatch {
        panic!("STUB: not implemented");
    }
    pub(crate) fn measure(&mut self, value: u64, count: u64) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn submit(&self, histogram: &Histogram) {
        panic!("STUB: not implemented");
    }
    fn value_to_bucket(&self, value: u64) -> usize {
        panic!("STUB: not implemented");
    }
}
impl HistogramBuilder {
    pub(crate) fn new() -> HistogramBuilder {
        panic!("STUB: not implemented");
    }
    pub(crate) fn legacy_mut(&mut self, f: impl Fn(&mut LegacyBuilder)) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn build(&self) -> Histogram {
        panic!("STUB: not implemented");
    }
}
impl Default for HistogramBuilder {
    fn default() -> HistogramBuilder {
        panic!("STUB: not implemented");
    }
}
#[cfg(all(test, target_has_atomic = "64"))]
mod test {
    use super::*;
    macro_rules! assert_bucket_eq {
        ($h:expr, $bucket:expr, $val:expr) => {
            { assert_eq!($h .buckets[$bucket], $val); }
        };
    }
    fn linear(resolution: u64, num_buckets: usize) -> Histogram {
        HistogramBuilder {
            histogram_type: HistogramType::Linear(LinearHistogram {
                bucket_width: resolution,
                num_buckets,
            }),
            legacy: None,
        }
            .build()
    }
    #[test]
    fn test_legacy_builder() {
        let mut builder = HistogramBuilder::new();
        builder.legacy_mut(|b| b.num_buckets = 20);
        assert_eq!(builder.build().num_buckets(), 20);
    }
    #[test]
    fn log_scale_resolution_1() {
        let h = HistogramBuilder {
            histogram_type: HistogramType::LogLegacy(LegacyLogHistogram {
                first_bucket_width: 1,
                num_buckets: 10,
            }),
            legacy: None,
        }
            .build();
        assert_eq!(h.bucket_range(0), 0..1);
        assert_eq!(h.bucket_range(1), 1..2);
        assert_eq!(h.bucket_range(2), 2..4);
        assert_eq!(h.bucket_range(3), 4..8);
        assert_eq!(h.bucket_range(9), 256..u64::MAX);
        let mut b = HistogramBatch::from_histogram(&h);
        b.measure(0, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 0);
        b.measure(1, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 1);
        assert_bucket_eq!(b, 2, 0);
        b.measure(2, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 1);
        assert_bucket_eq!(b, 2, 1);
        b.measure(3, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 1);
        assert_bucket_eq!(b, 2, 2);
        b.measure(4, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 1);
        assert_bucket_eq!(b, 2, 2);
        assert_bucket_eq!(b, 3, 1);
        b.measure(100, 1);
        assert_bucket_eq!(b, 7, 1);
        b.measure(128, 1);
        assert_bucket_eq!(b, 8, 1);
        b.measure(4096, 1);
        assert_bucket_eq!(b, 9, 1);
        b.measure(u64::MAX, 1);
        assert_bucket_eq!(b, 9, 2);
    }
    #[test]
    fn log_scale_resolution_2() {
        let h = HistogramBuilder {
            histogram_type: HistogramType::LogLegacy(LegacyLogHistogram {
                num_buckets: 10,
                first_bucket_width: 2,
            }),
            legacy: None,
        }
            .build();
        assert_eq!(h.bucket_range(0), 0..2);
        assert_eq!(h.bucket_range(1), 2..4);
        assert_eq!(h.bucket_range(2), 4..8);
        assert_eq!(h.bucket_range(3), 8..16);
        assert_eq!(h.bucket_range(9), 512..u64::MAX);
        let mut b = HistogramBatch::from_histogram(&h);
        b.measure(0, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 0);
        b.measure(1, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 0);
        b.measure(2, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 1);
        assert_bucket_eq!(b, 2, 0);
        b.measure(3, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 0);
        b.measure(4, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 1);
        b.measure(5, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 2);
        b.measure(6, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 3);
        b.measure(7, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 4);
        b.measure(8, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 4);
        assert_bucket_eq!(b, 3, 1);
        b.measure(100, 1);
        assert_bucket_eq!(b, 6, 1);
        b.measure(128, 1);
        assert_bucket_eq!(b, 7, 1);
        b.measure(4096, 1);
        assert_bucket_eq!(b, 9, 1);
        for bucket in h.buckets.iter() {
            assert_eq!(bucket.load(Relaxed), 0);
        }
        b.submit(&h);
        for i in 0..h.buckets.len() {
            assert_eq!(h.buckets[i].load(Relaxed), b.buckets[i]);
        }
        b.submit(&h);
        for i in 0..h.buckets.len() {
            assert_eq!(h.buckets[i].load(Relaxed), b.buckets[i]);
        }
    }
    #[test]
    fn linear_scale_resolution_1() {
        let h = linear(1, 10);
        assert_eq!(h.bucket_range(0), 0..1);
        assert_eq!(h.bucket_range(1), 1..2);
        assert_eq!(h.bucket_range(2), 2..3);
        assert_eq!(h.bucket_range(3), 3..4);
        assert_eq!(h.bucket_range(9), 9..u64::MAX);
        let mut b = HistogramBatch::from_histogram(&h);
        b.measure(0, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 0);
        b.measure(1, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 1);
        assert_bucket_eq!(b, 2, 0);
        b.measure(2, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 1);
        assert_bucket_eq!(b, 2, 1);
        assert_bucket_eq!(b, 3, 0);
        b.measure(3, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 1);
        assert_bucket_eq!(b, 2, 1);
        assert_bucket_eq!(b, 3, 1);
        b.measure(5, 1);
        assert_bucket_eq!(b, 5, 1);
        b.measure(4096, 1);
        assert_bucket_eq!(b, 9, 1);
        for bucket in h.buckets.iter() {
            assert_eq!(bucket.load(Relaxed), 0);
        }
        b.submit(&h);
        for i in 0..h.buckets.len() {
            assert_eq!(h.buckets[i].load(Relaxed), b.buckets[i]);
        }
        b.submit(&h);
        for i in 0..h.buckets.len() {
            assert_eq!(h.buckets[i].load(Relaxed), b.buckets[i]);
        }
    }
    #[test]
    fn linear_scale_resolution_100() {
        let h = linear(100, 10);
        assert_eq!(h.bucket_range(0), 0..100);
        assert_eq!(h.bucket_range(1), 100..200);
        assert_eq!(h.bucket_range(2), 200..300);
        assert_eq!(h.bucket_range(3), 300..400);
        assert_eq!(h.bucket_range(9), 900..u64::MAX);
        let mut b = HistogramBatch::from_histogram(&h);
        b.measure(0, 1);
        assert_bucket_eq!(b, 0, 1);
        assert_bucket_eq!(b, 1, 0);
        b.measure(50, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 0);
        b.measure(100, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 1);
        assert_bucket_eq!(b, 2, 0);
        b.measure(101, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 0);
        b.measure(200, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 1);
        b.measure(299, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 2);
        b.measure(222, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 3);
        b.measure(300, 1);
        assert_bucket_eq!(b, 0, 2);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 3);
        assert_bucket_eq!(b, 3, 1);
        b.measure(888, 1);
        assert_bucket_eq!(b, 8, 1);
        b.measure(4096, 1);
        assert_bucket_eq!(b, 9, 1);
        for bucket in h.buckets.iter() {
            assert_eq!(bucket.load(Relaxed), 0);
        }
        b.submit(&h);
        for i in 0..h.buckets.len() {
            assert_eq!(h.buckets[i].load(Relaxed), b.buckets[i]);
        }
        b.submit(&h);
        for i in 0..h.buckets.len() {
            assert_eq!(h.buckets[i].load(Relaxed), b.buckets[i]);
        }
    }
    #[test]
    fn inc_by_more_than_one() {
        let h = linear(100, 10);
        let mut b = HistogramBatch::from_histogram(&h);
        b.measure(0, 3);
        assert_bucket_eq!(b, 0, 3);
        assert_bucket_eq!(b, 1, 0);
        b.measure(50, 5);
        assert_bucket_eq!(b, 0, 8);
        assert_bucket_eq!(b, 1, 0);
        b.measure(100, 2);
        assert_bucket_eq!(b, 0, 8);
        assert_bucket_eq!(b, 1, 2);
        assert_bucket_eq!(b, 2, 0);
        b.measure(101, 19);
        assert_bucket_eq!(b, 0, 8);
        assert_bucket_eq!(b, 1, 21);
        assert_bucket_eq!(b, 2, 0);
        for bucket in h.buckets.iter() {
            assert_eq!(bucket.load(Relaxed), 0);
        }
        b.submit(&h);
        for i in 0..h.buckets.len() {
            assert_eq!(h.buckets[i].load(Relaxed), b.buckets[i]);
        }
        b.submit(&h);
        for i in 0..h.buckets.len() {
            assert_eq!(h.buckets[i].load(Relaxed), b.buckets[i]);
        }
    }
}
