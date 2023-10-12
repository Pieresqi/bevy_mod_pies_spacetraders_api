use bevy_ecs::system::{Res, ResMut, Resource};
use bevy_time::Time;

#[derive(Debug, Default, Clone)]
/// Limiters to customize request sending
pub struct Rates {
    pub limit: RateLimit,
    pub strategy: RateStrategy,
    pub priority: RatePriority,
}

impl Rates {
    /// Compares two rates and returns the ordering between them.
    /// Low to High.
    pub fn comp_rev(&self, b: &Rates) -> core::cmp::Ordering {
        if self.priority == b.priority {
            core::cmp::Ordering::Equal
        } else if self.priority < b.priority {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Less
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
/// Sending rate limiter
pub enum RateLimit {
    /// Sends request at rate of 2/s.
    #[default]
    Normal,
    /// Sends request at rate of 10/s. Has cooldown.
    Burst,
}

impl From<RateLimit> for usize {
    fn from(rate: RateLimit) -> Self {
        match rate {
            RateLimit::Burst => 0,
            RateLimit::Normal => 1,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
/// Strategy for handling requests which wont be possible to sent due to exhaustion of rate limiter
pub enum RateStrategy {
    /// If theres leftover capacity they will be send, if not they will be discarted
    Oneshot,
    #[default]
    /// Request gets queued until it is possible to be send.
    Queued,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
/// Priority of requests
pub enum RatePriority {
    High,
    #[default]
    Normal,
    Low,
}

#[derive(Debug, Resource)]
/// stores token bucket for rate limiting
pub struct RateBucket {
    pub inner: htb::HTB<RateLimit>,
}

impl RateBucket {
    pub(crate) fn consume_token_for_each<F: FnMut() -> bool>(
        &mut self,
        label: RateLimit,
        mut f: F,
    ) {
        while self.inner.peek(label) {
            if !f() {
                break;
            }

            self.inner.take(label);
        }
    }
}

impl Default for RateBucket {
    fn default() -> Self {
        let mut bucket = Self {
            inner: htb::HTB::new(&[
                htb::BucketCfg {
                    this: RateLimit::Burst,
                    parent: None,
                    // burst cooldown, 10 messages over 10 seconds
                    rate: (10, std::time::Duration::from_secs(10)),
                    // can sent up to 10 messages per second
                    capacity: 10,
                },
                htb::BucketCfg {
                    this: RateLimit::Normal,
                    parent: Some(RateLimit::Burst),
                    // normal "cooldown", 2 messages per second
                    rate: (2, std::time::Duration::from_secs(1)),
                    // can sent up to 2 messages per second
                    capacity: 2,
                },
            ])
            .unwrap(),
        };
        // empty the buckets so we dont get ratelimited by opening/closing app repeatedly
        bucket.inner.drain();
        bucket
    }
}

/// replenishes buckets based on delta of update set
pub fn replenish_buckets(mut buckets: ResMut<RateBucket>, time: Res<Time>) {
    buckets.inner.advance(time.delta());
}
