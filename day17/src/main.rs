struct Range {
    x_start: isize,
    x_end: isize,
    y_start: isize,
    y_end: isize,
}

#[derive(Debug)]
struct Probe {
    x: isize,
    y: isize,
    x_vel: isize,
    y_vel: isize,
    peak: isize,
}

impl Probe {
    // Does the probe hit its target?
    fn fire(&mut self, target: &Range) -> bool {
        while self.x < target.x_end && self.y > target.y_start {
            self.step();
            if self.x >= target.x_start
                && self.x <= target.x_end
                && self.y >= target.y_start
                && self.y <= target.y_end
            {
                return true;
            }
        }
        false
    }

    fn step(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
        if self.peak < self.y {
            self.peak = self.y;
        }
        if self.x_vel > 0 {
            self.x_vel -= 1;
        } else if self.x_vel < 0 {
            self.x_vel += 1;
        }
        self.y_vel -= 1;
    }
}

fn find_probes_for_y(y: isize, range: &Range) -> (isize, usize) {
    let mut x_vel = 0;
    let mut probe;

    let mut peak = 0;
    let mut count = 0;

    while x_vel < range.x_end {
        x_vel += 1;
        probe = Probe {
            x: 0,
            y: 0,
            x_vel,
            y_vel: y,
            peak: 0,
        };
        if probe.fire(&range) {
            count += 1;
            if probe.peak > peak {
                peak = probe.peak;
            }
        }
    }

    (peak, count)
}

fn main() {
    let range = Range {
        x_start: 244,
        x_end: 303,
        y_start: -91,
        y_end: -54,
    };

    let mut best_peak = -1;
    let mut count = 0;

    for y_vel in -100..100 {
        let (peak, new_count) = find_probes_for_y(y_vel, &range);
        if peak > best_peak {
            best_peak = peak;
        }
        count += new_count;
    }

    println!("Part 1: {}", best_peak);
    println!("Part 2: {}", count)
}
