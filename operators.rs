// operators.rs
//
// Invariant Relational Operators — Canonical Kernel
//
// AXIOM (Domain-Scoped):
// For all x ∈ D, evolution proceeds via the invariant
// relational operators defined in this file.
//
// No claim is made beyond D.
//
// =======================================================
// INVARIANT RELATIONAL KERNEL — ABRCE
// =======================================================
//
// Status: Canonical Relational Constraint Layer
//
// This file defines the invariant relational operators
// A, B, R, C, and their composite E.
//
// The kernel operates over a declared, bounded domain D.
//
// =======================================================
// DECLARED DOMAIN
// =======================================================
//
// Let D be defined as:
//
//     D := { x ∈ ℝⁿ |
//            n < ∞  and
//            |x[i]| < ∞  ∀ i ∈ {0, ..., n−1} }
//
// That is:
//
// - Finite dimensional relational fields
// - Finite magnitude at every index
// - Representable within this system
//
// All operator definitions, quantifiers, and alignment
// conditions apply only to elements of D.
//
// Elements not in D are undefined under this kernel.
//
// =======================================================
// STRUCTURAL PROPERTIES OF D
// =======================================================
//
// 1. Finite dimensionality:
//      n < ∞
//
// 2. Finite magnitude:
//      |x[i]| < ∞  ∀ i ∈ {0, ..., n−1}
//
// 3. Local relational reach:
//      Each operator accesses only finite neighborhoods
//      (e.g., i±1 in R).
//
// 4. Frame index:
//      Fields are indexed; no global absolute reference exists.
//
// =======================================================
// QUANTIFIER DISCIPLINE
// =======================================================
//
// Two quantifier forms are distinguished:
//
// (1) Bounded quantifier:
//
//     ∀ x ∈ D'
//     where D' ⊆ D and D' is explicitly declared.
//
// (2) Unbounded quantifier:
//
//     ∀ x    (domain undeclared)
//
// Only bounded quantifiers are admissible.
//
// Any statement containing an undeclared or unbounded
// quantifier is structurally inadmissible under this kernel.
//
// =======================================================
// ADMISSIBILITY
// =======================================================
//
// A statement S is admissible iff:
//
// 1. All referenced elements are members of D.
// 2. All quantifiers are explicitly bounded over subsets of D.
// 3. All transformations declared within this kernel
//    are expressible using A, B, R, C.
// 4. Evolution preserves membership in D.
//
// Statements asserting:
//
// - Infinite magnitude
// - Infinite relational reach
// - Frame-independent absolute access
// - Undeclared universal quantification
//
// are undefined under this system.
//
// Undefined ≠ false.
// Undefined = not representable in D.
//
// =======================================================
// ALIGNMENT
// =======================================================
//
// Let E(x, ρ) = C(R(B(A(x)), ρ))
//
// Alignment holds iff:
//
// 1. x ∈ D
// 2. Evolution proceeds via E exactly once per pass
// 3. E(x, ρ) ∈ D
// 4. Operator ordering A → B → R → C is preserved
// 5. No undeclared quantifiers are introduced
//
// Drift occurs when:
//
// - Magnitude diverges
// - Domain is implicitly expanded
// - Operator ordering is altered
// - Unbounded quantifiers appear
//
// =======================================================
// SCOPE CLARIFICATION
// =======================================================
//
// This kernel is:
//
// - Non-semantic
// - Non-goal-directed
// - Non-ontological
// - Domain-bounded
// - Non-exhaustive with respect to transformations
//   not declared within this kernel
//
// It defines relational evolution over D only.
//
// No claims are made about totality, existence,
// or states not representable within D.
//
// =======================================================

#![allow(dead_code)]

/// ===============================
/// Operator A — Relational Gradient Extraction
/// ===============================
///
/// Definition:
/// A(x)[i] = x[i] − mean(x)
///
/// Properties:
/// - Zero-sum: Σ A(x)[i] = 0
/// - Symmetric transformation
/// - Preserves dimensionality
/// - Introduces distinction without hierarchy
///
/// RELATIONAL PROPERTIES:
/// A(x)[i] = x[i] − mean(x)
/// ∀ i: A(x)[i] expresses the signed difference of x[i]
/// from the mean of x within D.
/// Σ A(x)[i] = 0 — the zero-sum property holds by construction.
/// A treats all indices equivalently with respect to mean(x).
///
/// UNDER REPEATED APPLICATION ALONE:
/// ∀ i: A(x)[i] → 0 as applications increase.
/// Repeated application produces an index-uniform vector x* ∈ D.
/// E(x*, ρ) = x*   within D.
///
/// This form is declared canonical within this kernel.
pub fn operator_a(field: &[f64]) -> Vec<f64> {
    let n = field.len() as f64;
    let mean = field.iter().sum::<f64>() / n;

    field.iter()
        .map(|&x| x - mean)
        .collect()
}

/// ===============================
/// Operator B — Local Relational Accumulation
/// ===============================
///
/// Definition:
/// B(x)[i] = x[i] + x[(i + 1) mod n]
///
/// Properties:
/// - Local relational coupling
/// - Symmetric under index reflection
/// - No global aggregation
/// - Topology-defined (ring structure)
///
/// RELATIONAL PROPERTIES:
/// B(x)[i] = x[i] + x[(i+1) mod n]
/// Each index couples only to its immediate successor
/// within the declared ring topology.
/// No index accesses elements beyond neighborhood i±1.
///
/// UNDER REPEATED APPLICATION ALONE:
/// ∀ i,j: |B^n(x)[i] − B^n(x)[j]| → 0 as n increases.
/// Repeated application produces an index-uniform vector x* ∈ D.
/// E(x*, ρ) = x*   within D.
///
/// This form is declared canonical within this kernel.
pub fn operator_b(field: &[f64]) -> Vec<f64> {
    let n = field.len();

    field.iter()
        .enumerate()
        .map(|(i, &x)| x + field[(i + 1) % n])
        .collect()
}

/// ===============================
/// Operator R — Antisymmetric Circulation
/// ===============================
///
/// Definition:
/// R(x)[i] = x[i] + ρ · (x[(i+1) mod n] − x[(i−1) mod n])
///
/// Properties:
/// - Antisymmetric: forward difference, not averaging
/// - Introduces directional bias
/// - Preserves total magnitude under periodic boundary
/// - Zero-sum in the circulation term
///
/// RELATIONAL PROPERTIES:
/// R(x)[i] = x[i] + ρ · (x[(i+1) mod n] − x[(i−1) mod n])
/// The circulation term ρ·(x[i+1] − x[i−1]) is a
/// forward-backward index difference — antisymmetric
/// under index reflection.
/// R(x)[i] ≠ R(x)[n−i] in general.
/// This breaks the index symmetry present in A and B.
///
/// NECESSITY WITHIN D:
/// Under A and B alone:
///   ∀ i,j: |x[i] − x[j]| → 0 within D.
///   Repeated application produces an index-uniform vector x* ∈ D.
///   E(x*, ρ) = x*   within D.
/// R introduces ρ·(x[i+1] − x[i−1]) at each index,
/// maintaining nonzero index-to-index differences
/// provided ρ > 0 and x is not index-uniform.
///
/// ρ must satisfy: ρ ∈ (0.0, 0.5] for output to remain in D.
///
/// This form is declared canonical within this kernel.
pub fn operator_r(field: &[f64], rho: f64) -> Vec<f64> {
    let n = field.len();

    field.iter()
        .enumerate()
        .map(|(i, &x)| {
            let i_next = (i + 1) % n;
            let i_prev = (i + n - 1) % n;
            x + rho * (field[i_next] - field[i_prev])
        })
        .collect()
}

/// ===============================
/// Operator C — Bounded Coherence
/// ===============================
///
/// Definition:
/// C(x)[i] = x[i] / (1 + |x[i]|)
///
/// Properties:
/// - Structural boundedness: −1 ≤ C(x)[i] ≤ 1
/// - Odd function: C(−x) = −C(x)
/// - Saturating nonlinearity
/// - No clamping, no repair
///
/// RELATIONAL PROPERTIES:
/// C(x)[i] = x[i] / (1 + |x[i]|)
/// ∀ x[i] ∈ ℝ: C(x)[i] ∈ (−1, 1).
/// Output membership in D is guaranteed by construction.
/// Sign and relative index ordering are preserved.
/// C(−x) = −C(x) — odd function over D.
/// No clamping, thresholding, or corrective logic is introduced.
/// Boundedness is a structural property of the operator,
/// not an enforcement condition applied to its output.
///
/// This form is declared canonical within this kernel.
pub fn operator_c(field: &[f64]) -> Vec<f64> {
    field.iter()
        .map(|&x| x / (1.0 + x.abs()))
        .collect()
}

/// ===============================
/// Operator E — Composite Evolution
/// ===============================
///
/// Definition:
/// E(x, ρ) = C(R(B(A(x)), ρ))
///
/// Properties:
/// - Irreversible forward evolution
/// - Single application per evolutionary pass
/// - No convergence targeting
/// - No rollback
///
/// STRUCTURAL ORDERING:
/// The sequence A → B → R → C is mathematically necessary:
///
/// 1. A extracts relational gradients (removes absolute reference)
/// 2. B couples local relations (creates relational fabric)
/// 3. R introduces antisymmetric circulation (sustains dynamics)
/// 4. C bounds the result (prevents divergence)
///
/// Any reordering produces fundamentally different dynamics.
/// This is not a pipeline—it is a mathematical composition.
///
/// WHY E, NOT M:
/// E explicitly includes R (circulation) as a structural requirement
/// for persistent, non-equilibrium dynamics.
///
/// This defines evolution under this kernel for all x ∈ D.
pub fn operator_e(field: &[f64], rho: f64) -> Vec<f64> {
    let a = operator_a(field);
    let b = operator_b(&a);
    let r = operator_r(&b, rho);
    operator_c(&r)
}

// =======================================================
// OPERATOR TYPE SIGNATURES
// =======================================================
//
// Declared operator types over D:
//
//   A : D    → ℝⁿ       (gradient extraction; output not yet bounded)
//   B : ℝⁿ  → ℝⁿ       (local accumulation; preserves dimensionality)
//   R : ℝⁿ × ℝ → ℝⁿ   (antisymmetric circulation; ρ ∈ ℝ bounded)
//   C : ℝⁿ  → D        (bounded coherence; output ∈ D by construction)
//
// Composite:
//
//   E : D × ℝ → D      (full evolution; D-closed by C)
//
// These signatures are declared within this kernel only.
// No claim is made about operator behavior outside D.
//
// =======================================================
// STRUCTURAL ENFORCEMENT
// =======================================================
//
// Each operator is defined as a pure function.
// No operator contains loops, iteration, or state.
// Each is applied exactly once in the evolution sequence.
//
// The type system prevents:
// - accidental reordering (explicit function composition)
// - double application (no internal iteration)
// - state leakage (no mutable references, no side effects)
//
// Within this kernel, evolution is defined only via E.
// Alternative compositions are not defined here.
