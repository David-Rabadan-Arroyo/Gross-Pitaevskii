# Simulation of the Gross-Pitaevskii equation

This program solves the radial component of the Gross-Pitaevskii equation for a uniform grid. The arguments are:

- `scattering_length: f64` scattering length in harmonic oscilator units (`a0` of old fortran program)
- `grid_size: u32` number of integration steps in r-grid (`n1` of old fortran program)
- `r_step: f64` integration step in r-space (`step` of old fortran program)
- `number_atoms: u32` number of atoms (`aa` of old fortran program)
- `time_step: f64` the step in time (`time` of old fortran program)
- `alpha: f64` the starting parameter for the harmonic oscilator function
- `time_iterations: u32` number of iterations for the imaginary time evolution (`iter` of old fortran program)

The solver also suports the non-linear interactionless aproximation and the thomas-fermi aproximation. 
To use this it should be provided the flags `--use-harmonic-oscilator` or `--thomas-fermi-aproximation`.

The program returns 3 files called **chemical_potential_r.txt**, **density_r.txt** and **values.txt** with
the values of the chemical potential at each node of the grid, the radial density at each node of the grid and a
set of values that are used for analizing the solution.

## Example of use

1. Compile the program with `cargo build --release`, provided that you have instaled Rust and Cargo.
2. Make a directory called `data`.
3. Execute the program with `target/release/gross_pitaevskii 0.00433 700 0.015 1000000 0.00005 0.3 70000`
4. Three new files should be created at `data`.
