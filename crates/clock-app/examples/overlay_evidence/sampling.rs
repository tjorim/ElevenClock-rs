use clock_core::PhysicalRect;

/// Sample strictly inside the measured clock bounds. After a fractional DPI
/// transition UIA can assign a shared integer boundary to the adjacent button
/// even though its bounding rectangle ends there. Geometry/exclusion validation
/// still uses the full rectangle; point hits supplement that validation.
pub(crate) fn clock_hit_points(clock: PhysicalRect) -> Option<impl Iterator<Item = (i32, i32)>> {
    if !(2..=512).contains(&clock.width()) || !(2..=256).contains(&clock.height()) {
        return None;
    }
    let xs = (clock.left() + 1..clock.right())
        .step_by(4)
        .chain([clock.right() - 1]);
    let ys = (clock.top() + 1..clock.bottom())
        .step_by(4)
        .chain([clock.bottom() - 1]);
    Some(ys.flat_map(move |y| xs.clone().map(move |x| (x, y))))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restored_dpi_samples_avoid_the_shared_neighbor_boundary() {
        let clock = PhysicalRect::new(910, 720, 74, 48).unwrap();
        let points: Vec<_> = clock_hit_points(clock).unwrap().collect();
        assert_eq!(points.first(), Some(&(911, 721)));
        assert!(points.contains(&(983, 767)));
        assert!(
            points
                .iter()
                .all(|&(x, y)| x > 910 && x < 984 && y > 720 && y < 768)
        );
        for axis in [
            points
                .iter()
                .map(|p| p.0)
                .collect::<std::collections::BTreeSet<_>>(),
            points.iter().map(|p| p.1).collect(),
        ] {
            let values: Vec<_> = axis.into_iter().collect();
            assert!(values.windows(2).all(|pair| pair[1] - pair[0] <= 4));
        }
    }

    #[test]
    fn interior_sampling_handles_negative_monitors_and_rejects_unprovable_sizes() {
        let clock = PhysicalRect::new(-1920, -48, 2, 2).unwrap();
        assert!(
            clock_hit_points(clock)
                .unwrap()
                .all(|point| point == (-1919, -47))
        );
        for (width, height) in [(1, 48), (74, 1), (513, 48), (74, 257)] {
            assert!(clock_hit_points(PhysicalRect::new(0, 0, width, height).unwrap()).is_none());
        }
    }
}
