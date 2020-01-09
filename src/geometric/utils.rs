use geo::{CoordinateType, LineString, Point};


pub(crate) fn to_point_list<T>(lines: &LineString<T>) -> Vec<Point<T>>
where
    T: CoordinateType,
{
    lines.clone().into_points()
}