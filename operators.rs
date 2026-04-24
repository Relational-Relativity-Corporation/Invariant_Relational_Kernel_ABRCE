// operators.rs
//
// Invariant Relational Operators -- ABRCE Kernel
//
// =======================================================
// STRUCTURAL PRINCIPLE
// =======================================================
//
// Information resides in relations between adjacent cells.
// The relation between cell a and cell b is their
// difference: a - b.
//
// This is the primitive.
//
// Every operator acts on differences or produces differences.
// No operator computes a statistical aggregate:
// no mean, no variance, no normalization, no sum-of-all.
//
// =======================================================
// DOMAIN
// =======================================================
//
// D := { x in R^n | n < inf, |x[i]| < inf }
//
// Topology is periodic at every dimensionality:
//   1D: ring        (neighbor: i+1 mod n)
//   2D: torus       (4-connected, Von Neumann)
//   3D: 3-torus     (6-connected)
//
// All quantifiers bounded over D.
// No claim beyond D.
//
// =======================================================
// OPERATOR COMPOSITION
// =======================================================
//
// E(x, rho_base) = C(R(B(A(x)), rho(A(x))))
//
// A -> B -> R -> C -> E
//
// A : field -> gradient field
//     Extracts directed pairwise differences.
//     Output carries one difference per neighbor per cell.
//
// B : gradient field -> gradient field
//     Accumulates each directed gradient with the
//     same-direction gradient at the next cell along
//     that direction. Extends relational reach.
//     No cross-axis coupling. That is R's function.
//
// R : gradient field x rho -> field
//     Cross-couples gradients between axes.
//     This is where circulation emerges.
//     Produces evolved cell values from circulated gradients.
//
// C : field -> field
//     Bounds output. C(x) = x / (1 + |x|).
//     Output in (-1, 1) by construction.
//
// rho is spatially varying, derived from A's gradient output.
// rho is computed outside the operator sequence as a
// derived quantity. It is not a statistical aggregate of
// the field -- it is a per-cell measure of relational
// gradient strength at that cell.
//
// =======================================================
// WHAT CHANGED FROM THE ORIGINAL KERNEL
// =======================================================
//
// Original A: A(x)[i] = x[i] - mean(x)
//   mean(x) is a global statistical aggregate.
//   It collapses all relational structure into one number.
//
// Relational A: A(x)[i] = x[i] - x[i+1]  (1D)
//   Direct pairwise difference. No aggregate.
//   The relational information IS the output.
//
// Original B (2D, 4-neighbor): B(x)[i,j] = x + N + S + E + W
//   Sums five values into one. Destroys gradients.
//
// Relational B: Each gradient accumulates along its own axis.
//   South gradient at (i,j) couples with south gradient
//   at (i+1,j). No cross-axis coupling. Gradients preserved
//   along non-coupled axes for R to use.
//
// =======================================================

#![allow(dead_code)]


// =======================================================
// 1D -- Periodic Ring
// =======================================================

pub mod one_d {

    /// A -- Relational Gradient Extraction
    ///
    /// A(x)[i] = x[i] - x[(i+1) mod n]
    ///
    /// Directed difference to forward neighbor.
    /// Zero-sum on a ring by telescoping.
    /// If field is uniform, A(x) = 0 everywhere.
    pub fn operator_a(field: &[f64]) -> Vec<f64> {
        let n = field.len();
        (0..n)
            .map(|i| field[i] - field[(i + 1) % n])
            .collect()
    }

    /// B -- Local Relational Accumulation
    ///
    /// B(x)[i] = x[i] + x[(i+1) mod n]
    ///
    /// Pairwise coupling. Extends relational reach by one cell.
    /// When applied to A's output, the gradient at i
    /// accumulates with the gradient at i+1.
    pub fn operator_b(field: &[f64]) -> Vec<f64> {
        let n = field.len();
        (0..n)
            .map(|i| field[i] + field[(i + 1) % n])
            .collect()
    }

    /// R -- Antisymmetric Circulation
    ///
    /// R(x)[i] = x[i] + rho * (x[(i+1) mod n] - x[(i-1) mod n])
    ///
    /// Forward-backward difference. Antisymmetric.
    /// Maintains nonzero differences under iteration.
    /// rho in (0.0, 0.5].
    pub fn operator_r(field: &[f64], rho: f64) -> Vec<f64> {
        let n = field.len();
        (0..n)
            .map(|i| {
                let fwd = field[(i + 1) % n];
                let bwd = field[(i + n - 1) % n];
                field[i] + rho * (fwd - bwd)
            })
            .collect()
    }

    /// C -- Bounded Coherence
    ///
    /// C(x)[i] = x[i] / (1 + |x[i]|)
    ///
    /// Output in (-1, 1). Odd function. No clamping.
    pub fn operator_c(field: &[f64]) -> Vec<f64> {
        field.iter()
            .map(|&x| x / (1.0 + x.abs()))
            .collect()
    }

    /// Compute rho from gradient field.
    ///
    /// rho[i] = rho_base * |a[i]| / (1 + |a[i]|)
    ///
    /// Per-cell. Not an aggregate. Each cell's rho reflects
    /// the strength of its own relational gradient.
    pub fn compute_rho(a: &[f64], rho_base: f64) -> Vec<f64> {
        a.iter()
            .map(|&v| {
                let mag = v.abs();
                rho_base * mag / (1.0 + mag)
            })
            .collect()
    }

    /// R with spatially varying rho.
    pub fn operator_r_varying(field: &[f64], rho: &[f64]) -> Vec<f64> {
        let n = field.len();
        (0..n)
            .map(|i| {
                let fwd = field[(i + 1) % n];
                let bwd = field[(i + n - 1) % n];
                field[i] + rho[i] * (fwd - bwd)
            })
            .collect()
    }

    /// E -- Composite Evolution
    ///
    /// E(x, rho_base) = C(R(B(A(x)), rho(A(x))))
    pub fn operator_e(field: &[f64], rho_base: f64) -> Vec<f64> {
        let a = operator_a(field);
        let rho = compute_rho(&a, rho_base);
        let b = operator_b(&a);
        let r = operator_r_varying(&b, &rho);
        operator_c(&r)
    }
}


// =======================================================
// 2D -- Periodic Torus
// =======================================================

pub mod two_d {

    // --- Grid access ---

    #[inline]
    fn wrap(i: isize, n: usize) -> usize {
        ((i % n as isize) + n as isize) as usize % n
    }

    #[inline]
    fn at(field: &[f64], rows: usize, cols: usize,
          i: isize, j: isize) -> f64 {
        field[wrap(i, rows) * cols + wrap(j, cols)]
    }

    #[inline]
    fn ix(cols: usize, i: usize, j: usize) -> usize {
        i * cols + j
    }

    // --- Gradient field ---

    /// Four directed differences per cell.
    /// Each is the relation: this cell minus that neighbor.
    pub struct Grad2D {
        pub north: Vec<f64>,  // x[i,j] - x[i-1,j]
        pub south: Vec<f64>,  // x[i,j] - x[i+1,j]
        pub east:  Vec<f64>,  // x[i,j] - x[i,j+1]
        pub west:  Vec<f64>,  // x[i,j] - x[i,j-1]
        pub rows: usize,
        pub cols: usize,
    }

    // --- A ---

    /// A -- Relational Gradient Extraction (2D)
    ///
    /// Produces four directed differences per cell.
    /// No aggregate. Each difference is a direct
    /// relation between this cell and one neighbor.
    pub fn operator_a(field: &[f64], rows: usize, cols: usize)
        -> Grad2D
    {
        let n = rows * cols;
        let mut north = vec![0.0; n];
        let mut south = vec![0.0; n];
        let mut east  = vec![0.0; n];
        let mut west  = vec![0.0; n];

        for i in 0..rows {
            for j in 0..cols {
                let x = field[ix(cols, i, j)];
                let k = ix(cols, i, j);

                north[k] = x - at(field, rows, cols,
                                  i as isize - 1, j as isize);
                south[k] = x - at(field, rows, cols,
                                  i as isize + 1, j as isize);
                east[k]  = x - at(field, rows, cols,
                                  i as isize, j as isize + 1);
                west[k]  = x - at(field, rows, cols,
                                  i as isize, j as isize - 1);
            }
        }

        Grad2D { north, south, east, west, rows, cols }
    }

    // --- B ---

    /// B -- Local Relational Accumulation (2D)
    ///
    /// Each gradient accumulates with the same-direction
    /// gradient at the next cell along that direction.
    ///
    /// South gradient at (i,j) + south gradient at (i+1,j).
    /// North gradient at (i,j) + north gradient at (i-1,j).
    /// East gradient at (i,j) + east gradient at (i,j+1).
    /// West gradient at (i,j) + west gradient at (i,j-1).
    ///
    /// Each direction extends its own relational reach.
    /// No cross-axis coupling.
    pub fn operator_b(g: &Grad2D) -> Grad2D {
        let (rows, cols) = (g.rows, g.cols);
        let n = rows * cols;
        let mut north = vec![0.0; n];
        let mut south = vec![0.0; n];
        let mut east  = vec![0.0; n];
        let mut west  = vec![0.0; n];

        for i in 0..rows {
            for j in 0..cols {
                let k = ix(cols, i, j);

                let i_n = (i + rows - 1) % rows;
                let i_s = (i + 1) % rows;
                let j_e = (j + 1) % cols;
                let j_w = (j + cols - 1) % cols;

                north[k] = g.north[k] + g.north[ix(cols, i_n, j)];
                south[k] = g.south[k] + g.south[ix(cols, i_s, j)];
                east[k]  = g.east[k]  + g.east[ix(cols, i, j_e)];
                west[k]  = g.west[k]  + g.west[ix(cols, i, j_w)];
            }
        }

        Grad2D { north, south, east, west, rows, cols }
    }

    // --- rho ---

    /// Compute per-cell rho from A's gradient output.
    ///
    /// For each cell, rho reflects the strength of
    /// relational gradients at that cell.
    ///
    /// We take the largest absolute gradient among the
    /// four directions rather than a norm. The strongest
    /// single relation determines circulation strength,
    /// not a blended summary of all four.
    ///
    /// rho[i,j] = rho_base * max_grad / (1 + max_grad)
    ///
    /// This avoids computing a Euclidean norm (which is
    /// a statistical reduction of four independent values
    /// into one). The max preserves the identity of the
    /// dominant relation.
    pub fn compute_rho(a: &Grad2D, rho_base: f64) -> Vec<f64> {
        let n = a.rows * a.cols;
        let mut rho = vec![0.0; n];

        for k in 0..n {
            let max_grad = a.north[k].abs()
                .max(a.south[k].abs())
                .max(a.east[k].abs())
                .max(a.west[k].abs());
            rho[k] = rho_base * max_grad / (1.0 + max_grad);
        }
        rho
    }

    // --- R ---

    /// R -- Antisymmetric Circulation (2D)
    ///
    /// Cross-couples gradients between axes.
    /// This is where rotation emerges.
    ///
    /// R takes the accumulated gradient field from B
    /// and produces evolved cell values.
    ///
    /// At each cell, R computes:
    ///
    /// Net zonal relation:
    ///   dz = south - north
    ///   (how the cell's southward relation differs from
    ///    its northward relation -- directional asymmetry
    ///    along the row axis)
    ///
    /// Net meridional relation:
    ///   dm = east - west
    ///   (directional asymmetry along the column axis)
    ///
    /// Circulation cross-term (NH): dz - dm
    ///   Zonal asymmetry drives meridional response.
    ///   Meridional asymmetry drives zonal response.
    ///   Opposite signs: this is rotation, not diffusion.
    ///
    /// Cell evolution:
    ///   The cell's evolved value comes from its relational
    ///   context. Each directed gradient tells us how this
    ///   cell relates to one neighbor. The four gradients
    ///   together define the cell's relational position.
    ///
    ///   We do not sum them (that would be an aggregate).
    ///   Instead, R uses the directional asymmetries (dz, dm)
    ///   which are themselves differences of differences --
    ///   relational all the way down.
    ///
    ///   The evolved value is the circulation term scaled
    ///   by rho. The cell's value becomes its rotational
    ///   contribution to the field.
    pub fn operator_r(bg: &Grad2D, rho: &[f64],
                      hemisphere_north: bool) -> Vec<f64> {
        let (rows, cols) = (bg.rows, bg.cols);
        let mut output = vec![0.0; rows * cols];

        for i in 0..rows {
            for j in 0..cols {
                let k = ix(cols, i, j);

                // Directional asymmetries (differences of differences)
                let dz = bg.south[k] - bg.north[k];
                let dm = bg.east[k] - bg.west[k];

                // Cross-coupling: rotation
                let circ = if hemisphere_north {
                    dz - dm
                } else {
                    -dz + dm
                };

                output[k] = rho[k] * circ;
            }
        }
        output
    }

    // --- C ---

    /// C -- Bounded Coherence (2D)
    ///
    /// C(x) = x / (1 + |x|)
    pub fn operator_c(field: &[f64]) -> Vec<f64> {
        field.iter()
            .map(|&x| x / (1.0 + x.abs()))
            .collect()
    }

    // --- E ---

    /// E -- Composite Evolution (2D)
    ///
    /// E(x, rho_base) = C(R(B(A(x)), rho(A(x))))
    pub fn operator_e(field: &[f64], rows: usize, cols: usize,
                      rho_base: f64, hemisphere_north: bool)
        -> Vec<f64>
    {
        let a = operator_a(field, rows, cols);
        let rho = compute_rho(&a, rho_base);
        let b = operator_b(&a);
        let r = operator_r(&b, &rho, hemisphere_north);
        operator_c(&r)
    }
}


// =======================================================
// 3D -- Periodic 3-Torus
// =======================================================

pub mod three_d {

    // --- Grid access ---

    #[inline]
    fn wrap(i: isize, n: usize) -> usize {
        ((i % n as isize) + n as isize) as usize % n
    }

    #[inline]
    fn at(field: &[f64], d0: usize, d1: usize, d2: usize,
          i: isize, j: isize, k: isize) -> f64 {
        let ri = wrap(i, d0);
        let rj = wrap(j, d1);
        let rk = wrap(k, d2);
        field[ri * d1 * d2 + rj * d2 + rk]
    }

    #[inline]
    fn ix(d1: usize, d2: usize,
          i: usize, j: usize, k: usize) -> usize {
        i * d1 * d2 + j * d2 + k
    }

    // --- Gradient field ---

    /// Six directed differences per cell.
    ///
    /// Axis 0: up/down (vertical -- altitude/pressure)
    /// Axis 1: north/south (meridional -- latitude)
    /// Axis 2: east/west (zonal -- longitude)
    pub struct Grad3D {
        pub up:    Vec<f64>,  // x[i,j,k] - x[i-1,j,k]
        pub down:  Vec<f64>,  // x[i,j,k] - x[i+1,j,k]
        pub north: Vec<f64>,  // x[i,j,k] - x[i,j-1,k]
        pub south: Vec<f64>,  // x[i,j,k] - x[i,j+1,k]
        pub east:  Vec<f64>,  // x[i,j,k] - x[i,j,k+1]
        pub west:  Vec<f64>,  // x[i,j,k] - x[i,j,k-1]
        pub d0: usize,
        pub d1: usize,
        pub d2: usize,
    }

    // --- A ---

    /// A -- Relational Gradient Extraction (3D)
    ///
    /// Six directed differences per cell.
    pub fn operator_a(field: &[f64], d0: usize, d1: usize,
                      d2: usize) -> Grad3D {
        let n = d0 * d1 * d2;
        let mut up    = vec![0.0; n];
        let mut down  = vec![0.0; n];
        let mut north = vec![0.0; n];
        let mut south = vec![0.0; n];
        let mut east  = vec![0.0; n];
        let mut west  = vec![0.0; n];

        for i in 0..d0 {
            for j in 0..d1 {
                for k in 0..d2 {
                    let x = field[ix(d1, d2, i, j, k)];
                    let p = ix(d1, d2, i, j, k);

                    up[p]    = x - at(field, d0, d1, d2,
                                      i as isize - 1,
                                      j as isize, k as isize);
                    down[p]  = x - at(field, d0, d1, d2,
                                      i as isize + 1,
                                      j as isize, k as isize);
                    north[p] = x - at(field, d0, d1, d2,
                                      i as isize,
                                      j as isize - 1, k as isize);
                    south[p] = x - at(field, d0, d1, d2,
                                      i as isize,
                                      j as isize + 1, k as isize);
                    east[p]  = x - at(field, d0, d1, d2,
                                      i as isize,
                                      j as isize, k as isize + 1);
                    west[p]  = x - at(field, d0, d1, d2,
                                      i as isize,
                                      j as isize, k as isize - 1);
                }
            }
        }

        Grad3D { up, down, north, south, east, west, d0, d1, d2 }
    }

    // --- B ---

    /// B -- Local Relational Accumulation (3D)
    ///
    /// Each of six gradients accumulates along its own axis.
    pub fn operator_b(g: &Grad3D) -> Grad3D {
        let (d0, d1, d2) = (g.d0, g.d1, g.d2);
        let n = d0 * d1 * d2;
        let mut up    = vec![0.0; n];
        let mut down  = vec![0.0; n];
        let mut north = vec![0.0; n];
        let mut south = vec![0.0; n];
        let mut east  = vec![0.0; n];
        let mut west  = vec![0.0; n];

        for i in 0..d0 {
            for j in 0..d1 {
                for k in 0..d2 {
                    let p = ix(d1, d2, i, j, k);

                    let i_up   = (i + d0 - 1) % d0;
                    let i_down = (i + 1) % d0;
                    let j_n    = (j + d1 - 1) % d1;
                    let j_s    = (j + 1) % d1;
                    let k_e    = (k + 1) % d2;
                    let k_w    = (k + d2 - 1) % d2;

                    up[p]    = g.up[p]
                        + g.up[ix(d1, d2, i_up, j, k)];
                    down[p]  = g.down[p]
                        + g.down[ix(d1, d2, i_down, j, k)];
                    north[p] = g.north[p]
                        + g.north[ix(d1, d2, i, j_n, k)];
                    south[p] = g.south[p]
                        + g.south[ix(d1, d2, i, j_s, k)];
                    east[p]  = g.east[p]
                        + g.east[ix(d1, d2, i, j, k_e)];
                    west[p]  = g.west[p]
                        + g.west[ix(d1, d2, i, j, k_w)];
                }
            }
        }

        Grad3D { up, down, north, south, east, west,
                 d0, d1, d2 }
    }

    // --- rho ---

    /// Per-cell rho from A's gradient output.
    ///
    /// Takes the largest absolute gradient among the six
    /// directions. The dominant relation determines
    /// circulation strength.
    pub fn compute_rho(a: &Grad3D, rho_base: f64) -> Vec<f64> {
        let n = a.d0 * a.d1 * a.d2;
        let mut rho = vec![0.0; n];

        for p in 0..n {
            let max_grad = a.up[p].abs()
                .max(a.down[p].abs())
                .max(a.north[p].abs())
                .max(a.south[p].abs())
                .max(a.east[p].abs())
                .max(a.west[p].abs());
            rho[p] = rho_base * max_grad / (1.0 + max_grad);
        }
        rho
    }

    // --- R ---

    /// R -- Antisymmetric Circulation (3D)
    ///
    /// Three circulation planes, each a cross-coupling
    /// of directional asymmetries between two axes:
    ///
    /// Axis1-Axis2 (lat-lon): Coriolis, hemisphere-dependent
    ///   d_merid = south - north
    ///   d_zonal = east - west
    ///   circ = d_merid - d_zonal  (NH)
    ///
    /// Axis0-Axis1 (vertical-meridional): convective circulation
    ///   d_vert = down - up
    ///   d_merid = south - north
    ///   circ = d_vert - d_merid
    ///
    /// Axis0-Axis2 (vertical-zonal): vertical-zonal circulation
    ///   d_vert = down - up
    ///   d_zonal = east - west
    ///   circ = d_vert - d_zonal
    ///
    /// All terms are differences of differences.
    /// Relational throughout.
    pub fn operator_r(bg: &Grad3D, rho: &[f64],
                      hemisphere_north: bool) -> Vec<f64> {
        let n = bg.d0 * bg.d1 * bg.d2;
        let mut output = vec![0.0; n];

        for p in 0..n {
            let d_vert  = bg.down[p]  - bg.up[p];
            let d_merid = bg.south[p] - bg.north[p];
            let d_zonal = bg.east[p]  - bg.west[p];

            let circ_latlon = if hemisphere_north {
                d_merid - d_zonal
            } else {
                -d_merid + d_zonal
            };

            let circ_vert_merid = d_vert - d_merid;
            let circ_vert_zonal = d_vert - d_zonal;

            output[p] = rho[p]
                * (circ_latlon + circ_vert_merid + circ_vert_zonal);
        }
        output
    }

    // --- C ---

    /// C -- Bounded Coherence (3D)
    pub fn operator_c(field: &[f64]) -> Vec<f64> {
        field.iter()
            .map(|&x| x / (1.0 + x.abs()))
            .collect()
    }

    // --- E ---

    /// E -- Composite Evolution (3D)
    pub fn operator_e(field: &[f64], d0: usize, d1: usize,
                      d2: usize, rho_base: f64,
                      hemisphere_north: bool) -> Vec<f64> {
        let a = operator_a(field, d0, d1, d2);
        let rho = compute_rho(&a, rho_base);
        let b = operator_b(&a);
        let r = operator_r(&b, &rho, hemisphere_north);
        operator_c(&r)
    }
}


// =======================================================
// TYPE SIGNATURES
// =======================================================
//
// Let G(d) denote the gradient field at dimensionality d.
//   G(1) = Vec<f64>         (one difference per cell)
//   G(2) = Grad2D           (four differences per cell)
//   G(3) = Grad3D           (six differences per cell)
//
// All dimensionalities:
//   A : D -> G(d)
//   B : G(d) -> G(d)
//   R : G(d) x rho -> D
//   C : D -> D
//   E : D x R -> D
//
// The intermediate representation between A and R is
// relational (G). Not statistical. Not scalar.
//
// R is the operator that converts relational representation
// back to field representation. It does so through
// cross-axis coupling (circulation), not through aggregation.
//
// =======================================================
