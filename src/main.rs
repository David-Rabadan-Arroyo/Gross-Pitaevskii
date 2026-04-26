use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// scattering lenght in harmonic oscilator units (a0 of old fortran program)
    scattering_length: f64,
    /// number of integration steps in r-grid (n1 of old fortrn program)
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
    
}
