use std::collections::HashMap;
use glam::IVec2;

use crate::{
    locations::Location,
    measurements::{Relative},
};

pub fn solve(raw: HashMap<String, Location>) -> HashMap<String, Location> {
    let mut unsolved_ids: Vec<String> = raw.keys().cloned().collect();
    let mut solved = HashMap::<String, Location>::new();

    let mut progress = true;
    while progress {
        progress = false;

        for id in &unsolved_ids {
            let Some(location) = raw.get(id) else {
                panic!("Unsolved id doesn't exist")
            };

            if let Some(absolute) = location.get_absolute() {
                // Forward pass
                progress = true;
                solved.insert(id.clone(), location.clone());

                for (oid, relative) in location.get_pos() {
                    if !solved.contains_key(&oid) {
                        let Some(olocation) = raw.get(&oid) else {
                            panic!("Undefined relative")
                        };
                        match relative {
                            Relative::Vector(vector) => {
                                let mut nlocation = olocation.clone();
                                nlocation.set_absolute(absolute + vector);
                                solved.insert(oid.clone(), nlocation.clone());
                                println!("solved {oid} via forward vector");
                            }
                            Relative::Distance(_distance) => {}
                            Relative::Gradient(gradient) => {
                                let mut nlocation = olocation.clone();
                                let extrapolation = IVec2::new(
                                    (gradient.east.pow(2) - gradient.west.pow(2))
                                        / (4 * gradient.step),
                                    (gradient.north.pow(2) - gradient.south.pow(2))
                                        / (4 * gradient.step),
                                );
                                nlocation.set_absolute(absolute - extrapolation);
                                solved.insert(oid.clone(), nlocation.clone());
                                println!("solved {oid} via forward gradient");
                            }
                        }
                    }
                }
            } else {
                // Backward pass, solve undetermined points via traingulation
                let mut known_distance: Option<(String, Relative, Location)> = None;
                for (oid, relative) in location.get_pos() {
                    if let Some(olocation) = solved.get(&oid) {
                        match relative {
                            Relative::Vector(vector) => {
                                let Some(absolute) = olocation.get_absolute() else {
                                    panic!("something in solved had no abs")
                                };

                                let mut nlocation = location.clone();
                                nlocation.set_absolute(absolute - vector);
                                solved.insert(id.clone(), nlocation);
                                progress = true;
                                println!("solved {id} via backwards vector");
                            }
                            Relative::Distance(_distance) => match known_distance.clone() {
                                Some((_koid, Relative::Distance(_kdistance), _kolocation)) => {
                                    println!("solved {id} via traingulation with guessing");
                                }
                                Some((_koid, Relative::Gradient(_kgradient), _kolocation)) => {
                                    println!("solved {id} via triangulation with gradient");
                                }
                                None => {
                                    known_distance = Some((oid, relative, olocation.clone()));
                                }
                                _ => {
                                    panic!("bad known distance");
                                }
                            },
                            Relative::Gradient(gradient) => {
                                match known_distance.clone() {
                                    Some((_koid, Relative::Distance(_kdistance), _kolocation)) => {
                                        println!("solved {id} via traingulation with gradient");
                                    }
                                    Some((_koid, Relative::Gradient(kgradient), kolocation)) => {
                                        // https://stackoverflow.com/questions/3349125/circle-circle-intersection-points
                                        // p0 - first known circle center
                                        let Some(p0) = kolocation.get_absolute() else {
                                            panic!("no abs")
                                        };
                                        // p1 - second known circle center
                                        let Some(p1) = olocation.get_absolute() else {
                                            panic!("no abs")
                                        };

                                        // distance from p0 to p1
                                        let d = (p1 - p0).as_vec2().length();
                                        println!("{id}: d = {d}");

                                        // distance from p0 to p2
                                        let a = ((kgradient.center.pow(2) as f32)
                                            - (gradient.center.pow(2) as f32)
                                            + d.powi(2))
                                            / (2. * d);

                                        println!("{id}: a = {a}");
                                        // distance from p2 to p1
                                        let _b = d - a;

                                        // p2 - the point on the chord of the intersection and the
                                        // vector between p0 and p1
                                        let p2 = p0.as_vec2() + ((p1 - p0).as_vec2() * a) / d;

                                        // height from p2 to the actual intersection points
                                        let ha = ((kgradient.center as f64).powf(2.0) - (a.powi(2) as f64)).max(0.0);

                                        println!("{id}: ha = {ha}");
                                        let h = ha.sqrt();

                                        println!("{id}: h = {h}");

                                        let x3a = p2.x + ((h as f32) * ((p1.y - p0.y) as f32) / d);
                                        let z3a = p2.y - ((h as f32) * ((p1.x - p0.x) as f32) / d);

                                        let x3b = p2.x - ((h as f32) * ((p1.y - p0.y) as f32) / d);
                                        let z3b = p2.y + ((h as f32) * ((p1.x - p0.x) as f32) / d);

                                        println!("solved {id} via traingulation with gradient");
                                        println!("interesection 1 at {x3a}, {z3a}");
                                        println!("interesection 2 at {x3b}, {z3b}");

                                        let extrapolation = IVec2::new(
                                             -(gradient.east.pow(2) - gradient.west.pow(2))
                                                / (4 * gradient.step),
                                            -(gradient.north.pow(2) - gradient.south.pow(2))
                                                / (4 * gradient.step),
                                        );
                                        let estimate = p1 - extrapolation;

                                        // 2. Determine which intersection is closer to the estimate
                                        let dist_a = ((x3a - estimate.x as f32).powi(2)
                                            + (z3a - estimate.y as f32).powi(2))
                                        .sqrt();
                                        let dist_b = ((x3b - estimate.x as f32).powi(2)
                                            + (z3b - estimate.y as f32).powi(2))
                                        .sqrt();

                                        let final_pos = if dist_a < dist_b {
                                            IVec2::new(
                                                x3a.round() as i32,
                                                z3a.round() as i32,
                                            )
                                        } else {
                                            IVec2::new(
                                                x3b.round() as i32,
                                                z3b.round() as i32,
                                            )
                                        };

                                        // 3. Update the solved map
                                        let mut nlocation = location.clone();
                                        nlocation.set_absolute(final_pos);
                                        solved.insert(id.clone(), nlocation);
                                        progress = true;

                                        println!(
                                            "  Chosen intersection: {}, {}",
                                            final_pos.x, final_pos.y
                                        );
                                    }
                                    None => {
                                            if location.get_pos().iter().len() > 1 {
                                                println!("stored {oid} for {id} triagnualtion");
                                                known_distance = Some((oid, relative, olocation.clone()));
                                            } else {
                                                if let Some(olocation) = solved.get(&oid) {
                                                    let Some(absolute) = olocation.get_absolute() else {
                                                        panic!("How did we get here?");
                                                    };
                                                    match relative {
                                                        Relative::Vector(vector) => {
                                                            let mut nlocation = location.clone();
                                                            nlocation.set_absolute(absolute + vector);
                                                            solved.insert(id.clone(), nlocation.clone());
                                                            println!("solved {id} via single forward vector");
                                                        }
                                                        Relative::Distance(_distance) => {}
                                                        Relative::Gradient(gradient) => {
                                                            let mut nlocation = location.clone();
                                                            let extrapolation = IVec2::new(
                                                                (gradient.east.pow(2) - gradient.west.pow(2))
                                                                    / (4 * gradient.step),
                                                                (gradient.north.pow(2) - gradient.south.pow(2))
                                                                    / (4 * gradient.step),
                                                            );
                                                            nlocation.set_absolute(absolute + extrapolation);
                                                            solved.insert(id.clone(), nlocation.clone());
                                                            println!("solved {id} via single forward gradient");
                                                        }
                                                    }
                                                }
                                            }
                                    }
                                    _ => {
                                        panic!("bad known distance");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        unsolved_ids.retain(|id| !solved.contains_key(id));
    }

    solved
}
