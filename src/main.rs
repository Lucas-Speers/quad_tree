#![feature(f128)]

use rand::Rng;

type Float = f64;

#[derive(Clone, Copy)]
#[derive(Debug)]
struct Point {
    pub pos: [Float; 2],
    pub velocity: [Float; 2],
}

impl Point {
    fn new(pos: [Float; 2]) -> Point {
        Point { pos, velocity: [0.0, 0.0] }
    }
}

#[derive(Debug)]
struct Rectangle {
    pub x: Float,
    pub y: Float,
    pub w: Float,
    pub h: Float,
}

impl Rectangle {
    fn new(x: Float, y: Float, w: Float, h: Float) -> Rectangle {
        Rectangle { x, y, w, h }
    }
    fn contains(&self, p: Point) -> bool {
        (self.x <= p.pos[0]) && (p.pos[0] < self.x+self.w) &&
        (self.y <= p.pos[1]) && (p.pos[1] < self.y+self.h)
    }
}

#[derive(Debug)]
enum QuadTreeType {
    Branch(Box<[QuadTree; 4]>),
    Leave(Vec<Point>),
}

#[derive(Debug)]
struct QuadTree {
    branch_type: QuadTreeType,
    bounds: Rectangle,
    capacity: usize,
}

impl QuadTree {
    fn new(capacity: usize) -> QuadTree {
        let branch_type = QuadTreeType::Leave(Vec::with_capacity(capacity));
        let bounds = Rectangle::new(-1.0, -1.0, 2.0, 2.0);
        QuadTree { branch_type, bounds, capacity }
    }
    fn add_point(&mut self, p: Point) {
        if !self.bounds.contains(p) {
            return;
        }
        match &mut self.branch_type {
            QuadTreeType::Branch(vec) => {
                for tree in vec.iter_mut() {
                    tree.add_point(p);
                }
            },
            QuadTreeType::Leave(vec) => {
                if vec.len() >= self.capacity {
                    self.split();
                    self.add_point(p);
                } else {
                    vec.push(p);
                }
            },
        }
    }
    fn split(&mut self) {
        match &mut self.branch_type {
            QuadTreeType::Branch(_) => panic!("Split a branch node (not a leave node)"),
            QuadTreeType::Leave(vec) => {
                let mut branches = [
                    QuadTree { branch_type: QuadTreeType::Leave(Vec::with_capacity(self.capacity)), bounds: Rectangle::new(self.bounds.x, self.bounds.y, self.bounds.w/2.0, self.bounds.h/2.0), capacity: self.capacity },
                    QuadTree { branch_type: QuadTreeType::Leave(Vec::with_capacity(self.capacity)), bounds: Rectangle::new(self.bounds.x+(self.bounds.w/2.0), self.bounds.y, self.bounds.w/2.0, self.bounds.h/2.0), capacity: self.capacity },
                    QuadTree { branch_type: QuadTreeType::Leave(Vec::with_capacity(self.capacity)), bounds: Rectangle::new(self.bounds.x, self.bounds.y+(self.bounds.h/2.0), self.bounds.w/2.0, self.bounds.h/2.0), capacity: self.capacity },
                    QuadTree { branch_type: QuadTreeType::Leave(Vec::with_capacity(self.capacity)), bounds: Rectangle::new(self.bounds.x+(self.bounds.w/2.0), self.bounds.y+(self.bounds.h/2.0), self.bounds.w/2.0, self.bounds.h/2.0), capacity: self.capacity },
                ];
                for branch in 0..4 {
                    for point in vec.iter() {
                        branches[branch].add_point(*point);
                    }
                }
                self.branch_type = QuadTreeType::Branch(Box::new(branches));
                
            },
        }
    }
    fn max_depth(&self) -> usize {
        match &self.branch_type {
            QuadTreeType::Branch(r#box) => {
                let mut max = 0;
                for branch in 0..4 {
                    max = max.max(r#box[branch].max_depth());
                }
                max + 1
            },
            QuadTreeType::Leave(_) => 1,
        }
    }
    // fn update_velocities(self) {
    //     self.update_velocities_with_root_ref(&self);
    // }
    // fn update_velocities_with_root_ref(self, root: &QuadTree) {
    //     match self.branch_type {
    //         QuadTreeType::Branch(r#box) => {
    //             for branch in 0..4 {
    //                 r#box[branch].update_velocities_with_root_ref(root);
    //             }
    //         },
    //         QuadTreeType::Leave(vec) => {
    //             for point in vec {
    //                 let mut sum_forces: [Float; 2] = [0.0, 0.0];

    //             }
    //         },
    //     }
    // }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut tree = QuadTree::new(50);
    for _ in 0..1_000_000 {
        tree.add_point(Point::new([rng.gen(), rng.gen()]));
    }
    println!("{}", tree.max_depth());
}
