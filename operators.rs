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
// INVARIANT RELATIONAL KERNEL — ABCRE
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
/// WHY SYMMETRIC:
/// A extracts relational differences from absolute values.
/// It is symmetric because A(x) = −A(−x) would hold if we negated around
/// a different reference, but more fundamentally: A treats all positions
/// equivalently with respect to the global mean.
///
/// WHY THIS LEADS TO CONVERGENCE (when used alone):
/// Repeated application of symmetric operators dissipates gradients.
/// Without antisymmetric circulation, information flows toward equilibrium.
/// Equilibrium is NOT an invariant—it is a degenerate attractor that
/// destroys relational structure.
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
/// WHY SYMMETRIC:
/// B couples each element only to its immediate neighbor.
/// The operation is symmetric because information flows equally
/// in the defined topological structure—there is no preferred direction.
///
/// WHY THIS LEADS TO CONVERGENCE (when used alone):
/// Symmetric accumulation smooths gradients. Without directional bias,
/// repeated application drives the field toward uniform distribution.
/// This is structural dissipation, not thermodynamic loss.
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
/// WHY ANTISYMMETRIC:
/// R computes a *difference* between forward and backward neighbors,
/// creating a directional gradient. This breaks the symmetry that
/// causes convergence.
///
/// R(x) ≠ R(−x) in general structure.
/// R introduces rotational or circulatory dynamics.
///
/// WHY R IS NECESSARY FOR PERSISTENCE:
/// Without antisymmetric circulation, symmetric operators (A, B)
/// drive all fields toward equilibrium—a state of zero relational
/// distinction.
///
/// R enables persistent, non-equilibrium dynamics by sustaining
/// gradients through circulation rather than dissipation.
///
/// Persistent structures are not stable objects.
/// They are dynamically sustained circulation patterns.
///
/// WHY EQUILIBRIUM IS NOT AN INVARIANT:
/// Equilibrium is a *degenerate attractor* where relational structure
/// collapses. It is not preserved under evolution—it is the absence
/// of evolution.
///
/// True invariants persist *through* transformation.
/// Equilibrium is the cessation of transformation.
///
/// The parameter ρ (rho) controls circulation strength.
/// ρ must be bounded: typical range [0.0, 0.5] for stability.
///
/// This form is declared canonical within this kernel.
pub fn operator_r(field: &[f64], rho: f64) -> Vec<f64> {
    let n = field.len();

    field.iter()
        .enumerate()
        .map(|(i, &x)| {
            let i_next = (i + 1) % n;
            let i_prev = (i + n - 1) % n;  // handles i=0 case correctly
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
/// WHY THIS FORM:
/// Boundedness arises from mathematical structure, not enforcement.
/// The operator naturally compresses unbounded inputs into a finite range
/// while preserving sign and relative ordering.
///
/// This prevents runaway growth without introducing thresholds,
/// clipping, or corrective logic.
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
