# Simulation of the Gross-Pitaeveskii equation

This program solves the Gross-Pitaeveskii equation in 1 dimension for a uniform
grid. The arguments are:

- `scattering_length: f64` scattering length in harmonic oscilator units (`a0` of old fortran program)
- `grid_size: u32` number of integration steps in r-grid (`n1` of old fortran program)
- `r_step: f64` integration step in r-space (`step` of old fortran program)
- `number_atoms: u32` number of atoms (`aa` of old fortran program)
- `time_step: f64` the step in time (`time` of old fortran program)
- `alpha: f64` the starting parameter for the harmonic oscilator function
- `time_iterations: u32` number of iterations for the imaginary time evolution (`iter` of old fortran program)

The solver also suports the non-linear interactionless aproximation and the 
thomas-fermi aproximation. To use this it should be provided the flags 
`--use-harmonic-oscilator` or `--thomas-fermi-aproximation`.

