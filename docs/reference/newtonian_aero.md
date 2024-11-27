# Introduction to Newtonian Aerodynamics

This is a summary of the paper
[Newtonian Aerodynamics for General Body Shapes](https://ntrs.nasa.gov/citations/19660012440)
and its implementation in our simulation. It is generated using ChatGPT
`o1-mini`.

## 1. Introduction to Newtonian Aerodynamics

Newtonian Aerodynamics applies Newtonian mechanics to predict aerodynamic forces
such as drag and lift on objects moving through a fluid (like air). This
approach is particularly useful for objects in supersonic or hypersonic flows.

## 2. Fundamental Concepts

- *Drag Force (Fₓ)* is the resistance force acting opposite to the direction of
  an object's velocity.

- *Lift Force (Fᵧ)* is the force perpendicular to the direction of motion, often
  associated with winged objects.

- *Flow Regimes*

  - **Subsonic**: Flow speeds less than the speed of sound.
  - **Supersonic**: Flow speeds greater than the speed of sound.
  - **Hypersonic**: Flow speeds significantly higher than supersonic speeds.

## 3. Newtonian Impact Theory

The Newtonian Impact Theory estimates aerodynamic forces based on the assumption
that fluid particles impacting the body either stick or rebound elastically.

### Key Assumptions

1. The fluid is treated as a collection of particles.
2. Particles impacting the body either stick or reflect off elastically.
3. The flow is steady and uniform far from the body.
4. Viscous effects are neglected.

### Mathematical Formulation

#### a. Pressure Coefficient (Cₚ)

\[ C_p = \frac{P - P_{\infty}}{\frac{1}{2} \rho V_{\infty}^2} \] Where:

- \( P \) = Local pressure on the body.
- \( P_{\infty} \) = Free-stream pressure.
- \( \rho \) = Fluid density.
- \( V_{\infty} \) = Free-stream velocity.

#### b. Newtonian Drag Coefficient (C_d)

\[ C_d = \int \left(1 - \cos(\theta)\right) dA \] Where:

- \( \theta \) = Angle between the local surface normal and the free-stream
  velocity vector.
- \( dA \) = Differential area element.

## 4. Calculating Aerodynamic Forces

### a. Drag Force (Fₓ)

\[ F_x = \frac{1}{2} \rho V_{\infty}^2 C_d A \] Where:

- \( A \) = Reference area (e.g., frontal area).

### b. Lift Force (Fᵧ)

\[ F_y = \frac{1}{2} \rho V_{\infty}^2 C_l A \] Where:

- \( C_l \) = Lift coefficient.

## 5. Application to General Body Shapes

The paper extends Newtonian Impact Theory to arbitrary body shapes by
decomposing the body's surface into differential elements, calculating local
impact angles, and integrating contributions to overall aerodynamic forces.

### a. Surface Sampling

To handle complex geometries, the body's surface is sampled at numerous points.
Each point's normal vector and its angle relative to the flow direction are
determined.

### b. Numerical Integration

Due to the complexity of analytical solutions for irregular shapes, numerical
techniques are employed to perform surface integrals for drag and lift.

## 6. Enhancements and Corrections

While Newtonian Impact Theory provides foundational estimates, the paper
discusses enhancements for improved accuracy, including:

- Correction factors based on empirical data.
- Considering how the body's orientation relative to the flow affects the
  distribution of impact angles.

## 7. Practical Implementation Considerations

### a. Computational Efficiency

Sampling the surface at too many points can be computationally intensive.
Optimal sampling densities should balance accuracy and performance.

### b. Integration with Physics Engines

Aerodynamic forces must be integrated with the physics engine's update loops,
ensuring forces are applied correctly based on the body's current state and
orientation.

### c. Validation

Comparing simulation results with experimental data to validate and calibrate
the implemented models.

## 8. Summary of Mathematical Steps for Implementation

1. **Surface Sampling**: Discretize the body's surface into numerous points.
2. **Local Impact Angle Calculation**: Compute the angle between each normal
   vector and the free-stream velocity vector.
3. **Pressure Coefficient Determination**: Use the local impact angles to
   estimate the pressure coefficient \( C_p \).
4. **Drag and Lift Force Integration**: Numerically integrate the contributions
   of all sampled points to compute total drag and lift forces.
5. **Normalization and Scaling**: Adjust the computed forces based on fluid
   density and relative velocity squared.
