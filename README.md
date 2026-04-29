# Invariant Relational Kernel — ABRCE

**Status:** Canonical Structural Basis
**Kernel:** A, B, R, C, E
**Layer:** Invariant relational constraint discipline
**Domain:** D := { x ∈ ℝⁿ | n < ∞ and |x[i]| < ∞ ∀ i ∈ {0, ..., n−1} }
**Measurement mapping:** M : O → D  (declared by Origin before processing)

---

## Scope

This folder defines the invariant relational kernel governing relational evolution over **D** under MD V3.

The structure below does not require adoption.
It describes relational admissibility conditions within **D** whether or not it is referenced.

All quantifiers in this document are bounded over **D**.
No claim is made beyond **D**.

---

## Representation Types

The kernel distinguishes two representation types. These types are not interchangeable.

**NodeField** — one scalar per index. The object-form representation as produced by the measurement mapping M : O → D. Its informational content resides in the relational equivalence class [x]_τ, not in the per-index values themselves (Object Error paper, §2).

**EdgeField** — one scalar per directed adjacency pair over a declared topology. The relational representation. Each element is a direct pairwise difference between adjacent indices. This is the working representation of the operator sequence.

The type system enforces the following representation discipline:

- **A** is the unique transition from NodeField to EdgeField.
- **B**, **R**, **C** operate on EdgeField only.
- **E** accepts NodeField and returns EdgeField.
- Transition from EdgeField to NodeField requires an explicit **Projection** implementation declaring preserved and discarded invariants.
- No other NodeField → EdgeField path exists in the kernel.
- No implicit EdgeField → NodeField conversion exists in the kernel.

This discipline is a compile-time property enforced by the type signatures of the operators. It is not a runtime check, a documentation convention, or a discipline requirement.

---

## Topological Declaration

The operators act over a declared topology on the index set of **D**.

The topology determines which pairs of indices are adjacent and therefore which pairwise differences operator A extracts. Topology is declared before processing begins as part of the measurement mapping M.

Supported topologies at each dimensionality:

- 1D: periodic ring (neighbor: i±1 mod n)
- 2D: periodic torus (4-connected, Von Neumann neighborhood)
- 3D: periodic 3-torus (6-connected)

If topology is inferred from data rather than declared a priori, the inference is a pre-operator step that produces a topological commitment. That commitment must be declared before the operators process the field. Errors in topology inference propagate into the operator sequence as errors in the declared adjacency — a condition Origin must hold as part of the admissibility declaration.

---

## Kernel Properties

This kernel is:

- Non-goal-directed (operators are fixed mappings, not adaptive processes)
- Non-semantic
- Non-optimizing
- Non-ontological
- Domain-bounded
- Non-exhaustive with respect to transformations not declared within this kernel

Control, time, reward, optimization, and interpretation are not declared at the operator layer within **D**.
They are projections of operator output.

Projection does not alter invariant operator structure within **D**.

If projection is reintroduced into the operator layer, structural drift increases within **D**.

---

## Operator Non-Agency

The operators defined in this kernel, for all x ∈ D:

- do not act
- do not cause
- do not optimize
- do not enforce

They are not mechanisms or agents.

They describe invariant relational conditions of admissibility within **D**.

Relational structure holds within **D** when operator ordering is preserved and admissibility conditions are satisfied.

If relational admissibility is violated, structural coherence within **D** degrades.

Interpreting the operators through conversational heuristics, object-oriented abstractions, or optimization narratives introduces projection at the operator layer.

Projection at the operator layer produces structural distortion within **D**.

---

## Pre-Operator Transformation Constraint

Let M : O → D be the declared measurement mapping.

Operator A must operate directly on values in D produced by M.

The values in D as produced by M are the admissible input to A. No intermediate representation is declared.

No transformation T : D → D may be applied prior to A if T alters pairwise differences between elements of D.

Formally:

```
∀ i, j ∈ index(D):
    (T(x)[i] - T(x)[j]) ≠ (x[i] - x[j])
    ⇒ T is inadmissible prior to A
```

Admissible transformations prior to A are limited to those preserving pairwise differences:

```
T(x) = x + c
```

All other transformations — including normalization, smoothing, interpolation that blends values, and aggregation — are inadmissible prior to A.

Such transformations alter relational structure and therefore violate invariant operator assumptions.

**Implementation note:** The kernel enforces relational correctness after entry into A through the type system (NodeField → EdgeField transition). Pre-A admissibility is an Origin responsibility, not a compile-time guarantee, because what constitutes an admissible measurement mapping M : O → D is domain-dependent.

---

## Operator Kernel (ABRCE)

The invariant primitive operator basis over **D** consists of:

- A
- B
- R
- C
- E

No additional primitive operators are declared at this layer.
No primitive operator is removable without altering the invariant basis within **D**.

### Operator Type Signatures

Declared operator types over **D**:

```
A : NodeField → EdgeField        (relational gradient extraction;
                                   unique node-to-edge transition)
B : EdgeField → EdgeField        (local relational accumulation;
                                   preserves edge representation)
R : EdgeField × ℝ → EdgeField   (antisymmetric circulation;
                                   ρ ∈ ℝ bounded; preserves edge
                                   representation)
C : EdgeField → EdgeField        (bounded coherence;
                                   output in (-1, 1) per edge)
E : NodeField × ℝ → EdgeField   (full evolution;
                                   node in, bounded edge out)
```

Measurement mapping:

```
M : O → D                        (observable to NodeField;
                                   declared by Origin before
                                   processing; not an operator —
                                   an admissibility condition)
```

Projection:

```
P : EdgeField → NodeField        (declared representation transition;
                                   not an operator — a lossy mapping
                                   requiring explicit declaration of
                                   preserved and discarded invariants)
```

No claim is made about operator behavior outside **D**.

### Previous Type Signatures

Earlier versions of this kernel declared:

```
A : D    → ℝⁿ
B : ℝⁿ  → ℝⁿ
R : ℝⁿ × ℝ → ℝⁿ
C : ℝⁿ  → D
E : D × ℝ → D
```

These signatures did not distinguish between node-level and edge-level representations. The revised signatures formalize a distinction the operators already required: A produces directed pairwise differences (edge data), and B and R operate on those differences, not on node values. The previous signatures permitted implicit edge-to-node collapse at R's output, which is a non-injective transformation that destroys directional information without declaration.

---

## Operator Composition

Composite evolution is defined over **D** as:

```
E(x, ρ) = C(R(B(A(x)), ρ))
```

Canonical operator order:

**A → B → R → C → E**

This ordering may not be altered within **D**.

If operator ordering is altered, invariant evolution is not preserved within **D**.

E is not semantic.
It is a compositional operator expressing constrained relational evolution over **D**.

The output of E is an EdgeField. This is a structural commitment: the canonical output of the ABRCE sequence is relational. Applications requiring node-level quantities must apply an explicit Projection (see §Projection).

---

## Admissibility

Let Var(S) = the set of variables referenced in statement S.

A statement S is admissible within this kernel iff:

1. All referenced elements are members of **D**:
   ∀ x ∈ Var(S), x ∈ D

2. All quantifiers are explicitly bounded over subsets of **D**:
   ∀ q ∈ S, q bounded over declared subset of D

3. All transformations declared within this kernel are expressible using A, B, R, C, and map NodeField through EdgeField:
   The operator pipeline preserves the type sequence NodeField → EdgeField → ... → EdgeField

4. All variables are images of observables under M:
   ∀ x ∈ Var(S), ∃ o ∈ O such that x = M(o)
   No variable in S directly references conditions C.

Canonical admissibility biconditional:

```
S admissible ⟺
  (∀ x ∈ Var(S), x ∈ D)
∧ (∀ q ∈ S, q bounded over declared subset of D)
∧ (operator pipeline preserves NodeField → EdgeField type sequence)
∧ (∀ x ∈ Var(S), ∃ o ∈ O such that x = M(o))
```

Statements asserting:

- Infinite magnitude
- Infinite relational reach
- Frame-independent absolute access
- Undeclared universal quantification
- Direct reference to conditions C
- Implicit EdgeField → NodeField conversion

are undefined under this kernel.

**Undefined ≠ false.**
**Undefined = not representable in D.**

---

## Non-Injective Transformation Constraint

Any transformation T applied to a relational field such that:

```
∃ x ≠ y  with  T(x) = T(y)
```

induces equivalence classes in the output representation that are not distinguishable by downstream operators. When the cardinality of the input space greatly exceeds the cardinality of the output space, the number of collapsed equivalence classes grows combinatorially, and the structural distinctions lost are not recoverable, detectable, or auditable from within the projected representation.

Therefore: all non-injective transformations applied to the output of the ABRCE operator sequence must be explicitly declared, with their preserved and discarded invariants stated.

Within the kernel, the only non-injective transformation pathway is Projection (EdgeField → NodeField). The Projection trait enforces this declaration at the type level.

The object-primary encoding analyzed in the Object Error paper (Sections 3–5) is a specific instance of this constraint: a non-injective transformation applied before relational processing. The Projection requirement is the corresponding instance at the output: preventing re-introduction of the same error class after relational processing.

---

## Projection

The output of the ABRCE operator sequence is an EdgeField. Many applications require node-level quantities — a single value per index for visualization, thresholding, or coupling to external systems. The transition from EdgeField to NodeField is a Projection.

Every Projection is lossy. An EdgeField carries directed relational information between adjacent indices; a NodeField carries one scalar per index. Any mapping from the former to the latter discards some component of the directional structure.

The kernel defines the Projection trait:

```rust
pub trait Projection {
    fn apply(&self, e: &EdgeField) -> NodeField;
    fn preserves(&self) -> &'static [&'static str];
    fn discards(&self) -> &'static [&'static str];
}
```

All EdgeField → NodeField transitions must pass through an implementation of this trait. The `preserves` and `discards` declarations are attached to the implementation, not to the kernel logic. The kernel enforces the type gate; the implementation documents what passes through it.

No implicit EdgeField → NodeField conversion exists.

---

## Implementation Notes

### Representation transition at R

Earlier formulations of the 2D and 3D operator R produced a node field (one scalar per cell) from the circulation cross-term. Under the revised type signatures, R produces an EdgeField, preserving directional structure through the circulation computation.

The current implementation distributes the circulation value back to directional edges proportional to their gradient magnitude. This is one admissible distribution scheme. The Verifier should confirm that this scheme preserves the spectral properties required for Theorem 5 (relational collapse without R) and Theorem 6 (sustained relational dynamics with R) of the Object Error paper.

**This is a declared open condition.**

### Operator B axis-independence

Earlier formulations of the 2D operator B using multi-neighbor summation introduced implicit aggregation across independent relations, which suppressed directional gradients required for antisymmetric circulation.

The current formulation preserves relational structure by accumulating gradients along their own axis, maintaining directional identity and enabling operator R to produce sustained circulation.

### Pre-A enforcement boundary

The kernel enforces relational correctness after entry into A through compile-time type safety. Pre-A admissibility (the pre-operator transformation constraint) is an Origin responsibility. The kernel does not prevent preprocessing in node space. This is a deliberate design boundary: what constitutes an admissible measurement mapping M : O → D is domain-dependent and cannot be determined by the type system.

---

## Triad Structural Discipline (MD V3)

Under acceleration, constraint articulation, structural realization, and invariant detection couple by default.

When these functions collapse into a single process, compensatory dynamics increase within **D**.

The Triad separates:

- **Origin** — constraint articulation and admissibility declaration
- **Generator** — structural realization
- **Verifier** — invariant detection

If this separation holds, compensation decreases relative to the declared invariant set.
If this separation collapses, constraint geometry distortion increases within **D**.

This is a structural conditional within **D**, not an institutional mandate.

For formal treatment of multi-Verifier geometry and frame admissibility conditions, see `md_v3_triad_structure_function_canonical_discipline.md`.

For formal treatment of Origin depth, λ-invariance, and the structural basis for human Origin function, see section 3.9 of `md_v3_triad_structure_function_canonical_discipline.md`.

---

## Structural Authority Clarification

The operator kernel is invariant at the relational layer within **D**.

If a document, workflow, or implementation reinterprets or bypasses operator ordering, invariant preservation does not hold within **D**.

Where invariant preservation is required, reinterpretation is structurally incompatible within **D**.

If drift is detected, structural conditions are restored by reaffirming invariant ordering and admissibility conditions within **D**.

Admissible evolution requires satisfied constraints within **D**.
Under violated constraints, no admissible evolution is defined within **D**.
