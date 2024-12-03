use std::iter;

use itertools::Itertools;
use levenberg_marquardt::{LeastSquaresProblem, LevenbergMarquardt};
use nalgebra::{DMatrix, DVector, Dyn, Matrix, OVector, Owned, Vector};

use util::point3::{Delta3G, Point3G};

type Delta3 = Delta3G<f64>;
type Point3 = Point3G<f64>;

fn parse_coords(s: &str) -> (f64, f64, f64) {
    s.split(',')
        .map(|p| p.trim().parse().unwrap())
        .collect_tuple()
        .unwrap()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    // let min_cross = 7f64;
    // let max_cross = 27f64;
    let input = include_str!("actual_input.txt").trim().replace('\r', "");
    let min_cross = 200000000000000f64;
    let max_cross = 400000000000000f64;

    let objects = input
        .lines()
        .map(|l| {
            let (pos_raw, vel_raw) = l.split_once(" @ ").unwrap();
            let (pos_x, pos_y, pos_z) = parse_coords(pos_raw);
            let pos = Point3::new(pos_x, pos_y, pos_z);
            let (vel_x, vel_y, vel_z) = parse_coords(vel_raw);
            let vel = Delta3::new(vel_x, vel_y, vel_z);
            (pos, vel)
        })
        .collect_vec();

    let p1 = objects
        .iter()
        .combinations(2)
        .filter(|c| intersects_2d(min_cross, max_cross, c[0], c[1]))
        .count();

    println!("Part 1: {}", p1);

    let p2 = collision_3d(objects);
    println!("Part 2: {:?}", p2);
}

fn intersects_2d(
    min_cross: f64,
    max_cross: f64,
    (a_pos, a_vel): &(Point3, Delta3),
    (b_pos, b_vel): &(Point3, Delta3),
) -> bool {
    // if (a_vel.dx, a_vel.dy) == (b_vel.dx, b_vel.dy) {
    //     return false;
    // }
    // y1 = m1x1 + b1
    // y2 = m2x2 + b2
    // m1*x + b1 = m2*x + b2
    // m1x-m2x = b2-b1
    // x = (b2-b1)/(m1-m2)

    // Pxa + Ta*Vxa = Pxb + Tb*Vxb
    // Pya + Ta*Vya = Pxb + Tb*Vyb

    // Xa = Pxa + Ta*Vxa
    // Ya = Pya + Ta*Vya
    // Ta = (Xa - Pxa) / Vxa
    // Ta = (Ya - Pya) / Vya
    // (Xa - Pxa) / Vxa = (Ya - Pya) / Vya

    // Ya = (Xa - Pxa) Vya / Vxa + Pya

    // (X - Pxa) * (Vya/Vxa) + Pya = (X - Pxb) * (Vyb/Vxb) + Pyb
    // (X-Pxa)*(Vya/Vxa)+Pya=(X-Pxb)*(Vyb/Vxb)+Pyb
    // (X-Y_0_0)*((Z_1_0)/Z_0_0)+Y_1_0=(X-Y_0_1)*((Z_1_1)/Z_0_1)+Y_1_1

    // X = (Pya Vxa Vxb - Pyb Vxa Vxb - Pxa Vya Vxb + Pxb Vxa Vyb)/(Vxa Vyb - Vxb Vya)
    // and Vxb Vya!=Vxa Vyb and Vxa Vxb!=0

    let will_fail = b_vel.dx * a_vel.dy == a_vel.dx * b_vel.dy || a_vel.dx * b_vel.dx == 0f64;
    if will_fail {
        // println!("Will fail {:?} {:?}", (a_pos, a_vel), (b_pos, b_vel));
        return false;
    }

    let x = (a_pos.y * a_vel.dx * b_vel.dx
        - b_pos.y * a_vel.dx * b_vel.dx
        - a_pos.x * a_vel.dy * b_vel.dx
        + b_pos.x * a_vel.dx * b_vel.dy)
        / (a_vel.dx * b_vel.dy - b_vel.dx * a_vel.dy);
    let y = (x - a_pos.x) * a_vel.dy / a_vel.dx + a_pos.y;
    let t_a = (x - a_pos.x) / a_vel.dx;
    let t_b = (x - b_pos.x) / b_vel.dx;

    min_cross <= x
        && x <= max_cross
        && min_cross <= y
        && y <= max_cross
        && t_a >= 0f64
        && t_b >= 0f64
}

/*

Pnew + TVne = Px + TVx
Pnew + TVnew - Px - TVx = 0


Pnew_x + T0 * Vnew_x - P0_x - T0 * V0_x = 0
Pnew_y + T0 * Vnew_y - P0_y - T0 * V0_y = 0
Pnew_z + T0 * Vnew_z - P0_z - T0 * V0_z = 0
Pnew_x + T1 * Vnew_x - P1_x - T1 * V1_x = 0
Pnew_y + T1 * Vnew_y - P1_y - T1 * V1_y = 0
Pnew_z + T1 * Vnew_z - P1_z - T1 * V1_z = 0
...
Pnew_x + TN * Vnew_x - PN_x - TN* VN_x = 0
Pnew_y + TN * Vnew_y - PN_y - TN* VN_y = 0
Pnew_z + TN * Vnew_z - PN_z - TN* VN_z = 0

3N equations

T0..N
Pxm Pym Pzm Vxm Vym Vzm

N + 6 unknowns

*/

#[derive(Debug)]
struct CollisionsEqs {
    objects: Vec<(Point3, Delta3)>,
    params: Vector<f64, Dyn, Owned<f64, Dyn>>,
    ts: OVector<f64, Dyn>,
    pos: Point3,
    vel: Delta3,
}

impl LeastSquaresProblem<f64, Dyn, Dyn> for CollisionsEqs {
    type ParameterStorage = Owned<f64, Dyn>;
    type ResidualStorage = Owned<f64, Dyn>;
    type JacobianStorage = Owned<f64, Dyn, Dyn>;

    fn set_params(&mut self, x: &Vector<f64, Dyn, Self::ParameterStorage>) {
        self.params.copy_from(x);
        self.ts.copy_from(&x.view((0, 0), (x.len() - 6, 1)));
        self.pos.x = x[x.len() - 6];
        self.pos.y = x[x.len() - 5];
        self.pos.z = x[x.len() - 4];
        self.vel.dx = x[x.len() - 3];
        self.vel.dy = x[x.len() - 2];
        self.vel.dz = x[x.len() - 1];
    }

    fn params(&self) -> Vector<f64, Dyn, Self::ParameterStorage> {
        self.params.clone()
    }

    fn residuals(&self) -> Option<Vector<f64, Dyn, Self::ResidualStorage>> {
        let residuals_iter = self
            .objects
            .iter()
            .zip(self.ts.iter())
            .flat_map(|((p, v), t)| {
                [
                    self.pos.x + t * self.vel.dx - p.x - t * v.dx,
                    self.pos.y + t * self.vel.dy - p.y - t * v.dy,
                    self.pos.z + t * self.vel.dz - p.z - t * v.dz,
                ]
                .into_iter()
            });
        Some(DVector::from_iterator(
            self.objects.len() * 3,
            residuals_iter,
        ))
    }

    fn jacobian(&self) -> Option<Matrix<f64, Dyn, Dyn, Self::JacobianStorage>> {
        /*
        d/dT0 = Vnew_x - V0_x
        d/dT1..N = 0
        d/dPnew_x = 1
        d/dPnew_y = 0
        d/dPnew_z = 0
        d/dVnew_x = T0
        d/dVnew_y = 0
        d/dVnew_z = 0

        d/dT0 = Vnew_y - V0_y
        d/dT1..N = 0
        d/dPnew_x = 0
        d/dPnew_y = 1
        d/dPnew_z = 0
        d/dVnew_x = 0
        d/dVnew_y = T0
        d/dVnew_z = 0
        */
        let ts_len = self.ts.len();
        let iter = self
            .objects
            .iter()
            .zip(self.ts.iter())
            .enumerate()
            .flat_map(|(i, ((_, v), t))| {
                let row1 = iter::repeat(0.)
                    .take(i)
                    .chain(iter::once(self.vel.dx - v.dx))
                    .chain(iter::repeat(0.).take(ts_len - i - 1))
                    .chain([1., 0., 0., *t, 0., 0.]);
                let row2 = iter::repeat(0.)
                    .take(i)
                    .chain(iter::once(self.vel.dy - v.dy))
                    .chain(iter::repeat(0.).take(ts_len - i - 1))
                    .chain([0., 1., 0., 0., *t, 0.]);
                let row3 = iter::repeat(0.)
                    .take(i)
                    .chain(iter::once(self.vel.dz - v.dz))
                    .chain(iter::repeat(0.).take(ts_len - i - 1))
                    .chain([0., 0., 1., 0., 0., *t]);
                row1.chain(row2).chain(row3)
            });

        Some(DMatrix::from_row_iterator(
            self.objects.len() * 3,
            self.objects.len() + 6,
            iter,
        ))
    }
}

fn collision_3d(objects: Vec<(Point3, Delta3)>) -> Point3 {
    let objects_len = objects.len();
    let initial_time = objects.len() as f64 / 2.;
    let bb = Point3::get_bounding_box(objects.iter().map(|(p, _)| p));
    let initial_pos = bb.start + ((bb.end - bb.start) / 2.);
    let initial_vel = objects
        .iter()
        .fold(Delta3::IDENT, |total, (_, v)| total + v)
        / objects.len() as f64;
    let params = iter::repeat(initial_time).take(objects.len()).chain([
        initial_pos.x,
        initial_pos.y,
        initial_pos.z,
        initial_vel.dx,
        initial_vel.dy,
        initial_vel.dz,
    ]);
    let mut eqs = CollisionsEqs {
        objects,
        // Going to override all of this in a sec
        params: DVector::repeat(objects_len + 6, 0.),
        ts: DVector::repeat(objects_len, 0.),
        pos: Point3::new(0., 0., 0.),
        vel: Delta3::new(0., 0., 0.),
    };
    eqs.set_params(&DVector::from_iterator(objects_len + 6, params));
    let (result, report) = LevenbergMarquardt::new().minimize(eqs);
    dbg!(&result);
    dbg!(&report);
    assert!(report.termination.was_successful());
    assert!(report.objective_function.abs() < 1e-10);
    println!("Unrounded {:?}", result.pos);
    Point3::new(
        result.pos.x.round(),
        result.pos.y.round(),
        result.pos.z.round(),
    )
}
