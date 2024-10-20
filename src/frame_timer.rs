use std::{
    collections::VecDeque,
    fmt::Display,
    time::{Duration, Instant},
};

pub struct FrameTimer {
    buffer_size: usize,
    frames: VecDeque<Duration>,
    last_frame: Instant,
}

pub struct FrameStats {
    samples: usize,
    total_frame_time: Duration,
    min_frame_time: Duration,
    max_frame_time: Duration,
}

impl From<&Duration> for FrameStats {
    fn from(value: &Duration) -> Self {
        Self {
            samples: 1,
            total_frame_time: *value,
            min_frame_time: *value,
            max_frame_time: *value,
        }
    }
}

impl FrameTimer {
    pub fn new(buffer_size: usize) -> Self {
        let buffer_size = buffer_size.max(1);
        Self {
            buffer_size,
            frames: VecDeque::with_capacity(buffer_size),
            last_frame: Instant::now(),
        }
    }

    pub fn mark_frame(&mut self) -> FrameStats {
        let now = Instant::now();
        while self.frames.len() >= self.buffer_size {
            self.frames.pop_back();
        }
        self.frames.push_front(now - self.last_frame);
        self.last_frame = now;

        self.stats()
    }

    fn stats(&self) -> FrameStats {
        self.frames
            .iter()
            .map(|it| it.into())
            .reduce(|a: FrameStats, b: FrameStats| FrameStats {
                samples: a.samples + b.samples,
                total_frame_time: a.total_frame_time + b.total_frame_time,
                min_frame_time: a.min_frame_time.min(b.min_frame_time),
                max_frame_time: a.max_frame_time.max(b.max_frame_time),
            })
            .unwrap()
    }
}

impl Display for FrameStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Frame time: {}ms (min: {}ms, max: {}ms)",
            self.total_frame_time.as_millis() / self.samples as u128,
            self.min_frame_time.as_millis(),
            self.max_frame_time.as_millis()
        )
    }
}
