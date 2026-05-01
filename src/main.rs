use clap::{Parser};
use gross_pitaeveskii::{chemical_potential, total_energy, imaginargy_time_evolution, initial_wave_function_builder, laplacian, radial_grid_builder};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    use_harmonic_oscilator: bool,
    #[arg(long)]
    thomas_fermi_aproximation: bool,
    /// scattering length in harmonic oscilator units (a0 of old fortran program)
    scattering_length: f64,
    /// number of integration steps in r-grid (n1 of old fortran program)
    grid_size: u32,
    /// integration step in r-space (step of old fortran program)
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

fn main() -> std::io::Result<()> {

    let cli = Cli::parse();

    println!("{:?}", cli);

    let use_harmonic_oscilator: bool = cli.use_harmonic_oscilator;

    let thomas_fermi_aproximation = cli.thomas_fermi_aproximation;

    let scattering_length = cli.scattering_length;

    let grid_size = cli.grid_size as usize;

    let r_step = cli.r_step;

    let number_atoms = cli.number_atoms;

    let time_step = cli.time_step;

    let alpha = cli.alpha;

    let time_iterations = cli.time_iterations as usize;

    let radial_grid = radial_grid_builder(r_step, grid_size);
    
    let initial_wave_function = initial_wave_function_builder(alpha, &radial_grid);

    let ground_state = imaginargy_time_evolution(use_harmonic_oscilator, thomas_fermi_aproximation, scattering_length, 
        number_atoms, time_iterations, time_step, &initial_wave_function, &radial_grid);

    let laplacial_ground_state: Vec<f64> = if thomas_fermi_aproximation {vec![0.; grid_size]} else {laplacian(&ground_state, r_step)};

    let non_linear_strength =  if use_harmonic_oscilator {0.} else {scattering_length*(number_atoms as f64)};

    let chemical_potential_r = chemical_potential(&ground_state, &laplacial_ground_state,
        &radial_grid, non_linear_strength);

    let mut file_chemical_potential = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("data/chemical_potential_r.txt")?;


    writeln!(file_chemical_potential, "{:>15} {:>20}", "Radius", "Chemical potential")?;
    for i in 1..grid_size {
        writeln!(file_chemical_potential, "{:>15.10} {:>20.10}", radial_grid[i], chemical_potential_r[i])?;
    }

    let energy: f64 = total_energy(&ground_state, &laplacial_ground_state, &radial_grid, non_linear_strength);

    let average_radius: f64;
    let mut kinetic_energy = 0.;
    let mut potencial_harmonic_oscilator = 0.;
    let mut internal_potential = 0.;
    let mut average_chemical_potential = 0.;
    let mut density = vec![0.;grid_size];
    let mut norm_density = 0.;
    
    for i in 1..grid_size {
        kinetic_energy = kinetic_energy + ground_state[i]*laplacial_ground_state[i];
        potencial_harmonic_oscilator = potencial_harmonic_oscilator + radial_grid[i].powi(2)*ground_state[i].powi(2);
        internal_potential = internal_potential + radial_grid[i].powi(2)*(ground_state[i]/radial_grid[i]).powi(4);
        average_chemical_potential = average_chemical_potential + chemical_potential_r[i]*ground_state[i].powi(2);
        density[i] = (ground_state[i]/radial_grid[i]).powi(2);
        norm_density = norm_density + radial_grid[i].powi(2)*density[i];
    }

    average_radius = (potencial_harmonic_oscilator*r_step).sqrt();
    let average_radius_squared = potencial_harmonic_oscilator*r_step;
    average_chemical_potential = average_chemical_potential*r_step;
    kinetic_energy = -kinetic_energy*r_step*0.5;
    potencial_harmonic_oscilator = 0.5*potencial_harmonic_oscilator*r_step;
    internal_potential = internal_potential*r_step*non_linear_strength*0.5;
    let total_potential = potencial_harmonic_oscilator + internal_potential;
    norm_density = norm_density*r_step;

    let values = [
        ("energy", energy),
        ("average_chemical_potential", average_chemical_potential),
        ("kinetic_energy", kinetic_energy),
        ("total_potential", total_potential),
        ("potencial_harmonic_oscilator", potencial_harmonic_oscilator),
        ("internal_potential", internal_potential),
        ("average_radius", average_radius),
        ("average_radius_squared", average_radius_squared),
        ("norm_density", norm_density),
    ];

    let mut file_values = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("data/values.txt")?;

    for (name, value) in values {
        writeln!(file_values, "{:<30} {:>15.10}", name, value)?;
    }

    let mut file_density = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("data/density_r.txt")?;

    writeln!(file_density, "{:>15} {:>15}", "Radius", "Density")?;
    for i in 1..grid_size {
        writeln!(file_density, "{:>15.10} {:>15.10}", radial_grid[i], density[i])?;
    }
    
    Ok(())

}
