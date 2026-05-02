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
// The operators do not replace existing methods;
// they constrain how any method may operate on relational
// structure.
//
// =======================================================
// REPRESENTATION TYPES
// =======================================================
//
// NodeField: one scalar per index. The measured data as
//   produced by M : O -> D. This is the object-form
//   representation. Its informational content resides in
//   the relational equivalence class [x]_tau, not in the
//   per-index values themselves.
//
// EdgeField: one scalar per directed adjacency pair. The
//   relational representation. Each element is a direct
//   pairwise difference between adjacent indices. This is
//   the working representation of the operator sequence.
//
// These types are not interchangeable. The type system
// enforces the representation discipline proved in the
// Object Error paper:
//
//   - A is the unique transition from NodeField to EdgeField.
//   - B, R, C operate on EdgeField only.
//   - Projection (EdgeField -> NodeField) requires an
//     explicit, declared Projection implementation.
//   - No other NodeField -> EdgeField path exists.
//   - No implicit EdgeField -> NodeField path exists.
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
// A : NodeField -> EdgeField
//     Extracts directed pairwise differences.
//     This is the unique transition from object-form
//     to relational-form representation.
//
// B : EdgeField -> EdgeField
//     Accumulates each directed gradient with the
//     same-direction gradient at the next cell along
//     that direction. Extends relational reach.
//     No cross-axis coupling. That is R's function.
//
// R : EdgeField x rho -> EdgeField
//     Cross-couples gradients between perpendicular axes
//     through additive antisymmetric circulation.
//     Each edge retains its value from B and receives
//     an additive term from perpendicular-axis asymmetry.
//     No replacement. No magnitude-weighted distribution.
//     No pooling of circulation planes.
//
// C : EdgeField -> EdgeField
//     Bounds output at the cell level. All edges at a
//     cell share the same denominator: 1 + max_grad,
//     where max_grad is the maximum absolute edge value
//     at that cell. Preserves relative proportions
//     between edges at a cell. Output in (-1, 1).
//
//     In 1D, each cell owns one edge, so cell-level
//     and per-edge bounding are equivalent:
//       C(e)[i] = e[i] / (1 + |e[i]|)
//
//     In >=2D, per-edge bounding destroys relative
//     proportions between directional edges at a cell.
//     Cell-level bounding preserves them:
//       C(e_d)[i] = e_d[i] / (1 + max_grad[i])
//
// rho is spatially varying, derived from A's edge output.
// rho is computed outside the operator sequence as a
// derived quantity. It is not a statistical aggregate of
// the field -- it is a per-cell measure of relational
// gradient strength at that cell.
//
// =======================================================
// R STRUCTURAL INVARIANT (ALL DIMENSIONS)
// =======================================================
//
// The 1D form of R is:
//   R(e)[i] = e[i] + rho * (forward - backward)
//
// Properties:
//   - Additive: edge retains its B output
//   - Antisymmetric: forward minus backward
//   - Local: depends only on immediate neighbors
//   - No replacement, no distribution, no aggregation
//
// In >=2D, this generalizes to:
//   R(e_d)[i] = e_d[i] + rho[i] * A_d(i)
//
// where A_d(i) is the perpendicular-axis asymmetry
// relevant to direction d at cell i.
//
// In 2D:
//   Meridional edges += rho * zonal_asymmetry
//   Zonal edges += rho * meridional_asymmetry
//   (sign determined by hemisphere convention)
//
// In 3D:
//   Each axis receives the difference of asymmetries
//   from the other two axes. The three additive terms
//   sum to zero: R introduces no net energy, only
//   redistribution across axes.
//
// =======================================================
// C STRUCTURAL INVARIANT (ALL DIMENSIONS)
// =======================================================
//
// Let N(i) be the set of directed edges at index i.
// Define: m(i) = max_{e in N(i)} |e|
//
// Then: C(e_d)[i] = e_d[i] / (1 + m(i))
//
// Properties:
//   - Bounded: |C(e_d)| < 1
//   - Sign-preserving: sign(C(e_d)) = sign(e_d)
//   - Monotone in each component
//   - Ratio-preserving: C(e_d1)/C(e_d2) = e_d1/e_d2
//     for any directions d1, d2 at the same cell
//   - No cross-cell coupling
//   - No aggregation across directions
//
// In 1D: m(i) = |e[i]| (one edge per index),
//   so C(e)[i] = e[i] / (1 + |e[i]|) — unchanged.
//
// In >=2D: m(i) = max over all directional edges at i.
//   All edges at a cell share the same denominator.
//
// =======================================================
// PROJECTION
// =======================================================
//
// The output of E is an EdgeField. Applications requiring
// node-level quantities must apply an explicit Projection.
//
// Every Projection is lossy. The Projection trait requires
// declaration of preserved and discarded invariants.
//
// No implicit EdgeField -> NodeField conversion exists.
//
// =======================================================

#![allow(dead_code)]


// =======================================================
// Representation Types
// =======================================================

/// Node field: one scalar per index.
/// The object-form representation as produced by M : O -> D.
/// Only operator A accepts this type.
#[derive(Clone, Debug)]
pub struct NodeField(pub Vec<f64>);

/// Edge field (1D): one scalar per directed adjacency pair.
/// The relational representation. All operators B, R, C
/// operate on this type.
#[derive(Clone, Debug)]
pub struct EdgeField(pub Vec<f64>);


// =======================================================
// Projection Trait
// =======================================================

/// Projection from edge field to node field.
///
/// Every projection is lossy. Implementations must declare
/// which structural properties survive and which are destroyed.
///
/// This is the only pathway from EdgeField to NodeField.
pub trait Projection {
    fn apply(&self, e: &EdgeField) -> NodeField;

    /// Structural properties of the edge field that survive
    /// this projection.
    fn preserves(&self) -> &'static [&'static str];

    /// Structural properties of the edge field that are
    /// destroyed by this projection.
    fn discards(&self) -> &'static [&'static str];
}

/// Explicit projection function.
/// Forces all EdgeField -> NodeField transitions through
/// a declared Projection implementation.
pub fn project<P: Projection>(p: &P, e: &EdgeField) -> NodeField {
    p.apply(e)
}


// =======================================================
// 1D -- Periodic Ring
// =======================================================

pub mod one_d {
    use super::{NodeField, EdgeField};

    /// A -- Relational Gradient Extraction
    ///
    /// A(x)[i] = x[i] - x[(i+1) mod n]
    ///
    /// Directed difference to forward neighbor.
    /// Zero-sum on a ring by telescoping.
    /// If field is uniform, A(x) = 0 everywhere.
    ///
    /// This is the unique transition from NodeField to EdgeField.
    /// No other operator accepts NodeField.
    pub fn operator_a(field: &NodeField) -> EdgeField {
        let f = &field.0;
        let n = f.len();
        EdgeField(
            (0..n)
                .map(|i| f[i] - f[(i + 1) % n])
                .collect()
        )
    }

    /// B -- Local Relational Accumulation
    ///
    /// B(e)[i] = e[i] + e[(i+1) mod n]
    ///
    /// Pairwise coupling of adjacent edges.
    /// Extends relational reach by one cell.
    /// When applied to A's output, the gradient at i
    /// accumulates with the gradient at i+1.
    pub fn operator_b(edges: &EdgeField) -> EdgeField {
        let e = &edges.0;
        let n = e.len();
        EdgeField(
            (0..n)
                .map(|i| e[i] + e[(i + 1) % n])
                .collect()
        )
    }

    /// R -- Antisymmetric Circulation
    ///
    /// R(e)[i] = e[i] + rho * (e[(i+1) mod n] - e[(i-1) mod n])
    ///
    /// Forward-backward difference on edges. Antisymmetric.
    /// Additive: edge retains its value from B.
    /// Maintains nonzero differences under iteration.
    /// rho in (0.0, 0.5].
    pub fn operator_r(edges: &EdgeField, rho: f64) -> EdgeField {
        let e = &edges.0;
        let n = e.len();
        EdgeField(
            (0..n)
                .map(|i| {
                    let fwd = e[(i + 1) % n];
                    let bwd = e[(i + n - 1) % n];
                    e[i] + rho * (fwd - bwd)
                })
                .collect()
        )
    }

    /// C -- Bounded Coherence
    ///
    /// C(e)[i] = e[i] / (1 + |e[i]|)
    ///
    /// Output in (-1, 1). Odd function. No clamping.
    ///
    /// In 1D, each index owns one edge. Cell-level and
    /// per-edge bounding are equivalent: m(i) = |e[i]|.
    pub fn operator_c(edges: &EdgeField) -> EdgeField {
        EdgeField(
            edges.0.iter()
                .map(|&x| x / (1.0 + x.abs()))
                .collect()
        )
    }

    /// Compute rho from A's edge output.
    ///
    /// rho[i] = rho_base * |a[i]| / (1 + |a[i]|)
    ///
    /// Per-cell. Not an aggregate. Each cell's rho reflects
    /// the strength of its own relational gradient.
    pub fn compute_rho(a: &EdgeField, rho_base: f64) -> Vec<f64> {
        a.0.iter()
            .map(|&v| {
                let mag = v.abs();
                rho_base * mag / (1.0 + mag)
            })
            .collect()
    }

    /// R with spatially varying rho.
    pub fn operator_r_varying(edges: &EdgeField, rho: &[f64]) -> EdgeField {
        let e = &edges.0;
        let n = e.len();
        EdgeField(
            (0..n)
                .map(|i| {
                    let fwd = e[(i + 1) % n];
                    let bwd = e[(i + n - 1) % n];
                    e[i] + rho[i] * (fwd - bwd)
                })
                .collect()
        )
    }

    /// E -- Composite Evolution
    ///
    /// E(x, rho_base) = C(R(B(A(x)), rho(A(x))))
    ///
    /// Returns EdgeField. Applications requiring NodeField
    /// must apply an explicit Projection.
    pub fn operator_e(field: &NodeField, rho_base: f64) -> EdgeField {
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
    use super::NodeField;

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

    // --- Edge field (2D) ---

    /// Four directed differences per cell.
    /// Each is the relation: this cell minus that neighbor.
    ///
    /// This is the 2D EdgeField. Each component is a
    /// directed edge along one axis. The four components
    /// together define the relational position of each cell.
    #[derive(Clone, Debug)]
    pub struct EdgeField2D {
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
    ///
    /// Unique transition from NodeField to EdgeField2D.
    pub fn operator_a(field: &NodeField, rows: usize, cols: usize)
        -> EdgeField2D
    {
        let f = &field.0;
        let n = rows * cols;
        let mut north = vec![0.0; n];
        let mut south = vec![0.0; n];
        let mut east  = vec![0.0; n];
        let mut west  = vec![0.0; n];

        for i in 0..rows {
            for j in 0..cols {
                let x = f[ix(cols, i, j)];
                let k = ix(cols, i, j);

                north[k] = x - at(f, rows, cols,
                                  i as isize - 1, j as isize);
                south[k] = x - at(f, rows, cols,
                                  i as isize + 1, j as isize);
                east[k]  = x - at(f, rows, cols,
                                  i as isize, j as isize + 1);
                west[k]  = x - at(f, rows, cols,
                                  i as isize, j as isize - 1);
            }
        }

        EdgeField2D { north, south, east, west, rows, cols }
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
    pub fn operator_b(g: &EdgeField2D) -> EdgeField2D {
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

        EdgeField2D { north, south, east, west, rows, cols }
    }

    // --- rho ---

    /// Compute per-cell rho from A's edge output.
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
    pub fn compute_rho(a: &EdgeField2D, rho_base: f64) -> Vec<f64> {
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
    /// Additive cross-coupling between perpendicular axes.
    /// Each edge retains its value from B and receives
    /// an additive circulation term from the perpendicular-
    /// axis asymmetry at its own cell.
    ///
    /// This is the 2D generalization of the 1D form:
    ///   R(e)[i] = e[i] + rho * (forward - backward)
    ///
    /// In 2D, "forward minus backward" becomes
    /// "perpendicular-axis asymmetry":
    ///
    ///   Meridional edges (north, south) receive
    ///   zonal asymmetry: east - west
    ///
    ///   Zonal edges (east, west) receive
    ///   meridional asymmetry: south - north
    ///
    /// Properties preserved from 1D:
    ///   - Additive: edge keeps its B output
    ///   - Antisymmetric: asymmetry is a difference
    ///   - Local: only this cell's edges
    ///   - No replacement, no distribution, no aggregation
    ///   - No magnitude-weighted fraction
    ///   - No pooling of asymmetries into a single scalar
    ///
    /// Hemisphere sign convention:
    ///   NH: meridional += zonal_asym, zonal += merid_asym
    ///   SH: signs reversed
    pub fn operator_r(bg: &EdgeField2D, rho: &[f64],
                      hemisphere_north: bool) -> EdgeField2D {
        let (rows, cols) = (bg.rows, bg.cols);
        let n = rows * cols;

        let mut north = vec![0.0; n];
        let mut south = vec![0.0; n];
        let mut east  = vec![0.0; n];
        let mut west  = vec![0.0; n];

        let sign: f64 = if hemisphere_north { 1.0 } else { -1.0 };

        for i in 0..rows {
            for j in 0..cols {
                let k = ix(cols, i, j);

                // Perpendicular-axis asymmetries at this cell
                let zonal_asym = bg.east[k] - bg.west[k];
                let merid_asym = bg.south[k] - bg.north[k];

                // Additive: each edge keeps its B value
                // and receives perpendicular-axis circulation
                north[k] = bg.north[k] + sign * rho[k] * zonal_asym;
                south[k] = bg.south[k] + sign * rho[k] * zonal_asym;
                east[k]  = bg.east[k]  + sign * rho[k] * merid_asym;
                west[k]  = bg.west[k]  + sign * rho[k] * merid_asym;
            }
        }

        EdgeField2D { north, south, east, west, rows, cols }
    }

    // --- C ---

    /// C -- Bounded Coherence (2D)
    ///
    /// Cell-level bounding. All edges at a cell share
    /// the same denominator: 1 + max_grad, where max_grad
    /// is the maximum absolute edge value at that cell.
    ///
    /// C(e_d)[k] = e_d[k] / (1 + m(k))
    /// where m(k) = max(|north|, |south|, |east|, |west|)
    ///
    /// This preserves the relative proportions between
    /// directional edges at a cell. Per-edge bounding
    /// (the 1D formula applied independently to each
    /// direction) destroys these proportions in >=2D.
    ///
    /// Properties:
    ///   - Bounded: |C(e_d)| < 1
    ///   - Sign-preserving
    ///   - Monotone in each component
    ///   - Ratio-preserving: C(e_d1)/C(e_d2) = e_d1/e_d2
    ///   - No cross-cell coupling
    ///   - No aggregation across directions
    ///   - C(0) = 0
    pub fn operator_c(g: &EdgeField2D) -> EdgeField2D {
        let n = g.rows * g.cols;
        let mut north = vec![0.0; n];
        let mut south = vec![0.0; n];
        let mut east  = vec![0.0; n];
        let mut west  = vec![0.0; n];

        for k in 0..n {
            let max_grad = g.north[k].abs()
                .max(g.south[k].abs())
                .max(g.east[k].abs())
                .max(g.west[k].abs());
            let denom = 1.0 + max_grad;

            north[k] = g.north[k] / denom;
            south[k] = g.south[k] / denom;
            east[k]  = g.east[k]  / denom;
            west[k]  = g.west[k]  / denom;
        }

        EdgeField2D { north, south, east, west,
                     rows: g.rows, cols: g.cols }
    }

    // --- E ---

    /// E -- Composite Evolution (2D)
    ///
    /// E(x, rho_base) = C(R(B(A(x)), rho(A(x))))
    ///
    /// Returns EdgeField2D. Applications requiring NodeField
    /// must apply an explicit Projection.
    pub fn operator_e(field: &NodeField, rows: usize, cols: usize,
                      rho_base: f64, hemisphere_north: bool)
        -> EdgeField2D
    {
        let a = operator_a(field, rows, cols);
        let rho = compute_rho(&a, rho_base);
        let b = operator_b(&a);
        let r = operator_r(&b, &rho, hemisphere_north);
        operator_c(&r)
    }

    // --- Projection implementations ---

    use super::{Projection, NodeField};

    /// Node sum projection: sum of all incident edge magnitudes.
    /// Preserves net imbalance at each index.
    /// Discards directional distribution.
    pub struct NodeSumProjection;

    impl Projection for NodeSumProjection {
        fn apply(&self, _e: &super::EdgeField) -> NodeField {
            // 1D projection -- for 2D, use project_2d below.
            unimplemented!("Use project_2d for 2D edge fields")
        }

        fn preserves(&self) -> &'static [&'static str] {
            &["net imbalance per cell"]
        }

        fn discards(&self) -> &'static [&'static str] {
            &["directional distribution"]
        }
    }

    impl NodeSumProjection {
        /// Project a 2D edge field to a node field by summing
        /// all four directional edges at each cell.
        pub fn project_2d(&self, g: &EdgeField2D) -> NodeField {
            let n = g.rows * g.cols;
            let mut out = vec![0.0; n];
            for k in 0..n {
                out[k] = g.north[k] + g.south[k]
                       + g.east[k] + g.west[k];
            }
            NodeField(out)
        }
    }

    /// Directional projection: extract a single directional
    /// edge component as a node field.
    pub enum Direction { North, South, East, West }

    pub struct DirectionalProjection {
        pub direction: Direction,
    }

    impl DirectionalProjection {
        pub fn project_2d(&self, g: &EdgeField2D) -> NodeField {
            let data = match self.direction {
                Direction::North => &g.north,
                Direction::South => &g.south,
                Direction::East  => &g.east,
                Direction::West  => &g.west,
            };
            NodeField(data.clone())
        }

        pub fn preserves(&self) -> &'static [&'static str] {
            &["single-direction gradient"]
        }

        pub fn discards(&self) -> &'static [&'static str] {
            &["all other directional gradients"]
        }
    }
}


// =======================================================
// 3D -- Periodic 3-Torus
// =======================================================

pub mod three_d {
    use super::NodeField;

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

    // --- Edge field (3D) ---

    /// Six directed differences per cell.
    ///
    /// Axis 0: up/down (vertical -- altitude/pressure)
    /// Axis 1: north/south (meridional -- latitude)
    /// Axis 2: east/west (zonal -- longitude)
    #[derive(Clone, Debug)]
    pub struct EdgeField3D {
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
    /// Unique transition from NodeField to EdgeField3D.
    pub fn operator_a(field: &NodeField, d0: usize, d1: usize,
                      d2: usize) -> EdgeField3D {
        let f = &field.0;
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
                    let x = f[ix(d1, d2, i, j, k)];
                    let p = ix(d1, d2, i, j, k);

                    up[p]    = x - at(f, d0, d1, d2,
                                      i as isize - 1,
                                      j as isize, k as isize);
                    down[p]  = x - at(f, d0, d1, d2,
                                      i as isize + 1,
                                      j as isize, k as isize);
                    north[p] = x - at(f, d0, d1, d2,
                                      i as isize,
                                      j as isize - 1, k as isize);
                    south[p] = x - at(f, d0, d1, d2,
                                      i as isize,
                                      j as isize + 1, k as isize);
                    east[p]  = x - at(f, d0, d1, d2,
                                      i as isize,
                                      j as isize, k as isize + 1);
                    west[p]  = x - at(f, d0, d1, d2,
                                      i as isize,
                                      j as isize, k as isize - 1);
                }
            }
        }

        EdgeField3D { up, down, north, south, east, west, d0, d1, d2 }
    }

    // --- B ---

    /// B -- Local Relational Accumulation (3D)
    ///
    /// Each of six gradients accumulates along its own axis.
    pub fn operator_b(g: &EdgeField3D) -> EdgeField3D {
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

        EdgeField3D { up, down, north, south, east, west,
                     d0, d1, d2 }
    }

    // --- rho ---

    /// Per-cell rho from A's edge output.
    ///
    /// Takes the largest absolute gradient among the six
    /// directions. The dominant relation determines
    /// circulation strength.
    pub fn compute_rho(a: &EdgeField3D, rho_base: f64) -> Vec<f64> {
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
    /// Additive cross-coupling between perpendicular axes.
    /// Three axis pairs, each producing independent
    /// circulation contributions. No pooling into a
    /// single scalar. No magnitude-weighted distribution.
    ///
    /// Axis asymmetries:
    ///   A_vert  = down - up       (vertical asymmetry)
    ///   A_merid = south - north   (meridional asymmetry)
    ///   A_zonal = east - west     (zonal asymmetry)
    ///
    /// Cross-coupling (each axis receives the difference
    /// of asymmetries from the other two axes):
    ///
    ///   Vertical edges (up, down):
    ///     += rho * (A_merid - A_zonal)
    ///
    ///   Meridional edges (north, south):
    ///     += rho * (A_zonal - A_vert)
    ///
    ///   Zonal edges (east, west):
    ///     += rho * (A_vert - A_merid)
    ///
    /// The three additive terms sum to zero:
    ///   (A_merid - A_zonal) + (A_zonal - A_vert)
    ///     + (A_vert - A_merid) = 0
    ///
    /// R introduces no net energy, only redistribution
    /// across axes. This is a structural property, not
    /// a coincidence.
    ///
    /// Properties preserved from 1D:
    ///   - Additive: each edge keeps its B output
    ///   - Antisymmetric: all terms are differences
    ///   - Local: only this cell's edges
    ///   - No replacement, no distribution, no aggregation
    pub fn operator_r(bg: &EdgeField3D, rho: &[f64],
                      hemisphere_north: bool) -> EdgeField3D {
        let n = bg.d0 * bg.d1 * bg.d2;

        let mut up    = vec![0.0; n];
        let mut down  = vec![0.0; n];
        let mut north = vec![0.0; n];
        let mut south = vec![0.0; n];
        let mut east  = vec![0.0; n];
        let mut west  = vec![0.0; n];

        let sign: f64 = if hemisphere_north { 1.0 } else { -1.0 };

        for p in 0..n {
            // Axis asymmetries at this cell
            let a_vert  = bg.down[p]  - bg.up[p];
            let a_merid = bg.south[p] - bg.north[p];
            let a_zonal = bg.east[p]  - bg.west[p];

            // Each axis receives difference of asymmetries
            // from the other two axes. Additive, no pooling.
            let circ_vert  = sign * rho[p] * (a_merid - a_zonal);
            let circ_merid = sign * rho[p] * (a_zonal - a_vert);
            let circ_zonal = sign * rho[p] * (a_vert  - a_merid);

            // Additive: edge keeps its B value
            up[p]    = bg.up[p]    + circ_vert;
            down[p]  = bg.down[p]  + circ_vert;
            north[p] = bg.north[p] + circ_merid;
            south[p] = bg.south[p] + circ_merid;
            east[p]  = bg.east[p]  + circ_zonal;
            west[p]  = bg.west[p]  + circ_zonal;
        }

        EdgeField3D { up, down, north, south, east, west,
                     d0: bg.d0, d1: bg.d1, d2: bg.d2 }
    }

    // --- C ---

    /// C -- Bounded Coherence (3D)
    ///
    /// Cell-level bounding. All six edges at a cell share
    /// the same denominator: 1 + max_grad.
    ///
    /// C(e_d)[p] = e_d[p] / (1 + m(p))
    /// where m(p) = max(|up|, |down|, |north|, |south|,
    ///                  |east|, |west|) at cell p.
    ///
    /// Properties: bounded, sign-preserving, monotone,
    /// ratio-preserving, no cross-cell coupling.
    pub fn operator_c(g: &EdgeField3D) -> EdgeField3D {
        let n = g.d0 * g.d1 * g.d2;
        let mut up    = vec![0.0; n];
        let mut down  = vec![0.0; n];
        let mut north = vec![0.0; n];
        let mut south = vec![0.0; n];
        let mut east  = vec![0.0; n];
        let mut west  = vec![0.0; n];

        for p in 0..n {
            let max_grad = g.up[p].abs()
                .max(g.down[p].abs())
                .max(g.north[p].abs())
                .max(g.south[p].abs())
                .max(g.east[p].abs())
                .max(g.west[p].abs());
            let denom = 1.0 + max_grad;

            up[p]    = g.up[p]    / denom;
            down[p]  = g.down[p]  / denom;
            north[p] = g.north[p] / denom;
            south[p] = g.south[p] / denom;
            east[p]  = g.east[p]  / denom;
            west[p]  = g.west[p]  / denom;
        }

        EdgeField3D { up, down, north, south, east, west,
                     d0: g.d0, d1: g.d1, d2: g.d2 }
    }

    // --- E ---

    /// E -- Composite Evolution (3D)
    ///
    /// Returns EdgeField3D. Applications requiring NodeField
    /// must apply an explicit Projection.
    pub fn operator_e(field: &NodeField, d0: usize, d1: usize,
                      d2: usize, rho_base: f64,
                      hemisphere_north: bool) -> EdgeField3D {
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
// Let E(d) denote the edge field at dimensionality d.
//   E(1) = EdgeField        (one directed edge per cell)
//   E(2) = EdgeField2D      (four directed edges per cell)
//   E(3) = EdgeField3D      (six directed edges per cell)
//
// All dimensionalities:
//   A : NodeField -> E(d)       (unique node-to-edge transition)
//   B : E(d) -> E(d)            (edge-to-edge accumulation)
//   R : E(d) x rho -> E(d)     (edge-to-edge circulation)
//   C : E(d) -> E(d)            (edge-to-edge bounded coherence)
//   E : NodeField x R -> E(d)  (composite: node in, edge out)
//
// The intermediate representation between A and C is
// relational (edge fields). Not statistical. Not scalar.
// Not node-level.
//
// R is the operator that cross-couples directional edges.
// It does so through additive antisymmetric circulation,
// not through aggregation or magnitude-weighted distribution.
// Each edge retains its value from B and receives an
// additive term from perpendicular-axis asymmetry.
// Its output is an edge field, preserving directional
// structure.
//
// C bounds relational magnitude at the cell level using
// the strongest local relation as the denominator.
// All edges at a cell share the same denominator,
// preserving their relative proportions. In 1D, this
// is equivalent to per-edge bounding (one edge per cell).
// In >=2D, cell-level bounding is structurally distinct
// from per-edge bounding and is the correct form.
//
// Transition from edge field to node field requires an
// explicit Projection implementation declaring preserved
// and discarded invariants.
//
// No implicit EdgeField -> NodeField conversion exists.
//
// =======================================================
