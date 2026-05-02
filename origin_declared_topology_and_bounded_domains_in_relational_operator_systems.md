# Origin-Declared Topology and the Necessity of Bounded Domains in Relational Operator Systems

## Abstract

This paper presents a formal argument that relational operator systems require explicitly declared, bounded topological domains in order to preserve structural validity. We show that applying relational operators to irregular observational graphs results in heuristic approximations rather than well-defined transformations. Using the ABRCE operator system as a reference implementation, we demonstrate that correct modeling of complex natural systems requires a separation between measurement mapping (M : O → D) and operator execution, with topology declared by Origin prior to computation.

---

## 1. Introduction

Modern modeling of complex natural systems—such as magnetospheric dynamics, weather systems, and biological networks—relies heavily on irregular observational data. These observations are typically distributed across non-uniform spatial configurations, such as sensor networks or station arrays.

Standard approaches apply statistical or interpolative methods directly to these irregular structures. However, these approaches lack guarantees of structural preservation and often collapse directional or relational information prior to analysis.

This paper addresses a foundational question:

> Under what conditions are relational operators well-defined and structurally valid?

---

## 2. The Object Error

Traditional modeling frameworks treat node values as primary and derive relationships secondarily. This introduces a structural limitation:

- relational information is either approximated or lost
- directional structure cannot be preserved
- antisymmetric components are collapsed

We refer to this as the *Object Error*: the assumption that node-level representations are sufficient to capture system dynamics.

Relational operator systems invert this assumption:

> Relations are primary; nodes are derived.

This work builds directly on the formal argument presented in **object_error.md**, which establishes that index-local (node-based) representations cannot preserve relational structure under general transformation. That document defines the representation constraint for admissible systems.

The present paper does not modify or extend the Object Error itself. Instead, it introduces an additional requirement:

> Even when relations are explicitly represented, relational operators remain valid only when applied to domains with declared topology.

Thus, resolution of the Object Error requires both:

1. Explicit relational representation (EdgeField)
2. Origin-declared, bounded topology for operator application

---

## 3. Kernel Requirements

We consider a relational operator system defined over a domain D with the following operators:

- **A**: NodeField → EdgeField
- **B, R, C**: EdgeField → EdgeField

The operator sequence is:

```
E(x, ρ) = C(R(B(A(x)), ρ))
```

### 3.1 Topological Constraints

For the operators to be well-defined, the domain must provide:

- unique continuation (forward and backward neighbors)
- closed structure (no boundary ambiguity)
- bounded indexing (finite, cyclic domain)

Admissible topologies include:

- 1D ring
- 2D torus
- 3D torus

These topologies guarantee that relational traversal is deterministic and well-defined under the declared topology.

---

## 4. Failure on Irregular Graphs

Real-world observational data is typically defined on irregular graphs:

- non-uniform spacing
- varying connectivity
- no global symmetry

Applying relational operators directly to such graphs introduces ambiguity:

- multiple possible continuations
- undefined forward/backward structure
- lack of closure

As a result, operators such as B and R must rely on heuristics (e.g., cosine similarity) to approximate continuation.

These approximations are not part of the kernel definition and break structural guarantees.

---

## 5. Origin Declaration

### 5.1 Definition

We define **Origin** as the entity responsible for declaring the domain D and its topology prior to operator execution.

Formally:

```
M : O → D
```

Where:

- O = observables (raw measurements)
- D = structured domain with declared topology

### 5.2 Role of M

The measurement mapping M is responsible for:

- embedding irregular observations into D
- defining coordinate systems and units
- performing interpolation where necessary

Critically:

> All interpolation and transformation occurs in M, not within the operator pipeline.

---

## 6. Separation of Concerns

We distinguish three layers:

1. **Observational Layer (O)** — raw measurements
2. **Mapping Layer (M)** — embedding into structured domain
3. **Operator Layer (ABRCE)** — relational transformations

This separation ensures that operators act only on admissible structures.

---

## 7. Application: Magnetospheric Modeling

### 7.1 Invalid Approach

- apply operators directly to station network
- use heuristic continuation

Result:

- loss of structural guarantees
- ambiguous circulation detection

### 7.2 Valid Approach

1. Map station data to a regular lat/lon grid (2D torus)
2. Declare topology
3. Apply ABRCE operators

This preserves relational structure and enables correct interpretation of circulation phenomena.

---

## 8. Units and Domain Definition

### 8.1 Problem

Data magnitude affects operator sensitivity.

### 8.2 Incorrect Solutions

- normalization
- statistical scaling

These introduce global coupling and violate admissibility.

### 8.3 Correct Solution

Units are part of domain declaration:

```
M : O → D_units
```

Example:

- D_Tesla vs. D_nT

This changes the definition of D, not the values within it.

---

## 9. General Principle

> Operators require structure. Structure must be declared, not inferred.

Irregular data must be embedded into a structured domain before relational computation.

---

## 10. Implications

This framework applies within domains where observables can be mapped into bounded topologies satisfying the stated conditions.

Examples include (non-exhaustive):

- physics
- climate modeling
- biological systems
- machine learning architectures

Without domain declaration:

- models become heuristic
- results lose interpretability

With domain declaration:

- operators are well-defined
- structure is preserved
- results are structurally interpretable within the declared domain

---

## 11. Conclusion

The validity of relational modeling does not depend on the data itself, but on the structure into which the data is embedded.

Declaring bounded domains and topology at the level of Origin is not optional—it is the prerequisite for any operator system claiming structural correctness.

The Object Error establishes that relational structure must be explicitly represented. This work completes that requirement by showing that representation alone is insufficient: relational operators require a domain with declared topology to preserve structural meaning.

Together, these results define the minimal conditions for admissible relational computation:

- relational representation (object_error.md)
- origin-declared topology (this work)
- operator consistency (ABRCE kernel)

Without all three, relational structure cannot be preserved under transformation.

---

## References

- object_error.md (Relational representation constraint)
- ABRCE kernel (operators.rs and associated documentation)
