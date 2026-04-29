use clap::{Parser};
use gross_pitaeveskii::{chemical_potential, imaginargy_time_evolution, initial_wave_function_builder, laplacian, radial_grid_builder};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    use_harmonic_oscilator: bool,
    /// scattering length in harmonic oscilator units (a0 of old fortran program)
    scattering_length: f64,
    /// number of integration steps in r-grid (n1 of old fortran program)
    grid_size: u32,
    /// integration step in r-space (step of old fortran program))
    r_step: f64,
    /// number of atoms (aa of old fortran program)
    number_atoms: u32,
    /// the step in time (time of old fortran program)
    time_step: f64,
    /// the starting parameter for the harmonic oscilator function
    alpha: f64,
    /// number of iterations for the imaginary time evolution (iter of old fortran program)
    time_iterations: u32,
}

fn main() {

    let cli = Cli::parse();

    println!("{:?}", cli);

    let use_harmonic_oscilator: bool = cli.use_harmonic_oscilator;

    let scattering_length = cli.scattering_length;

    let grid_size = cli.grid_size as usize;

    let r_step = cli.r_step;

    let number_atoms = cli.number_atoms;

    let time_step = cli.time_step;

    let alpha = cli.alpha;

    let time_iterations = cli.time_iterations as usize;

    let radial_grid = radial_grid_builder(r_step, grid_size);
    
    let initial_wave_function = initial_wave_function_builder(alpha, &radial_grid);

    let ground_state = imaginargy_time_evolution(use_harmonic_oscilator, scattering_length, 
        number_atoms, time_iterations, time_step, &initial_wave_function, &radial_grid);

    let laplacial_ground_state = laplacian(&ground_state, r_step);

    let non_linear_strength =  if use_harmonic_oscilator {0.} else {scattering_length*(number_atoms as f64)};

    let chemical_potential_r = chemical_potential(&ground_state, &laplacial_ground_state,
        &radial_grid, non_linear_strength);

    
}
