use rand::Rng;
use rust_animation::{actor::{Actor, EventHandler}, play::Play};
use stretch::geometry::Point;

use crate::geometry::{DEG_TO_RAD, get_between_point, get_complementary_cross_points, RAD_TO_DEG};

trait ClearActors {
    fn clear_sub_actors(&mut self);
}

impl ClearActors for Actor<'_> {
    fn clear_sub_actors(&mut self) {
        self.sub_actor_list.clear();
    }
}

pub struct ActorEvent {
    name: String,
}

impl ActorEvent {
    pub fn new() -> Self {
        ActorEvent {
        	name: "actor_event".to_string()
        }
    }
}

impl EventHandler for ActorEvent {
    fn key_focus_in(&mut self, actor: &mut Actor) {
        println!("key_focus_in: {} {}", self.name, actor.name);
    }

    fn key_focus_out(&mut self, actor: &mut Actor) {
        println!("key_focus_out: {} {}", self.name, actor.name);
    }

    fn key_down(&mut self, key: rust_animation::actor::Key, actor: &mut Actor) {
        println!("key_down: {}  {:?}  {}", self.name, key, actor.name);

        if key == rust_animation::actor::Key::Space {
            println!("adding random line");
            
            let mut rng = rand::thread_rng();
            let stretch_factor = 1000.0;
            let start = Point { x: rng.gen::<f64>()*stretch_factor,  y: rng.gen::<f64>()*stretch_factor };
            let end = Point { x: rng.gen::<f64>()*stretch_factor,  y: rng.gen::<f64>()*stretch_factor };

            actor.clear_sub_actors();
            
			draw_line(actor, start, end);			
        } else if key == rust_animation::actor::Key::Up {
            println!("increasing koch fractal resolution");

            let mut lines = Vec::new();
            for i in 0..actor.sub_actor_list.len() {
                let segment: &Actor<'_> = &actor.sub_actor_list[i];
                if segment.name == "line" {
                    let mid_x = segment.x;
                    let mid_y = (segment.y as f64 + segment.height as f64 / 2.0) as i32;
					
                    let start_x = mid_x as f64 - (segment.height as f64 / 2.0) * (-1.0 * segment.rotation as f64 * DEG_TO_RAD).sin();
                    let start_y = mid_y as f64 - (segment.height as f64 / 2.0) * (-1.0 * segment.rotation as f64 * DEG_TO_RAD).cos();

                    let end_x = mid_x as f64 + (segment.height as f64 / 2.0) * (-1.0 * segment.rotation as f64 * DEG_TO_RAD).sin();
                    let end_y = mid_y as f64 + (segment.height as f64 / 2.0) * (-1.0 * segment.rotation as f64 * DEG_TO_RAD).cos();

                    let start = Point { x: start_x, y: start_y };
                    let end = Point { x: end_x, y: end_y };

					let first_third = get_between_point(start, end, 1f64/3f64);
                    let second_third = get_between_point(start, end, 2f64/3f64);

                    let cross_points = get_complementary_cross_points(first_third, second_third);

                    let mid;
                    if rand::random() {
                        mid = cross_points.0;
                    } else {
                        mid = cross_points.1;
                    }
					
                    lines.push((start, first_third));
                    lines.push((first_third, mid));
                    lines.push((mid, second_third));
                    lines.push((second_third, end));
                }
            }

            actor.clear_sub_actors();

            for (start, end) in lines {
                draw_line(actor, start, end);
            }
        }
    }
}

/**
 * Given an actor and two points, draw a line between the two points
 */
pub fn draw_line(actor: &mut Actor, start: Point<f64>, end: Point<f64>) {
	let mid = Point { x: (end.x + start.x) / 2.0, y: (end.y + start.y) / 2.0 };
	let length = ((end.x - start.x).powf(2.0) + (end.y - start.y).powf(2.0)).sqrt();
	
	// set measurement markers
	let mut start_marker = Play::new_actor("start_marker".to_string(), 10, 10, None);
	start_marker.x = start.x as i32;
	start_marker.y = start.y as i32;
	start_marker.set_color(1.0, 0.0, 0.0);
	actor.add_sub_actor(start_marker);
	
	let mut end_marker = Play::new_actor("end_marker".to_string(), 10, 10, None);
	end_marker.x = end.x as i32;
	end_marker.y = end.y as i32;
	end_marker.set_color(0.0, 0.0, 1.0);
	actor.add_sub_actor(end_marker);
	
	// draw a line from start to end
	let mut line = Play::new_actor("line".to_string(), 1, length as u32, None);
	line.x = mid.x as i32;
	line.y = (mid.y - length/2.0) as i32;
	let rot = -((((end.x - start.x) / (end.y - start.y)).atan() * RAD_TO_DEG) as i32);
	line.rotation = rot;
	line.set_color(0.0, 1.0, 0.0);
	actor.add_sub_actor(line);

	actor.set_needs_layout(&mut None);
}
