#[derive(Debug, Clone, Copy)]
struct Matrix2 {
    m: [[f64; 2]; 2],
}

impl Matrix2 {
    fn inverse(&self) -> Matrix2 {
        let det = self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0];

        let inv_det = 1.0 / det;

        Matrix2 {
            m: [
                [self.m[1][1] * inv_det, -self.m[0][1] * inv_det],
                [-self.m[1][0] * inv_det, self.m[0][0] * inv_det],
            ],
        }
    }
}

fn polar_metric(r: f64) -> Matrix2 {
    Matrix2 {
        m: [[1.0, 0.0], [0.0, r * r]],
    }
}

fn partial_metric_r(r: f64) -> Matrix2 {
    Matrix2 {
        m: [[0.0, 0.0], [0.0, 2.0 * r]],
    }
}

fn christoffel_polar(r: f64) {
    let g = polar_metric(r);
    let g_inv = g.inverse();

    let dg_dr = partial_metric_r(r);

    // Γ^r_{θθ} = -r
    let gamma_r_thetatheta = -r;

    // Γ^θ_{rθ} = 1/r
    let gamma_theta_rtheta = 1.0 / r;

    println!("Gamma^r_θθ = {}", gamma_r_thetatheta);
    println!("Gamma^θ_rθ = {}", gamma_theta_rtheta);
}

fn main() {
    let r = 2.0;
    christoffel_polar(r);
}
