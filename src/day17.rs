struct Target {
    x: (isize, isize),
    y: (isize, isize),
}

fn peak_height(mut x_vel: isize, mut y_vel: isize, target: &Target) -> Option<isize> {
    let mut x = 0;
    let mut y = 0;
    let mut peak = 0;
    while x <= target.x.1 && y >= target.y.1 {
        if y_vel == 0 {
            peak = y;
        }
        if x >= target.x.0 && y <= target.y.0 {
            return Some(peak);
        }

        x += x_vel;
        y += y_vel;
        x_vel = 0.max(x_vel - 1);
        y_vel -= 1;
    }
    None
}

fn q1(target: &Target) -> isize {
    // We will always overshoot if x_vel is more than this
    let mut max = 0;
    let max_y_vel = target.y.1.abs() + 1;
    for x_vel in 0..target.x.1 + 1 {
        // the only time we will not overshoot is y = abs(y_max) - 1
        // I havent fully proven to myself that it is also the best we can do in terms of peak
        // height but I suspect that is true so the solution could probably be trivial
        for y_vel in (0..max_y_vel).rev() {
            if let Some(peak) = peak_height(x_vel, y_vel, target) {
                max = max.max(peak);
                break;
            }
        }
    }
    max
}

fn q2(target: &Target) -> isize {
    // We will always overshoot if x_vel is more than this
    let mut num_solutions = 0;
    let max_y_vel = target.y.1.abs() + 1;
    for x_vel in 0..target.x.1 + 1 {
        for y_vel in ((target.y.1 - 1)..max_y_vel).rev() {
            if let Some(_peak) = peak_height(x_vel, y_vel, target) {
                num_solutions += 1;
            }
        }
    }
    num_solutions
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let target = Target {
            x: (70, 125),
            y: (-121, -159),
        };
        assert_eq!(q1(&target), 12561);
        assert_eq!(q2(&target), 3785);
    }
}
