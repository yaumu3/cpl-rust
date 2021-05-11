use cargo_snippet::snippet;

#[snippet]
pub fn adjacent_grids(
    i: usize,
    j: usize,
    height: usize,
    width: usize,
    directions: &[(usize, usize)],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    assert!(height < !0 && width < !0);
    directions.iter().filter_map(move |&(di, dj)| {
        let ni = i.wrapping_add(di);
        let nj = j.wrapping_add(dj);
        if ni < height && nj < width {
            Some((ni, nj))
        } else {
            None
        }
    })
}

#[test]
fn test_adjacent_grids_out_of_bounds() {
    assert_eq!(None, adjacent_grids(1, 0, 1, 1, &[(0, 1), (1, 0)]).next());
    assert_eq!(None, adjacent_grids(0, 1, 1, 1, &[(0, 1), (1, 0)]).next());
    assert_eq!(None, adjacent_grids(1, 1, 1, 1, &[(0, 1), (1, 0)]).next());
}

#[snippet(include = "adjacent_grids")]
pub fn adjacent_grids_4(
    i: usize,
    j: usize,
    height: usize,
    width: usize,
) -> impl Iterator<Item = (usize, usize)> {
    adjacent_grids(i, j, height, width, &[(0, 1), (1, 0), (0, !0), (!0, 0)])
}

#[test]
fn test_adjacent_grids_4_corners() {
    for (result, expected) in [
        (
            adjacent_grids_4(0, 0, 3, 4).collect::<Vec<(_, _)>>(),
            vec![(0, 1), (1, 0)],
        ),
        (
            adjacent_grids_4(0, 3, 3, 4).collect::<Vec<(_, _)>>(),
            vec![(1, 3), (0, 2)],
        ),
        (
            adjacent_grids_4(2, 3, 3, 4).collect::<Vec<(_, _)>>(),
            vec![(2, 2), (1, 3)],
        ),
        (
            adjacent_grids_4(2, 0, 3, 4).collect::<Vec<(_, _)>>(),
            vec![(2, 1), (1, 0)],
        ),
    ]
    .iter_mut()
    {
        result.sort_unstable();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }

    assert_eq!(None, adjacent_grids_4(3, 5, 3, 4).next());
}

#[snippet(include = "adjacent_grids")]
pub fn adjacent_grids_8(
    i: usize,
    j: usize,
    height: usize,
    width: usize,
) -> impl Iterator<Item = (usize, usize)> {
    adjacent_grids(
        i,
        j,
        height,
        width,
        &[
            (0, 1),
            (1, 1),
            (1, 0),
            (1, !0),
            (0, !0),
            (!0, !0),
            (!0, 0),
            (!0, 1),
        ],
    )
}

#[test]
fn test_adjacent_grids_8_corners() {
    for (result, expected) in [
        (
            adjacent_grids_8(0, 0, 3, 4).collect::<Vec<(_, _)>>(),
            vec![(0, 1), (1, 1), (1, 0)],
        ),
        (
            adjacent_grids_8(0, 3, 3, 4).collect::<Vec<(_, _)>>(),
            vec![(1, 3), (1, 2), (0, 2)],
        ),
        (
            adjacent_grids_8(2, 3, 3, 4).collect::<Vec<(_, _)>>(),
            vec![(2, 2), (1, 2), (1, 3)],
        ),
        (
            adjacent_grids_8(2, 0, 3, 4).collect::<Vec<(_, _)>>(),
            vec![(2, 1), (1, 0), (1, 1)],
        ),
    ]
    .iter_mut()
    {
        result.sort_unstable();
        expected.sort_unstable();
        assert_eq!(result, expected);
    }
}
