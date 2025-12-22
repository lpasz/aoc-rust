pub fn up((x, y): (usize, usize)) -> Option<(usize, usize)> {
    y.checked_sub(1).map(|new_y| (x, new_y))
}

pub fn down((x, y): (usize, usize)) -> Option<(usize, usize)> {
    Some((x, y + 1))
}

pub fn left((x, y): (usize, usize)) -> Option<(usize, usize)> {
    x.checked_sub(1).map(|new_x| (new_x, y))
}

pub fn right((x, y): (usize, usize)) -> Option<(usize, usize)> {
    Some((x + 1, y))
}

pub fn around(point: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [
        up(point),
        down(point),
        left(point),
        right(point),
        up(point).and_then(left),
        up(point).and_then(right),
        down(point).and_then(left),
        down(point).and_then(right),
    ]
    .into_iter()
    .flatten()
}
