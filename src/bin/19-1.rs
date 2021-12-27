use glam::DMat3;
use glam::DVec3;
use glam::IVec3;
use itertools::Itertools;
// use rustfft::FftPlanner;
use std::collections::HashSet;

const FFT_SIZE: usize = 257;

#[derive(Debug)]
struct Cluster {
    name: usize,
    beacons: Vec<DVec3>,
    b_fft: [f32; FFT_SIZE],
    xyz_fft: [[f32; FFT_SIZE]; 3],
    beacon_set: HashSet<IVec3>,
    detectors: Vec<DVec3>,
}

impl Cluster {
    fn new(name: usize) -> Cluster {
        Cluster {
            name,
            beacons: Vec::new(),
            beacon_set: HashSet::new(),
            b_fft: [0.0; FFT_SIZE],
            xyz_fft: [[0.0; FFT_SIZE]; 3],
            detectors: vec![DVec3::new(0., 0., 0.)],
        }
    }
    fn push(&mut self, v: DVec3) {
        self.beacons.push(v);
        self.beacon_set.insert(v.as_ivec3());

        let fft_num = v.dot(DVec3::new(1., 2000., 2000. * 2000.)).round() as i64;
        let fft_idx = fft_num.rem_euclid(FFT_SIZE as i64) as usize;
        self.b_fft[fft_idx] = 1.0;

        for (i, fft_proj) in DVec3::AXES.iter().enumerate() {
            let fft_num = v.dot(*fft_proj).round() as i64;
            let fft_idx = fft_num.rem_euclid(FFT_SIZE as i64) as usize;
            self.xyz_fft[i][fft_idx] = 1.0;
        }
    }

    fn overlap_brute(&self, other: &Cluster) -> (usize, DVec3) {
        let detector_range = DVec3::new(1000., 1000., 1000.);
        let mut overlap_max = 0;
        let mut overlap_trans = DVec3::ZERO;
        for b1 in &self.beacons {
            'b2: for b2 in &other.beacons {
                let db = *b1 - *b2;
                assert_eq!(db, db.round());
                let mut overlap = 0;
                for (i, b3) in other.beacons.iter().enumerate() {
                    let max_possible_overlap = overlap + other.beacons.len() - i;
                    if max_possible_overlap < 12 || max_possible_overlap <= overlap_max {
                        break;
                    }
                    let b3_trans = *b3 + db;
                    assert_eq!(b3_trans, b3_trans.round());
                    if self.beacon_set.contains(&b3_trans.as_ivec3()) {
                        overlap += 1;
                    } else {
                        for detector in &self.detectors {
                            let is_in_range =
                                (*detector - b3_trans).abs().cmple(detector_range).all();
                            if is_in_range {
                                continue 'b2;
                            }
                        }
                    }
                }
                if overlap > overlap_max {
                    overlap_max = overlap;
                    overlap_trans = db;
                }
            }
        }
        (overlap_max, overlap_trans)
    }
}

fn parse(s: &str) -> Vec<Cluster> {
    let mut cs = vec![];
    for (i, c_str) in s.split("\n\n").enumerate() {
        let mut beacon_it = c_str.lines();
        beacon_it.next();

        let mut cluster = Cluster::new(i);
        for v_str in beacon_it {
            let xyz = v_str.split(',').map(|x| x.parse().unwrap()).collect_vec();
            cluster.push(DVec3::from_slice(&xyz));
        }
        cs.push(cluster);
    }
    cs
}

fn rot90s() -> Vec<DMat3> {
    let mut rots = vec![];
    let dirs = [
        DVec3::X,
        DVec3::Y,
        DVec3::Z,
        DVec3::new(-1., 0., 0.),
        DVec3::new(0., -1., 0.),
        DVec3::new(0., 0., -1.),
    ];
    for xy in dirs.iter().cloned().permutations(2) {
        if xy[0] == -xy[1] {
            continue;
        }
        let rot = DMat3::from_cols(xy[0], xy[1], xy[0].cross(xy[1]));
        rots.push(rot);
    }
    rots
}

fn main() {
    // let input = include_str!("../../inputs/19-sample.txt");
    let input = include_str!("../../inputs/19.txt");
    let rots = rot90s();
    dbg!(rots.len());
    let mut cs = parse(input);
    // for c in &cs {
    //     dbg!(c.b_fft.iter().filter(|x| **x > 1.0).count());
    //     for i in 0..3 {
    //         dbg!(c.xyz_fft[i].iter().filter(|x| **x > 1.0).count());
    //     }
    // }
    // dbg!(&cs[0]);
    // let mut fft_planner = FftPlanner::<f32>::new();
    // let fft_fwd = fft_planner.plan_fft_forward(FFT_SIZE);
    // let fft_rev = fft_planner.plan_fft_inverse(FFT_SIZE);
    while cs.len() != 1 {
        let mut ix_pop = None;
        'search_pair: for ix_self in 0..cs.len() {
            for ix_other in ix_self + 1..cs.len() {
                let c_self = &cs[ix_self];
                let c_other = &cs[ix_other];
                for rot in &rots {
                    let mut c_flipped = Cluster::new(c_other.name);
                    for b in &c_other.beacons {
                        c_flipped.push(*rot * *b);
                    }
                    for d in &c_other.detectors {
                        c_flipped.detectors.push(*rot * *d);
                    }
                    let (over, delta) = c_self.overlap_brute(&c_flipped);
                    if over >= 12 {
                        let (over2, _delta2) = c_flipped.overlap_brute(c_self);
                        if over2 < 12 {
                            continue;
                        }
                        println!(
                            "Merging {:4} {:4} {:2} {:?} {}",
                            c_self.name, c_other.name, over, delta, rot
                        );
                        // println!("BUT               {:2} {:?}", over2, _delta2);
                        ix_pop = Some((ix_self, ix_other, c_flipped, delta));
                        break 'search_pair;
                    }
                }
            }
        }
        if let Some((ix_self, ix_other, c_flipped, delta)) = ix_pop {
            for b in &c_flipped.beacons {
                cs[ix_self].push(*b + delta);
            }
            cs[ix_self]
                .detectors
                .extend(c_flipped.detectors.iter().map(|d| *d + delta));
            cs.remove(ix_other);
        } else {
            panic!("Couldn't find overlapping cluster");
        }
    }
    let mut b_all = cs[0].beacon_set.iter().collect_vec();
    b_all.sort_by_key(|b| b[0]);
    for b in b_all {
        println!("{:?}", b);
    }
    dbg!(cs[0].beacon_set.len());
    dbg!(cs[0].beacons.len());
    let max_dist = cs[0]
        .detectors
        .iter()
        .cartesian_product(cs[0].detectors.iter())
        .map(|(d1, d2)| (*d1 - *d2).abs().dot(DVec3::ONE) as i64)
        .max()
        .unwrap();
    dbg!(max_dist);
}
