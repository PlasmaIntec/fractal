use std::f64::consts::PI;
use stretch::geometry::Point;

pub const RAD_TO_DEG: f64 = 180.0 / PI;
pub const DEG_TO_RAD: f64 = PI / 180.0;

/**
 * Given start and end points, return a point along the segment which is the fraction times the length of the segment
 */
pub fn get_between_point(start: Point<f64>, end: Point<f64>, fraction: f64) -> Point<f64> {
	let segment_x = start.x + (end.x - start.x) * fraction;
	let segment_y = start.y + (end.y - start.y) * fraction;

	Point { x: segment_x, y: segment_y }
}
  
/**
 * Given two points, return the two points which form the perpendicular bisector of the first two points
 */
pub fn get_complementary_cross_points(start: Point<f64>, end: Point<f64>) -> (Point<f64>, Point<f64>) {
	let delta_x = end.x - start.x;
	let delta_y = end.y - start.y;

	let mid_x = start.x + delta_x * 0.5;
	let mid_y = start.y + delta_y * 0.5;

	let p1 = Point { x: mid_x - delta_y, y: mid_y + delta_x };
	let p2 = Point { x: mid_x + delta_y, y: mid_y - delta_x };

	(p1, p2)
}
