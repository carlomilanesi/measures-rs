// To run this, type:
// cargo run --example full

mod units;
use measures::{angle::Radian, dimensionless::One, traits::CrossProduct};
use units::*;

fn print_all_acceleration_units() {
    println!("* Acceleration units");
    println!(
        "  MetrePerSquareSecond: {}, {}, {}, {}, {}, {};",
        Measure::<MetrePerSquareSecond>::new(1.),
        MeasurePoint::<MetrePerSquareSecond>::new(1.),
        Measure2d::<MetrePerSquareSecond>::new([1., 2.]),
        MeasurePoint2d::<MetrePerSquareSecond>::new([1., 2.]),
        Measure3d::<MetrePerSquareSecond>::new([1., 2., 3.]),
        MeasurePoint3d::<MetrePerSquareSecond>::new([1., 2., 3.]),
    );
    println!(
        "  GForce: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {};",
        Measure::<GForce>::new(1.),
        Measure::<GForce>::new(1.).convert::<MetrePerSquareSecond>(),
        MeasurePoint::<GForce>::new(1.),
        MeasurePoint::<GForce>::new(1.).convert::<MetrePerSquareSecond>(),
        Measure2d::<GForce>::new([1., 2.]),
        Measure2d::<GForce>::new([1., 2.]).convert::<MetrePerSquareSecond>(),
        MeasurePoint2d::<GForce>::new([1., 2.]),
        MeasurePoint2d::<GForce>::new([1., 2.]).convert::<MetrePerSquareSecond>(),
        Measure3d::<GForce>::new([1., 2., 3.]),
        Measure3d::<GForce>::new([1., 2., 3.]).convert::<MetrePerSquareSecond>(),
        MeasurePoint3d::<GForce>::new([1., 2., 3.]),
        MeasurePoint3d::<GForce>::new([1., 2., 3.]).convert::<MetrePerSquareSecond>(),
    );
    println!(
        "  KilometrePerHourPerSecond: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {};",
        Measure::<KilometrePerHourPerSecond>::new(1.),
        Measure::<KilometrePerHourPerSecond>::new(1.).convert::<MetrePerSquareSecond>(),
        MeasurePoint::<KilometrePerHourPerSecond>::new(1.),
        MeasurePoint::<KilometrePerHourPerSecond>::new(1.).convert::<MetrePerSquareSecond>(),
        Measure2d::<KilometrePerHourPerSecond>::new([1., 2.]),
        Measure2d::<KilometrePerHourPerSecond>::new([1., 2.]).convert::<MetrePerSquareSecond>(),
        MeasurePoint2d::<KilometrePerHourPerSecond>::new([1., 2.]),
        MeasurePoint2d::<KilometrePerHourPerSecond>::new([1., 2.])
            .convert::<MetrePerSquareSecond>(),
        Measure3d::<KilometrePerHourPerSecond>::new([1., 2., 3.]),
        Measure3d::<KilometrePerHourPerSecond>::new([1., 2., 3.]).convert::<MetrePerSquareSecond>(),
        MeasurePoint3d::<KilometrePerHourPerSecond>::new([1., 2., 3.]),
        MeasurePoint3d::<KilometrePerHourPerSecond>::new([1., 2., 3.])
            .convert::<MetrePerSquareSecond>(),
    );
    println!();
}

fn print_all_action_units() {
    println!("* Action units");
    println!(
        "  JouleSecond: {}, {};",
        Measure::<JouleSecond>::new(1.),
        MeasurePoint::<JouleSecond>::new(1.),
    );
    println!();
}

fn print_all_amount_units() {
    println!("* Amount units");
    println!(
        "  Unit: {}, {};",
        Measure::<Unit>::new(1.),
        MeasurePoint::<Unit>::new(1.),
    );
    println!(
        "  Dozen: {}, {}, {}, {};",
        Measure::<Dozen>::new(1.),
        Measure::<Dozen>::new(1.).convert::<Unit>(),
        MeasurePoint::<Dozen>::new(1.),
        MeasurePoint::<Dozen>::new(1.).convert::<Unit>(),
    );
    println!(
        "  Mole: {}, {}, {}, {};",
        Measure::<Mole>::new(1.),
        Measure::<Mole>::new(1.).convert::<Unit>(),
        MeasurePoint::<Mole>::new(1.),
        MeasurePoint::<Mole>::new(1.).convert::<Unit>(),
    );
    println!();
}

fn print_all_angle_units() {
    println!("** Angle units **");
    println!(
        "  Radian: {}, {}, {}, {};",
        Measure::<Radian>::new(1.),
        MeasurePoint::<Radian>::new(1.),
        SignedDirection::<Radian>::new(1.),
        UnsignedDirection::<Radian>::new(1.),
    );
    println!(
        "  Cycle: {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Cycle>::new(1.),
        Measure::<Cycle>::new(1.).convert::<Radian>(),
        MeasurePoint::<Cycle>::new(1.),
        MeasurePoint::<Cycle>::new(1.).convert::<Radian>(),
        SignedDirection::<Cycle>::new(1.),
        SignedDirection::<Cycle>::new(1.).convert::<Radian>(),
        UnsignedDirection::<Cycle>::new(1.),
        UnsignedDirection::<Cycle>::new(1.).convert::<Radian>(),
    );
    println!(
        "  Gradian: {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Gradian>::new(1.),
        Measure::<Gradian>::new(1.).convert::<Radian>(),
        MeasurePoint::<Gradian>::new(1.),
        MeasurePoint::<Gradian>::new(1.).convert::<Radian>(),
        SignedDirection::<Gradian>::new(1.),
        SignedDirection::<Gradian>::new(1.).convert::<Radian>(),
        UnsignedDirection::<Gradian>::new(1.),
        UnsignedDirection::<Gradian>::new(1.).convert::<Radian>(),
    );
    println!(
        "  Degree: {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Degree>::new(1.),
        Measure::<Degree>::new(1.).convert::<Radian>(),
        MeasurePoint::<Degree>::new(1.),
        MeasurePoint::<Degree>::new(1.).convert::<Radian>(),
        SignedDirection::<Degree>::new(1.),
        SignedDirection::<Degree>::new(1.).convert::<Radian>(),
        UnsignedDirection::<Degree>::new(1.),
        UnsignedDirection::<Degree>::new(1.).convert::<Radian>(),
    );
    println!(
        "  ArcMinute: {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<ArcMinute>::new(1.),
        Measure::<ArcMinute>::new(1.).convert::<Radian>(),
        MeasurePoint::<ArcMinute>::new(1.),
        MeasurePoint::<ArcMinute>::new(1.).convert::<Radian>(),
        SignedDirection::<ArcMinute>::new(1.),
        SignedDirection::<ArcMinute>::new(1.).convert::<Radian>(),
        UnsignedDirection::<ArcMinute>::new(1.),
        UnsignedDirection::<ArcMinute>::new(1.).convert::<Radian>(),
    );
    println!(
        "  ArcSecond: {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<ArcSecond>::new(1.),
        Measure::<ArcSecond>::new(1.).convert::<Radian>(),
        MeasurePoint::<ArcSecond>::new(1.),
        MeasurePoint::<ArcSecond>::new(1.).convert::<Radian>(),
        SignedDirection::<ArcSecond>::new(1.),
        SignedDirection::<ArcSecond>::new(1.).convert::<Radian>(),
        UnsignedDirection::<ArcSecond>::new(1.),
        UnsignedDirection::<ArcSecond>::new(1.).convert::<Radian>(),
    );
    println!();
}

fn print_all_angular_acceleration_units() {
    println!("* AngularAcceleration units");
    println!(
        "  RadianPerSquareSecond: {}, {};",
        Measure::<RadianPerSquareSecond>::new(1.),
        MeasurePoint::<RadianPerSquareSecond>::new(1.),
    );
    println!();
}

fn print_all_angular_momentum_units() {
    println!("* AngularMomentum units");
    println!(
        "  KilogramSquareMetrePerSecond: {}, {}, {}, {}, {}, {};",
        Measure::<KilogramSquareMetrePerSecond>::new(1.),
        MeasurePoint::<KilogramSquareMetrePerSecond>::new(1.),
        Measure2d::<KilogramSquareMetrePerSecond>::new([1., 2.]),
        MeasurePoint2d::<KilogramSquareMetrePerSecond>::new([1., 2.]),
        Measure3d::<KilogramSquareMetrePerSecond>::new([1., 2., 3.]),
        MeasurePoint3d::<KilogramSquareMetrePerSecond>::new([1., 2., 3.]),
    );
    println!(
        "  GramSquareCentimetrePerSecond: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<GramSquareCentimetrePerSecond>::new(1.),
        Measure::<GramSquareCentimetrePerSecond>::new(1.).convert::<KilogramSquareMetrePerSecond>(),
        MeasurePoint::<GramSquareCentimetrePerSecond>::new(1.),
        MeasurePoint::<GramSquareCentimetrePerSecond>::new(1.).convert::<KilogramSquareMetrePerSecond>(),
        Measure2d::<GramSquareCentimetrePerSecond>::new([1., 2.]),
        Measure2d::<GramSquareCentimetrePerSecond>::new([1., 2.]).convert::<KilogramSquareMetrePerSecond>(),
        MeasurePoint2d::<GramSquareCentimetrePerSecond>::new([1., 2.]),
        MeasurePoint2d::<GramSquareCentimetrePerSecond>::new([1., 2.]).convert::<KilogramSquareMetrePerSecond>(),
        Measure3d::<GramSquareCentimetrePerSecond>::new([1., 2., 3.]),
        Measure3d::<GramSquareCentimetrePerSecond>::new([1., 2., 3.]).convert::<KilogramSquareMetrePerSecond>(),
        MeasurePoint3d::<GramSquareCentimetrePerSecond>::new([1., 2., 3.]),
        MeasurePoint3d::<GramSquareCentimetrePerSecond>::new([1., 2., 3.]).convert::<KilogramSquareMetrePerSecond>(),
    );
    println!();
}

fn print_all_area_units() {
    println!("* Area units");
    println!(
        "  SquareMetre: {}, {};",
        Measure::<SquareMetre>::new(1.),
        MeasurePoint::<SquareMetre>::new(1.)
    );
    println!(
        "  SquareKilometre: {} == {}, {} == {};",
        Measure::<SquareKilometre>::new(1.),
        Measure::<SquareKilometre>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareKilometre>::new(1.),
        MeasurePoint::<SquareKilometre>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  Hectare: {} == {}, {} == {};",
        Measure::<Hectare>::new(1.),
        Measure::<Hectare>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<Hectare>::new(1.),
        MeasurePoint::<Hectare>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  Are: {} == {}, {} == {};",
        Measure::<Are>::new(1.),
        Measure::<Are>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<Are>::new(1.),
        MeasurePoint::<Are>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareDecimetre: {} == {}, {} == {};",
        Measure::<SquareDecimetre>::new(1.),
        Measure::<SquareDecimetre>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareDecimetre>::new(1.),
        MeasurePoint::<SquareDecimetre>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareCentimetre: {} == {}, {} == {};",
        Measure::<SquareCentimetre>::new(1.),
        Measure::<SquareCentimetre>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareCentimetre>::new(1.),
        MeasurePoint::<SquareCentimetre>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareMillimetre: {} == {}, {} == {};",
        Measure::<SquareMillimetre>::new(1.),
        Measure::<SquareMillimetre>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareMillimetre>::new(1.),
        MeasurePoint::<SquareMillimetre>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareInch: {} == {}, {} == {};",
        Measure::<SquareInch>::new(1.),
        Measure::<SquareInch>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareInch>::new(1.),
        MeasurePoint::<SquareInch>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareFoot: {} == {}, {} == {};",
        Measure::<SquareFoot>::new(1.),
        Measure::<SquareFoot>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareFoot>::new(1.),
        MeasurePoint::<SquareFoot>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareYard: {} == {}, {} == {};",
        Measure::<SquareYard>::new(1.),
        Measure::<SquareYard>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareYard>::new(1.),
        MeasurePoint::<SquareYard>::new(1.).convert::<SquareMetre>(),
    );
    println!(
        "  SquareMile: {} == {}, {} == {};",
        Measure::<SquareMile>::new(1.),
        Measure::<SquareMile>::new(1.).convert::<SquareMetre>(),
        MeasurePoint::<SquareMile>::new(1.),
        MeasurePoint::<SquareMile>::new(1.).convert::<SquareMetre>(),
    );
    println!();
}

fn print_all_area_density_units() {
    println!("* AreaDensity units");
    println!(
        "  KilogramPerSquareMetre: {}, {};",
        Measure::<KilogramPerSquareMetre>::new(1.),
        MeasurePoint::<KilogramPerSquareMetre>::new(1.),
    );
    println!();
}

fn print_all_capacitance_units() {
    println!("* Capacitance units");
    println!(
        "  Farad: {}, {};",
        Measure::<Farad>::new(1.),
        MeasurePoint::<Farad>::new(1.),
    );
    println!(
        "  Millifarad: {} == {}, {} == {};",
        Measure::<Millifarad>::new(1.),
        Measure::<Millifarad>::new(1.).convert::<Farad>(),
        MeasurePoint::<Millifarad>::new(1.),
        MeasurePoint::<Millifarad>::new(1.).convert::<Farad>(),
    );
    println!(
        "  Microfarad: {} == {}, {} == {};",
        Measure::<Microfarad>::new(1.),
        Measure::<Microfarad>::new(1.).convert::<Farad>(),
        MeasurePoint::<Microfarad>::new(1.),
        MeasurePoint::<Microfarad>::new(1.).convert::<Farad>(),
    );
    println!(
        "  Nanofarad: {} == {}, {} == {};",
        Measure::<Nanofarad>::new(1.),
        Measure::<Nanofarad>::new(1.).convert::<Farad>(),
        MeasurePoint::<Nanofarad>::new(1.),
        MeasurePoint::<Nanofarad>::new(1.).convert::<Farad>(),
    );
    println!(
        "  Picofarad: {} == {}, {} == {};",
        Measure::<Picofarad>::new(1.),
        Measure::<Picofarad>::new(1.).convert::<Farad>(),
        MeasurePoint::<Picofarad>::new(1.),
        MeasurePoint::<Picofarad>::new(1.).convert::<Farad>(),
    );
    println!();
}

fn print_all_catalytic_activity_units() {
    println!("* CatalyticActivity units");
    println!(
        "  Katal: {}, {};",
        Measure::<Katal>::new(1.),
        MeasurePoint::<Katal>::new(1.),
    );
    println!();
}

fn print_all_chemical_potential_units() {
    println!("* ChemicalPotential units");
    println!(
        "  JoulePerMole: {}, {};",
        Measure::<JoulePerMole>::new(1.),
        MeasurePoint::<JoulePerMole>::new(1.),
    );
    println!();
}

fn print_all_current_density_units() {
    println!("* CurrentDensity units");
    println!(
        "  AmperePerSquareMetre: {}, {}, {}, {}, {}, {};",
        Measure::<AmperePerSquareMetre>::new(1.),
        MeasurePoint::<AmperePerSquareMetre>::new(1.),
        Measure2d::<AmperePerSquareMetre>::new([1., 2.]),
        MeasurePoint2d::<AmperePerSquareMetre>::new([1., 2.]),
        Measure3d::<AmperePerSquareMetre>::new([1., 2., 3.]),
        MeasurePoint3d::<AmperePerSquareMetre>::new([1., 2., 3.]),
    );
    println!();
}

fn print_all_dimensionless_units() {
    println!("* Dimensionless units");
    println!(
        "  One: {}, {}, {}, {}, {}, {};",
        Measure::<One>::new(1.),
        MeasurePoint::<One>::new(1.),
        Measure2d::<One>::new([1., 2.]),
        MeasurePoint2d::<One>::new([1., 2.]),
        Measure3d::<One>::new([1., 2., 3.]),
        MeasurePoint3d::<One>::new([1., 2., 3.])
    );
    println!(
        "  Mach: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Mach>::new(1.),
        Measure::<Mach>::new(1.).convert::<One>(),
        MeasurePoint::<Mach>::new(1.),
        MeasurePoint::<Mach>::new(1.).convert::<One>(),
        Measure2d::<Mach>::new([1., 2.]),
        Measure2d::<Mach>::new([1., 2.]).convert::<One>(),
        MeasurePoint2d::<Mach>::new([1., 2.]),
        MeasurePoint2d::<Mach>::new([1., 2.]).convert::<One>(),
        Measure3d::<Mach>::new([1., 2., 3.]),
        Measure3d::<Mach>::new([1., 2., 3.]).convert::<One>(),
        MeasurePoint3d::<Mach>::new([1., 2., 3.]),
        MeasurePoint3d::<Mach>::new([1., 2., 3.]).convert::<One>(),
    );
    println!();
}

fn print_all_dose_equivalent_units() {
    println!("* DoseEquivalent units");
    println!(
        "  Sievert: {}, {};",
        Measure::<Sievert>::new(1.),
        MeasurePoint::<Sievert>::new(1.),
    );
    println!(
        "  Rem: {} == {}, {} == {};",
        Measure::<Rem>::new(1.),
        Measure::<Rem>::new(1.).convert::<Sievert>(),
        MeasurePoint::<Rem>::new(1.),
        MeasurePoint::<Rem>::new(1.).convert::<Sievert>(),
    );
    println!();
}

fn print_all_dynamic_viscosity_units() {
    println!("* DynamicViscosity units");
    println!(
        "  PascalSecond: {}, {};",
        Measure::<PascalSecond>::new(1.),
        MeasurePoint::<PascalSecond>::new(1.),
    );
    println!();
}

fn print_all_electrical_conductance_units() {
    println!("* ElectricalConductance units");
    println!(
        "  Siemens: {}, {};",
        Measure::<Siemens>::new(1.),
        MeasurePoint::<Siemens>::new(1.),
    );
    println!();
}

fn print_all_electrical_conductivity_units() {
    println!("* ElectricalConductivity units");
    println!(
        "  SiemensPerMetre: {}, {};",
        Measure::<SiemensPerMetre>::new(1.),
        MeasurePoint::<SiemensPerMetre>::new(1.),
    );
    println!();
}

fn print_all_electrical_resistance_units() {
    println!("* ElectricalResistance units");
    println!(
        "  Ohm: {}, {};",
        Measure::<Ohm>::new(1.),
        MeasurePoint::<Ohm>::new(1.),
    );
    println!(
        "  Milliohm: {} == {}, {} == {};",
        Measure::<Milliohm>::new(1.),
        Measure::<Milliohm>::new(1.).convert::<Ohm>(),
        MeasurePoint::<Milliohm>::new(1.),
        MeasurePoint::<Milliohm>::new(1.).convert::<Ohm>(),
    );
    println!(
        "  Kiloohm: {} == {}, {} == {};",
        Measure::<Kiloohm>::new(1.),
        Measure::<Kiloohm>::new(1.).convert::<Ohm>(),
        MeasurePoint::<Kiloohm>::new(1.),
        MeasurePoint::<Kiloohm>::new(1.).convert::<Ohm>(),
    );
    println!();
}

fn print_all_electrical_resistivity_units() {
    println!("* ElectricalResistivity units");
    println!(
        "  OhmMetre: {}, {};",
        Measure::<OhmMetre>::new(1.),
        MeasurePoint::<OhmMetre>::new(1.),
    );
    println!();
}

fn print_all_electric_charge_units() {
    println!("* ElectricCharge units");
    println!(
        "  Coulomb: {}, {};",
        Measure::<Coulomb>::new(1.),
        MeasurePoint::<Coulomb>::new(1.),
    );
    println!(
        "  Millicoulomb: {} == {}, {} == {};",
        Measure::<Millicoulomb>::new(1.),
        Measure::<Millicoulomb>::new(1.).convert::<Coulomb>(),
        MeasurePoint::<Millicoulomb>::new(1.),
        MeasurePoint::<Millicoulomb>::new(1.).convert::<Coulomb>(),
    );
    println!(
        "  Microcoulomb: {} == {}, {} == {};",
        Measure::<Microcoulomb>::new(1.),
        Measure::<Microcoulomb>::new(1.).convert::<Coulomb>(),
        MeasurePoint::<Microcoulomb>::new(1.),
        MeasurePoint::<Microcoulomb>::new(1.).convert::<Coulomb>(),
    );
    println!(
        "  Nanocoulomb: {} == {}, {} == {};",
        Measure::<Nanocoulomb>::new(1.),
        Measure::<Nanocoulomb>::new(1.).convert::<Coulomb>(),
        MeasurePoint::<Nanocoulomb>::new(1.),
        MeasurePoint::<Nanocoulomb>::new(1.).convert::<Coulomb>(),
    );
    println!(
        "  Picocoulomb: {} == {}, {} == {};",
        Measure::<Picocoulomb>::new(1.),
        Measure::<Picocoulomb>::new(1.).convert::<Coulomb>(),
        MeasurePoint::<Picocoulomb>::new(1.),
        MeasurePoint::<Picocoulomb>::new(1.).convert::<Coulomb>(),
    );
    println!();
}

fn print_all_electric_charge_density_units() {
    println!("* ElectricChargeDensity units");
    println!(
        "  CoulombPerCubicMetre: {}, {};",
        Measure::<CoulombPerCubicMetre>::new(1.),
        MeasurePoint::<CoulombPerCubicMetre>::new(1.),
    );
    println!();
}

fn print_all_electric_current_units() {
    println!("* ElectricCurrent units");
    println!(
        "  Ampere: {}, {};",
        Measure::<Ampere>::new(1.),
        MeasurePoint::<Ampere>::new(1.),
    );
    println!(
        "  Milliampere: {} == {}, {} == {};",
        Measure::<Milliampere>::new(1.),
        Measure::<Milliampere>::new(1.).convert::<Ampere>(),
        MeasurePoint::<Milliampere>::new(1.),
        MeasurePoint::<Milliampere>::new(1.).convert::<Ampere>(),
    );
    println!(
        "  Microampere: {} == {}, {} == {};",
        Measure::<Microampere>::new(1.),
        Measure::<Microampere>::new(1.).convert::<Ampere>(),
        MeasurePoint::<Microampere>::new(1.),
        MeasurePoint::<Microampere>::new(1.).convert::<Ampere>(),
    );
    println!();
}

fn print_all_electric_displacement_units() {
    println!("* ElectricDisplacement units");
    println!(
        "  CoulombPerSquareMetre: {}, {};",
        Measure::<CoulombPerSquareMetre>::new(1.),
        MeasurePoint::<CoulombPerSquareMetre>::new(1.),
    );
    println!();
}

fn print_all_electric_field_strength_units() {
    println!("* ElectricFieldStrength units");
    println!(
        "  VoltPerMetre: {}, {}, {}, {}, {}, {};",
        Measure::<VoltPerMetre>::new(1.),
        MeasurePoint::<VoltPerMetre>::new(1.),
        Measure2d::<VoltPerMetre>::new([1., 2.]),
        MeasurePoint2d::<VoltPerMetre>::new([1., 2.]),
        Measure3d::<VoltPerMetre>::new([1., 2., 3.]),
        MeasurePoint3d::<VoltPerMetre>::new([1., 2., 3.]),
    );
    println!();
}

fn print_all_electric_potential_units() {
    println!("* ElectricPotential units");
    println!(
        "  Volt: {}, {};",
        Measure::<Volt>::new(1.),
        MeasurePoint::<Volt>::new(1.),
    );
    println!(
        "  Kilovolt: {} == {}, {} == {};",
        Measure::<Kilovolt>::new(1.),
        Measure::<Kilovolt>::new(1.).convert::<Volt>(),
        MeasurePoint::<Kilovolt>::new(1.),
        MeasurePoint::<Kilovolt>::new(1.).convert::<Volt>(),
    );
    println!(
        "  Millivolt: {} == {}, {} == {};",
        Measure::<Millivolt>::new(1.),
        Measure::<Millivolt>::new(1.).convert::<Volt>(),
        MeasurePoint::<Millivolt>::new(1.),
        MeasurePoint::<Millivolt>::new(1.).convert::<Volt>(),
    );
    println!(
        "  Microvolt: {} == {}, {} == {};",
        Measure::<Microvolt>::new(1.),
        Measure::<Microvolt>::new(1.).convert::<Volt>(),
        MeasurePoint::<Microvolt>::new(1.),
        MeasurePoint::<Microvolt>::new(1.).convert::<Volt>(),
    );
    println!();
}

fn print_all_energy_units() {
    println!("* Energy units");
    println!(
        "  Joule: {}, {};",
        Measure::<Joule>::new(1.),
        MeasurePoint::<Joule>::new(1.),
    );
    println!(
        "  WattHour: {} == {}, {} == {};",
        Measure::<WattHour>::new(1.),
        Measure::<WattHour>::new(1.).convert::<Joule>(),
        MeasurePoint::<WattHour>::new(1.),
        MeasurePoint::<WattHour>::new(1.).convert::<Joule>(),
    );
    println!(
        "  KilowattHour: {} == {}, {} == {};",
        Measure::<KilowattHour>::new(1.),
        Measure::<KilowattHour>::new(1.).convert::<Joule>(),
        MeasurePoint::<KilowattHour>::new(1.),
        MeasurePoint::<KilowattHour>::new(1.).convert::<Joule>(),
    );
    println!(
        "  MegawattHour: {} == {}, {} == {};",
        Measure::<MegawattHour>::new(1.),
        Measure::<MegawattHour>::new(1.).convert::<Joule>(),
        MeasurePoint::<MegawattHour>::new(1.),
        MeasurePoint::<MegawattHour>::new(1.).convert::<Joule>(),
    );
    println!(
        "  Calorie: {} == {}, {} == {};",
        Measure::<Calorie>::new(1.),
        Measure::<Calorie>::new(1.).convert::<Joule>(),
        MeasurePoint::<Calorie>::new(1.),
        MeasurePoint::<Calorie>::new(1.).convert::<Joule>(),
    );
    println!(
        "  Kilocalorie: {} == {}, {} == {};",
        Measure::<Kilocalorie>::new(1.),
        Measure::<Kilocalorie>::new(1.).convert::<Joule>(),
        MeasurePoint::<Kilocalorie>::new(1.),
        MeasurePoint::<Kilocalorie>::new(1.).convert::<Joule>(),
    );
    println!(
        "  Electronvlt: {} == {}, {} == {};",
        Measure::<Electronvolt>::new(1.),
        Measure::<Electronvolt>::new(1.).convert::<Joule>(),
        MeasurePoint::<Electronvolt>::new(1.),
        MeasurePoint::<Electronvolt>::new(1.).convert::<Joule>(),
    );
    println!(
        "  Kiloelectronvolt: {} == {}, {} == {};",
        Measure::<Kiloelectronvolt>::new(1.),
        Measure::<Kiloelectronvolt>::new(1.).convert::<Joule>(),
        MeasurePoint::<Kiloelectronvolt>::new(1.),
        MeasurePoint::<Kiloelectronvolt>::new(1.).convert::<Joule>(),
    );
    println!(
        "  MegaelectronVolt: {} == {}, {} == {};",
        Measure::<Megaelectronvolt>::new(1.),
        Measure::<Megaelectronvolt>::new(1.).convert::<Joule>(),
        MeasurePoint::<Megaelectronvolt>::new(1.),
        MeasurePoint::<Megaelectronvolt>::new(1.).convert::<Joule>(),
    );
    println!(
        "  Gigaelectronvolt: {} == {}, {} == {};",
        Measure::<Gigaelectronvolt>::new(1.),
        Measure::<Gigaelectronvolt>::new(1.).convert::<Joule>(),
        MeasurePoint::<Gigaelectronvolt>::new(1.),
        MeasurePoint::<Gigaelectronvolt>::new(1.).convert::<Joule>(),
    );
    println!(
        "  Teraelectronvolt: {} == {}, {} == {};",
        Measure::<Teraelectronvolt>::new(1.),
        Measure::<Teraelectronvolt>::new(1.).convert::<Joule>(),
        MeasurePoint::<Teraelectronvolt>::new(1.),
        MeasurePoint::<Teraelectronvolt>::new(1.).convert::<Joule>(),
    );
    println!();
}

fn print_all_energy_density_units() {
    println!("* EnergyDensity units");
    println!(
        "  JoulePerCubicMetre: {}, {};",
        Measure::<JoulePerCubicMetre>::new(1.),
        MeasurePoint::<JoulePerCubicMetre>::new(1.),
    );
    println!();
}

fn print_all_entropy_units() {
    println!("* Entropy units");
    println!(
        "  JoulePerKelvin: {}, {};",
        Measure::<JoulePerKelvin>::new(1.),
        MeasurePoint::<JoulePerKelvin>::new(1.),
    );
    println!();
}

fn print_all_force_units() {
    println!("* Force units");
    println!(
        "  Newton: {}, {}, {}, {}, {}, {};",
        Measure::<Newton>::new(1.),
        MeasurePoint::<Newton>::new(1.),
        Measure2d::<Newton>::new([1., 2.]),
        MeasurePoint2d::<Newton>::new([1., 2.]),
        Measure3d::<Newton>::new([1., 2., 3.]),
        MeasurePoint3d::<Newton>::new([1., 2., 3.]),
    );
    println!(
        "  Dyne: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Dyne>::new(1.),
        Measure::<Dyne>::new(1.).convert::<Newton>(),
        MeasurePoint::<Dyne>::new(1.),
        MeasurePoint::<Dyne>::new(1.).convert::<Newton>(),
        Measure2d::<Dyne>::new([1., 2.]),
        Measure2d::<Dyne>::new([1., 2.]).convert::<Newton>(),
        MeasurePoint2d::<Dyne>::new([1., 2.]),
        MeasurePoint2d::<Dyne>::new([1., 2.]).convert::<Newton>(),
        Measure3d::<Dyne>::new([1., 2., 3.]),
        Measure3d::<Dyne>::new([1., 2., 3.]).convert::<Newton>(),
        MeasurePoint3d::<Dyne>::new([1., 2., 3.]),
        MeasurePoint3d::<Dyne>::new([1., 2., 3.]).convert::<Newton>(),
    );
    println!(
        "  KilogramForce: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<KilogramForce>::new(1.),
        Measure::<KilogramForce>::new(1.).convert::<Newton>(),
        MeasurePoint::<KilogramForce>::new(1.),
        MeasurePoint::<KilogramForce>::new(1.).convert::<Newton>(),
        Measure2d::<KilogramForce>::new([1., 2.]),
        Measure2d::<KilogramForce>::new([1., 2.]).convert::<Newton>(),
        MeasurePoint2d::<KilogramForce>::new([1., 2.]),
        MeasurePoint2d::<KilogramForce>::new([1., 2.]).convert::<Newton>(),
        Measure3d::<KilogramForce>::new([1., 2., 3.]),
        Measure3d::<KilogramForce>::new([1., 2., 3.]).convert::<Newton>(),
        MeasurePoint3d::<KilogramForce>::new([1., 2., 3.]),
        MeasurePoint3d::<KilogramForce>::new([1., 2., 3.]).convert::<Newton>(),
    );
    println!(
        "  PoundForce: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<PoundForce>::new(1.),
        Measure::<PoundForce>::new(1.).convert::<Newton>(),
        MeasurePoint::<PoundForce>::new(1.),
        MeasurePoint::<PoundForce>::new(1.).convert::<Newton>(),
        Measure2d::<PoundForce>::new([1., 2.]),
        Measure2d::<PoundForce>::new([1., 2.]).convert::<Newton>(),
        MeasurePoint2d::<PoundForce>::new([1., 2.]),
        MeasurePoint2d::<PoundForce>::new([1., 2.]).convert::<Newton>(),
        Measure3d::<PoundForce>::new([1., 2., 3.]),
        Measure3d::<PoundForce>::new([1., 2., 3.]).convert::<Newton>(),
        MeasurePoint3d::<PoundForce>::new([1., 2., 3.]),
        MeasurePoint3d::<PoundForce>::new([1., 2., 3.]).convert::<Newton>(),
    );
    println!(
        "  Poundal: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Poundal>::new(1.),
        Measure::<Poundal>::new(1.).convert::<Newton>(),
        MeasurePoint::<Poundal>::new(1.),
        MeasurePoint::<Poundal>::new(1.).convert::<Newton>(),
        Measure2d::<Poundal>::new([1., 2.]),
        Measure2d::<Poundal>::new([1., 2.]).convert::<Newton>(),
        MeasurePoint2d::<Poundal>::new([1., 2.]),
        MeasurePoint2d::<Poundal>::new([1., 2.]).convert::<Newton>(),
        Measure3d::<Poundal>::new([1., 2., 3.]),
        Measure3d::<Poundal>::new([1., 2., 3.]).convert::<Newton>(),
        MeasurePoint3d::<Poundal>::new([1., 2., 3.]),
        MeasurePoint3d::<Poundal>::new([1., 2., 3.]).convert::<Newton>(),
    );
    println!();
}

fn print_all_frequency_units() {
    println!("* All Frequency units");
    println!(
        "  Hertz: {}, {};",
        Measure::<Hertz>::new(1.),
        MeasurePoint::<Hertz>::new(1.),
    );
    println!(
        "  Kilohertz: {} == {}, {} == {};",
        Measure::<Kilohertz>::new(1.),
        Measure::<Kilohertz>::new(1.).convert::<Hertz>(),
        MeasurePoint::<Kilohertz>::new(1.),
        MeasurePoint::<Kilohertz>::new(1.).convert::<Hertz>(),
    );
    println!(
        "  Megahertz: {} == {}, {} == {};",
        Measure::<Megahertz>::new(1.),
        Measure::<Megahertz>::new(1.).convert::<Hertz>(),
        MeasurePoint::<Megahertz>::new(1.),
        MeasurePoint::<Megahertz>::new(1.).convert::<Hertz>(),
    );
    println!(
        "  Gigahertz: {} == {}, {} == {};",
        Measure::<Gigahertz>::new(1.),
        Measure::<Gigahertz>::new(1.).convert::<Hertz>(),
        MeasurePoint::<Gigahertz>::new(1.),
        MeasurePoint::<Gigahertz>::new(1.).convert::<Hertz>(),
    );
    println!(
        "  RadianPerSecond: {} == {}, {} == {};",
        Measure::<RadianPerSecond>::new(1.),
        Measure::<RadianPerSecond>::new(1.).convert::<Hertz>(),
        MeasurePoint::<RadianPerSecond>::new(1.),
        MeasurePoint::<RadianPerSecond>::new(1.).convert::<Hertz>(),
    );
    println!(
        "  CyclePerMinute: {} == {}, {} == {};",
        Measure::<CyclePerMinute>::new(1.),
        Measure::<CyclePerMinute>::new(1.).convert::<Hertz>(),
        MeasurePoint::<CyclePerMinute>::new(1.),
        MeasurePoint::<CyclePerMinute>::new(1.).convert::<Hertz>(),
    );
    println!();
}

fn print_all_illuminance_units() {
    println!("* Illuminance units");
    println!(
        "  Lux: {}, {};",
        Measure::<Lux>::new(1.),
        MeasurePoint::<Lux>::new(1.),
    );
    println!(
        "  Phot: {} == {}, {} == {};",
        Measure::<Phot>::new(1.),
        Measure::<Phot>::new(1.).convert::<Lux>(),
        MeasurePoint::<Phot>::new(1.),
        MeasurePoint::<Phot>::new(1.).convert::<Lux>(),
    );
    println!(
        "  FootCandle: {} == {}, {} == {};",
        Measure::<FootCandle>::new(1.),
        Measure::<FootCandle>::new(1.).convert::<Lux>(),
        MeasurePoint::<FootCandle>::new(1.),
        MeasurePoint::<FootCandle>::new(1.).convert::<Lux>(),
    );
    println!();
}

fn print_all_inductance_units() {
    println!("* Inductance units");
    println!(
        "  Henry: {}, {};",
        Measure::<Henry>::new(1.),
        MeasurePoint::<Henry>::new(1.),
    );
    println!();
}

fn print_all_information_units() {
    println!("* Information units");
    println!(
        "  Bit: {}, {};",
        Measure::<Bit>::new(1.),
        MeasurePoint::<Bit>::new(1.),
    );
    println!(
        "  Byte: {} == {}, {} == {};",
        Measure::<Byte>::new(1.),
        Measure::<Byte>::new(1.).convert::<Bit>(),
        MeasurePoint::<Byte>::new(1.),
        MeasurePoint::<Byte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Kilobit: {} == {}, {} == {};",
        Measure::<Kilobit>::new(1.),
        Measure::<Kilobit>::new(1.).convert::<Bit>(),
        MeasurePoint::<Kilobit>::new(1.),
        MeasurePoint::<Kilobit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Kilobyte: {} == {}, {} == {};",
        Measure::<Kilobyte>::new(1.),
        Measure::<Kilobyte>::new(1.).convert::<Bit>(),
        MeasurePoint::<Kilobyte>::new(1.),
        MeasurePoint::<Kilobyte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Kibibit: {} == {}, {} == {};",
        Measure::<Kibibit>::new(1.),
        Measure::<Kibibit>::new(1.).convert::<Bit>(),
        MeasurePoint::<Kibibit>::new(1.),
        MeasurePoint::<Kibibit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Kibibyte: {} == {}, {} == {};",
        Measure::<Kibibyte>::new(1.),
        Measure::<Kibibyte>::new(1.).convert::<Bit>(),
        MeasurePoint::<Kibibyte>::new(1.),
        MeasurePoint::<Kibibyte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Megabit: {} == {}, {} == {};",
        Measure::<Megabit>::new(1.),
        Measure::<Megabit>::new(1.).convert::<Bit>(),
        MeasurePoint::<Megabit>::new(1.),
        MeasurePoint::<Megabit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Megabyte: {} == {}, {} == {};",
        Measure::<Megabyte>::new(1.),
        Measure::<Megabyte>::new(1.).convert::<Bit>(),
        MeasurePoint::<Megabyte>::new(1.),
        MeasurePoint::<Megabyte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Mebibit: {} == {}, {} == {};",
        Measure::<Mebibit>::new(1.),
        Measure::<Mebibit>::new(1.).convert::<Bit>(),
        MeasurePoint::<Mebibit>::new(1.),
        MeasurePoint::<Mebibit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Mebibyte: {} == {}, {} == {};",
        Measure::<Mebibyte>::new(1.),
        Measure::<Mebibyte>::new(1.).convert::<Bit>(),
        MeasurePoint::<Mebibyte>::new(1.),
        MeasurePoint::<Mebibyte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Gigabit: {} == {}, {} == {};",
        Measure::<Gigabit>::new(1.),
        Measure::<Gigabit>::new(1.).convert::<Bit>(),
        MeasurePoint::<Gigabit>::new(1.),
        MeasurePoint::<Gigabit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Gigabyte: {} == {}, {} == {};",
        Measure::<Gigabyte>::new(1.),
        Measure::<Gigabyte>::new(1.).convert::<Bit>(),
        MeasurePoint::<Gigabyte>::new(1.),
        MeasurePoint::<Gigabyte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Gibibit: {} == {}, {} == {};",
        Measure::<Gibibit>::new(1.),
        Measure::<Gibibit>::new(1.).convert::<Bit>(),
        MeasurePoint::<Gibibit>::new(1.),
        MeasurePoint::<Gibibit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Gibibyte: {} == {}, {} == {};",
        Measure::<Gibibyte>::new(1.),
        Measure::<Gibibyte>::new(1.).convert::<Bit>(),
        MeasurePoint::<Gibibyte>::new(1.),
        MeasurePoint::<Gibibyte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Terabit: {} == {}, {} == {};",
        Measure::<Terabit>::new(1.),
        Measure::<Terabit>::new(1.).convert::<Bit>(),
        MeasurePoint::<Terabit>::new(1.),
        MeasurePoint::<Terabit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Terabyte: {} == {}, {} == {};",
        Measure::<Terabyte>::new(1.),
        Measure::<Terabyte>::new(1.).convert::<Bit>(),
        MeasurePoint::<Terabyte>::new(1.),
        MeasurePoint::<Terabyte>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Tebibit: {} == {}, {} == {};",
        Measure::<Tebibit>::new(1.),
        Measure::<Tebibit>::new(1.).convert::<Bit>(),
        MeasurePoint::<Tebibit>::new(1.),
        MeasurePoint::<Tebibit>::new(1.).convert::<Bit>(),
    );
    println!(
        "  Tebibyte: {} == {}, {} == {};",
        Measure::<Tebibyte>::new(1.),
        Measure::<Tebibyte>::new(1.).convert::<Bit>(),
        MeasurePoint::<Tebibyte>::new(1.),
        MeasurePoint::<Tebibyte>::new(1.).convert::<Bit>(),
    );
    println!();
}

fn print_all_information_rate_units() {
    println!("* InformationRate units");
    println!(
        "  BitPerSecond: {}, {};",
        Measure::<BitPerSecond>::new(1.),
        MeasurePoint::<BitPerSecond>::new(1.),
    );
    println!(
        "  BytePerSecond: {} == {}, {} == {};",
        Measure::<BytePerSecond>::new(1.),
        Measure::<BytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<BytePerSecond>::new(1.),
        MeasurePoint::<BytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );

    println!(
        "  KilobitPerSecond: {} == {}, {} == {};",
        Measure::<KilobitPerSecond>::new(1.),
        Measure::<KilobitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<KilobitPerSecond>::new(1.),
        MeasurePoint::<KilobitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  KilobytePerSecond: {} == {}, {} == {};",
        Measure::<KilobytePerSecond>::new(1.),
        Measure::<KilobytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<KilobytePerSecond>::new(1.),
        MeasurePoint::<KilobytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  KibibitPerSecond: {} == {}, {} == {};",
        Measure::<KibibitPerSecond>::new(1.),
        Measure::<KibibitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<KibibitPerSecond>::new(1.),
        MeasurePoint::<KibibitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  KibibytePerSecond: {} == {}, {} == {};",
        Measure::<KibibytePerSecond>::new(1.),
        Measure::<KibibytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<KibibytePerSecond>::new(1.),
        MeasurePoint::<KibibytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );

    println!(
        "  MegabitPerSecond: {} == {}, {} == {};",
        Measure::<MegabitPerSecond>::new(1.),
        Measure::<MegabitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<MegabitPerSecond>::new(1.),
        MeasurePoint::<MegabitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  MegabytePerSecond: {} == {}, {} == {};",
        Measure::<MegabytePerSecond>::new(1.),
        Measure::<MegabytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<MegabytePerSecond>::new(1.),
        MeasurePoint::<MegabytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  MebibitPerSecond: {} == {}, {} == {};",
        Measure::<MebibitPerSecond>::new(1.),
        Measure::<MebibitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<MebibitPerSecond>::new(1.),
        MeasurePoint::<MebibitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  MebibytePerSecond: {} == {}, {} == {};",
        Measure::<MebibytePerSecond>::new(1.),
        Measure::<MebibytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<MebibytePerSecond>::new(1.),
        MeasurePoint::<MebibytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );

    println!(
        "  GigabitPerSecond: {} == {}, {} == {};",
        Measure::<GigabitPerSecond>::new(1.),
        Measure::<GigabitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<GigabitPerSecond>::new(1.),
        MeasurePoint::<GigabitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  GigabytePerSecond: {} == {}, {} == {};",
        Measure::<GigabytePerSecond>::new(1.),
        Measure::<GigabytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<GigabytePerSecond>::new(1.),
        MeasurePoint::<GigabytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  GibibitPerSecond: {} == {}, {} == {};",
        Measure::<GibibitPerSecond>::new(1.),
        Measure::<GibibitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<GibibitPerSecond>::new(1.),
        MeasurePoint::<GibibitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  GibibytePerSecond: {} == {}, {} == {};",
        Measure::<GibibytePerSecond>::new(1.),
        Measure::<GibibytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<GibibytePerSecond>::new(1.),
        MeasurePoint::<GibibytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );

    println!(
        "  TerabitPerSecond: {} == {}, {} == {};",
        Measure::<TerabitPerSecond>::new(1.),
        Measure::<TerabitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<TerabitPerSecond>::new(1.),
        MeasurePoint::<TerabitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  TerabytePerSecond: {} == {}, {} == {};",
        Measure::<TerabytePerSecond>::new(1.),
        Measure::<TerabytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<TerabytePerSecond>::new(1.),
        MeasurePoint::<TerabytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  TebibitPerSecond: {} == {}, {} == {};",
        Measure::<TebibitPerSecond>::new(1.),
        Measure::<TebibitPerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<TebibitPerSecond>::new(1.),
        MeasurePoint::<TebibitPerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!(
        "  TebibytePerSecond: {} == {}, {} == {};",
        Measure::<TebibytePerSecond>::new(1.),
        Measure::<TebibytePerSecond>::new(1.).convert::<BitPerSecond>(),
        MeasurePoint::<TebibytePerSecond>::new(1.),
        MeasurePoint::<TebibytePerSecond>::new(1.).convert::<BitPerSecond>(),
    );
    println!();
}

fn print_all_irradiance_units() {
    println!("* Irradiance units");
    println!(
        "  WattPerSquareMetre: {}, {};",
        Measure::<WattPerSquareMetre>::new(1.),
        MeasurePoint::<WattPerSquareMetre>::new(1.),
    );
    println!();
}

fn print_all_kinematic_viscosity_units() {
    println!("* KinematicViscosity units");
    println!(
        "  SquareMetrePerSecond: {}, {};",
        Measure::<SquareMetrePerSecond>::new(1.),
        MeasurePoint::<SquareMetrePerSecond>::new(1.),
    );
    println!(
        "  Stoke: {} == {}, {} == {};",
        Measure::<Stoke>::new(1.),
        Measure::<Stoke>::new(1.).convert::<SquareMetrePerSecond>(),
        MeasurePoint::<Stoke>::new(1.),
        MeasurePoint::<Stoke>::new(1.).convert::<SquareMetrePerSecond>(),
    );
    println!(
        "  Centistoke: {} == {}, {} == {};",
        Measure::<Centistoke>::new(1.),
        Measure::<Centistoke>::new(1.).convert::<SquareMetrePerSecond>(),
        MeasurePoint::<Centistoke>::new(1.),
        MeasurePoint::<Centistoke>::new(1.).convert::<SquareMetrePerSecond>(),
    );
    println!();
}

fn print_all_length_units() {
    println!("* Length units");
    println!(
        "  Metre: {}, {}, {}, {}, {}, {};",
        Measure::<Metre>::new(1.),
        MeasurePoint::<Metre>::new(1.),
        Measure2d::<Metre>::new([1., 2.]),
        MeasurePoint2d::<Metre>::new([1., 2.]),
        Measure3d::<Metre>::new([1., 2., 3.]),
        MeasurePoint3d::<Metre>::new([1., 2., 3.]),
    );
    println!(
        "  AstronomicalUnit: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<AstronomicalUnit>::new(1.),
        Measure::<AstronomicalUnit>::new(1.).convert::<Metre>(),
        MeasurePoint::<AstronomicalUnit>::new(1.),
        MeasurePoint::<AstronomicalUnit>::new(1.).convert::<Metre>(),
        Measure2d::<AstronomicalUnit>::new([1., 2.]),
        Measure2d::<AstronomicalUnit>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<AstronomicalUnit>::new([1., 2.]),
        MeasurePoint2d::<AstronomicalUnit>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<AstronomicalUnit>::new([1., 2., 3.]),
        Measure3d::<AstronomicalUnit>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<AstronomicalUnit>::new([1., 2., 3.]),
        MeasurePoint3d::<AstronomicalUnit>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Parsec: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Parsec>::new(1.),
        Measure::<Parsec>::new(1.).convert::<Metre>(),
        MeasurePoint::<Parsec>::new(1.),
        MeasurePoint::<Parsec>::new(1.).convert::<Metre>(),
        Measure2d::<Parsec>::new([1., 2.]),
        Measure2d::<Parsec>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Parsec>::new([1., 2.]),
        MeasurePoint2d::<Parsec>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Parsec>::new([1., 2., 3.]),
        Measure3d::<Parsec>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Parsec>::new([1., 2., 3.]),
        MeasurePoint3d::<Parsec>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  LightYear: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<LightYear>::new(1.),
        Measure::<LightYear>::new(1.).convert::<Metre>(),
        MeasurePoint::<LightYear>::new(1.),
        MeasurePoint::<LightYear>::new(1.).convert::<Metre>(),
        Measure2d::<LightYear>::new([1., 2.]),
        Measure2d::<LightYear>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<LightYear>::new([1., 2.]),
        MeasurePoint2d::<LightYear>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<LightYear>::new([1., 2., 3.]),
        Measure3d::<LightYear>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<LightYear>::new([1., 2., 3.]),
        MeasurePoint3d::<LightYear>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Kilometre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Kilometre>::new(1.),
        Measure::<Kilometre>::new(1.).convert::<Metre>(),
        MeasurePoint::<Kilometre>::new(1.),
        MeasurePoint::<Kilometre>::new(1.).convert::<Metre>(),
        Measure2d::<Kilometre>::new([1., 2.]),
        Measure2d::<Kilometre>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Kilometre>::new([1., 2.]),
        MeasurePoint2d::<Kilometre>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Kilometre>::new([1., 2., 3.]),
        Measure3d::<Kilometre>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Kilometre>::new([1., 2., 3.]),
        MeasurePoint3d::<Kilometre>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Hectometre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Hectometre>::new(1.),
        Measure::<Hectometre>::new(1.).convert::<Metre>(),
        MeasurePoint::<Hectometre>::new(1.),
        MeasurePoint::<Hectometre>::new(1.).convert::<Metre>(),
        Measure2d::<Hectometre>::new([1., 2.]),
        Measure2d::<Hectometre>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Hectometre>::new([1., 2.]),
        MeasurePoint2d::<Hectometre>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Hectometre>::new([1., 2., 3.]),
        Measure3d::<Hectometre>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Hectometre>::new([1., 2., 3.]),
        MeasurePoint3d::<Hectometre>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Decametre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Decametre>::new(1.),
        Measure::<Decametre>::new(1.).convert::<Metre>(),
        MeasurePoint::<Decametre>::new(1.),
        MeasurePoint::<Decametre>::new(1.).convert::<Metre>(),
        Measure2d::<Decametre>::new([1., 2.]),
        Measure2d::<Decametre>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Decametre>::new([1., 2.]),
        MeasurePoint2d::<Decametre>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Decametre>::new([1., 2., 3.]),
        Measure3d::<Decametre>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Decametre>::new([1., 2., 3.]),
        MeasurePoint3d::<Decametre>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Decimetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Decimetre>::new(1.),
        Measure::<Decimetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<Decimetre>::new(1.),
        MeasurePoint::<Decimetre>::new(1.).convert::<Metre>(),
        Measure2d::<Decimetre>::new([1., 2.]),
        Measure2d::<Decimetre>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Decimetre>::new([1., 2.]),
        MeasurePoint2d::<Decimetre>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Decimetre>::new([1., 2., 3.]),
        Measure3d::<Decimetre>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Decimetre>::new([1., 2., 3.]),
        MeasurePoint3d::<Decimetre>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Centimetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Centimetre>::new(1.),
        Measure::<Centimetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<Centimetre>::new(1.),
        MeasurePoint::<Centimetre>::new(1.).convert::<Metre>(),
        Measure2d::<Centimetre>::new([1., 2.]),
        Measure2d::<Centimetre>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Centimetre>::new([1., 2.]),
        MeasurePoint2d::<Centimetre>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Centimetre>::new([1., 2., 3.]),
        Measure3d::<Centimetre>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Centimetre>::new([1., 2., 3.]),
        MeasurePoint3d::<Centimetre>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Millimetre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Millimetre>::new(1.),
        Measure::<Millimetre>::new(1.).convert::<Metre>(),
        MeasurePoint::<Millimetre>::new(1.),
        MeasurePoint::<Millimetre>::new(1.).convert::<Metre>(),
        Measure2d::<Millimetre>::new([1., 2.]),
        Measure2d::<Millimetre>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Millimetre>::new([1., 2.]),
        MeasurePoint2d::<Millimetre>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Millimetre>::new([1., 2., 3.]),
        Measure3d::<Millimetre>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Millimetre>::new([1., 2., 3.]),
        MeasurePoint3d::<Millimetre>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Micrometre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Micrometre>::new(1.),
        Measure::<Micrometre>::new(1.).convert::<Metre>(),
        MeasurePoint::<Micrometre>::new(1.),
        MeasurePoint::<Micrometre>::new(1.).convert::<Metre>(),
        Measure2d::<Micrometre>::new([1., 2.]),
        Measure2d::<Micrometre>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Micrometre>::new([1., 2.]),
        MeasurePoint2d::<Micrometre>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Micrometre>::new([1., 2., 3.]),
        Measure3d::<Micrometre>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Micrometre>::new([1., 2., 3.]),
        MeasurePoint3d::<Micrometre>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Nanometre: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Nanometre>::new(1.),
        Measure::<Nanometre>::new(1.).convert::<Metre>(),
        MeasurePoint::<Nanometre>::new(1.),
        MeasurePoint::<Nanometre>::new(1.).convert::<Metre>(),
        Measure2d::<Nanometre>::new([1., 2.]),
        Measure2d::<Nanometre>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Nanometre>::new([1., 2.]),
        MeasurePoint2d::<Nanometre>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Nanometre>::new([1., 2., 3.]),
        Measure3d::<Nanometre>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Nanometre>::new([1., 2., 3.]),
        MeasurePoint3d::<Nanometre>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Angstrom: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Angstrom>::new(1.),
        Measure::<Angstrom>::new(1.).convert::<Metre>(),
        MeasurePoint::<Angstrom>::new(1.),
        MeasurePoint::<Angstrom>::new(1.).convert::<Metre>(),
        Measure2d::<Angstrom>::new([1., 2.]),
        Measure2d::<Angstrom>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Angstrom>::new([1., 2.]),
        MeasurePoint2d::<Angstrom>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Angstrom>::new([1., 2., 3.]),
        Measure3d::<Angstrom>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Angstrom>::new([1., 2., 3.]),
        MeasurePoint3d::<Angstrom>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Inch: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Inch>::new(1.),
        Measure::<Inch>::new(1.).convert::<Metre>(),
        MeasurePoint::<Inch>::new(1.),
        MeasurePoint::<Inch>::new(1.).convert::<Metre>(),
        Measure2d::<Inch>::new([1., 2.]),
        Measure2d::<Inch>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Inch>::new([1., 2.]),
        MeasurePoint2d::<Inch>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Inch>::new([1., 2., 3.]),
        Measure3d::<Inch>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Inch>::new([1., 2., 3.]),
        MeasurePoint3d::<Inch>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Foot: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Foot>::new(1.),
        Measure::<Foot>::new(1.).convert::<Metre>(),
        MeasurePoint::<Foot>::new(1.),
        MeasurePoint::<Foot>::new(1.).convert::<Metre>(),
        Measure2d::<Foot>::new([1., 2.]),
        Measure2d::<Foot>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Foot>::new([1., 2.]),
        MeasurePoint2d::<Foot>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Foot>::new([1., 2., 3.]),
        Measure3d::<Foot>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Foot>::new([1., 2., 3.]),
        MeasurePoint3d::<Foot>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Yard: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Yard>::new(1.),
        Measure::<Yard>::new(1.).convert::<Metre>(),
        MeasurePoint::<Yard>::new(1.),
        MeasurePoint::<Yard>::new(1.).convert::<Metre>(),
        Measure2d::<Yard>::new([1., 2.]),
        Measure2d::<Yard>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Yard>::new([1., 2.]),
        MeasurePoint2d::<Yard>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Yard>::new([1., 2., 3.]),
        Measure3d::<Yard>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Yard>::new([1., 2., 3.]),
        MeasurePoint3d::<Yard>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  Mile: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Mile>::new(1.),
        Measure::<Mile>::new(1.).convert::<Metre>(),
        MeasurePoint::<Mile>::new(1.),
        MeasurePoint::<Mile>::new(1.).convert::<Metre>(),
        Measure2d::<Mile>::new([1., 2.]),
        Measure2d::<Mile>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<Mile>::new([1., 2.]),
        MeasurePoint2d::<Mile>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<Mile>::new([1., 2., 3.]),
        Measure3d::<Mile>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<Mile>::new([1., 2., 3.]),
        MeasurePoint3d::<Mile>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!(
        "  NauticalMile: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<NauticalMile>::new(1.),
        Measure::<NauticalMile>::new(1.).convert::<Metre>(),
        MeasurePoint::<NauticalMile>::new(1.),
        MeasurePoint::<NauticalMile>::new(1.).convert::<Metre>(),
        Measure2d::<NauticalMile>::new([1., 2.]),
        Measure2d::<NauticalMile>::new([1., 2.]).convert::<Metre>(),
        MeasurePoint2d::<NauticalMile>::new([1., 2.]),
        MeasurePoint2d::<NauticalMile>::new([1., 2.]).convert::<Metre>(),
        Measure3d::<NauticalMile>::new([1., 2., 3.]),
        Measure3d::<NauticalMile>::new([1., 2., 3.]).convert::<Metre>(),
        MeasurePoint3d::<NauticalMile>::new([1., 2., 3.]),
        MeasurePoint3d::<NauticalMile>::new([1., 2., 3.]).convert::<Metre>(),
    );
    println!();
}

fn print_all_linear_density_units() {
    println!("* LinearDensity units");
    println!(
        "  KilogramPerMetre: {}, {};",
        Measure::<KilogramPerMetre>::new(1.),
        MeasurePoint::<KilogramPerMetre>::new(1.),
    );
    println!();
}

fn print_all_linear_electric_charge_density_units() {
    println!("* LinearElectricChargeDensity units");
    println!(
        "  CoulombPerMetre: {}, {};",
        Measure::<CoulombPerMetre>::new(1.),
        MeasurePoint::<CoulombPerMetre>::new(1.),
    );
    println!();
}

fn print_all_luminance_units() {
    println!("* Luminance units");
    println!(
        "  CandelaPerSquareMetre: {}, {};",
        Measure::<CandelaPerSquareMetre>::new(1.),
        MeasurePoint::<CandelaPerSquareMetre>::new(1.),
    );
    println!(
        "  Stilb: {} == {}, {} == {};",
        Measure::<Stilb>::new(1.),
        Measure::<Stilb>::new(1.).convert::<CandelaPerSquareMetre>(),
        MeasurePoint::<Stilb>::new(1.),
        MeasurePoint::<Stilb>::new(1.).convert::<CandelaPerSquareMetre>(),
    );
    println!();
}

fn print_all_luminous_flux_units() {
    println!("* LuminousFlux units");
    println!(
        "  Lumen: {}, {}, {}, {}, {}, {};",
        Measure::<Lumen>::new(1.),
        MeasurePoint::<Lumen>::new(1.),
        Measure2d::<Lumen>::new([1., 2.]),
        MeasurePoint2d::<Lumen>::new([1., 2.]),
        Measure3d::<Lumen>::new([1., 2., 3.]),
        MeasurePoint3d::<Lumen>::new([1., 2., 3.]),
    );
    println!();
}

fn print_all_luminous_intensity_units() {
    println!("* LuminousIntensity units");
    println!(
        "  Candela: {}, {};",
        Measure::<Candela>::new(1.),
        MeasurePoint::<Candela>::new(1.),
    );
    println!();
}

fn print_all_magnetic_field_strength_units() {
    println!("* MagneticFieldStrength units");
    println!(
        "  AmperePerMetre: {}, {}, {}, {}, {}, {};",
        Measure::<AmperePerMetre>::new(1.),
        MeasurePoint::<AmperePerMetre>::new(1.),
        Measure2d::<AmperePerMetre>::new([1., 2.]),
        MeasurePoint2d::<AmperePerMetre>::new([1., 2.]),
        Measure3d::<AmperePerMetre>::new([1., 2., 3.]),
        MeasurePoint3d::<AmperePerMetre>::new([1., 2., 3.]),
    );
    println!();
}

fn print_all_magnetic_flux_units() {
    println!("* MagneticFlux units");
    println!(
        "  Weber: {}, {};",
        Measure::<Weber>::new(1.),
        MeasurePoint::<Weber>::new(1.),
    );
    println!();
}

fn print_all_magnetic_flux_density_units() {
    println!("* MagneticFluxDensity units");
    println!(
        "  Tesla: {}, {}, {}, {}, {}, {};",
        Measure::<Tesla>::new(1.),
        MeasurePoint::<Tesla>::new(1.),
        Measure2d::<Tesla>::new([1., 2.]),
        MeasurePoint2d::<Tesla>::new([1., 2.]),
        Measure3d::<Tesla>::new([1., 2., 3.]),
        MeasurePoint3d::<Tesla>::new([1., 2., 3.]),
    );
    println!(
        "  Gauss: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Gauss>::new(1.),
        Measure::<Gauss>::new(1.).convert::<Tesla>(),
        MeasurePoint::<Gauss>::new(1.),
        MeasurePoint::<Gauss>::new(1.).convert::<Tesla>(),
        Measure2d::<Gauss>::new([1., 2.]),
        Measure2d::<Gauss>::new([1., 2.]).convert::<Tesla>(),
        MeasurePoint2d::<Gauss>::new([1., 2.]),
        MeasurePoint2d::<Gauss>::new([1., 2.]).convert::<Tesla>(),
        Measure3d::<Gauss>::new([1., 2., 3.]),
        Measure3d::<Gauss>::new([1., 2., 3.]).convert::<Tesla>(),
        MeasurePoint3d::<Gauss>::new([1., 2., 3.]),
        MeasurePoint3d::<Gauss>::new([1., 2., 3.]).convert::<Tesla>(),
    );
    println!();
}

fn print_all_magnetic_permeability_units() {
    println!("* MagneticPermeability units");
    println!(
        "  HenryPerMetre: {}, {};",
        Measure::<HenryPerMetre>::new(1.),
        MeasurePoint::<HenryPerMetre>::new(1.),
    );
    println!();
}

fn print_all_magnetic_reluctance_units() {
    println!("* MagneticReluctance units");
    println!(
        "  InverseHenry: {}, {};",
        Measure::<InverseHenry>::new(1.),
        MeasurePoint::<InverseHenry>::new(1.),
    );
    println!();
}

fn print_all_mass_units() {
    println!("* Mass units");
    println!(
        "  Kilogram: {}, {};",
        Measure::<Kilogram>::new(1.),
        MeasurePoint::<Kilogram>::new(1.),
    );
    println!(
        "  Tonne: {} == {}, {} == {};",
        Measure::<Tonne>::new(1.),
        Measure::<Tonne>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Tonne>::new(1.),
        MeasurePoint::<Tonne>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  MetricTon: {} == {}, {} == {};",
        Measure::<MetricTon>::new(1.),
        Measure::<MetricTon>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<MetricTon>::new(1.),
        MeasurePoint::<MetricTon>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  Hectogram: {} == {}, {} == {};",
        Measure::<Hectogram>::new(1.),
        Measure::<Hectogram>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Hectogram>::new(1.),
        MeasurePoint::<Hectogram>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  Decagram: {} == {}, {} == {};",
        Measure::<Decagram>::new(1.),
        Measure::<Decagram>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Decagram>::new(1.),
        MeasurePoint::<Decagram>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  Gram: {} == {}, {} == {};",
        Measure::<Gram>::new(1.),
        Measure::<Gram>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Gram>::new(1.),
        MeasurePoint::<Gram>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  Milligram: {} == {}, {} == {};",
        Measure::<Milligram>::new(1.),
        Measure::<Milligram>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Milligram>::new(1.),
        MeasurePoint::<Milligram>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  Microgram: {} == {}, {} == {};",
        Measure::<Microgram>::new(1.),
        Measure::<Microgram>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Microgram>::new(1.),
        MeasurePoint::<Microgram>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  Nanogram: {} == {}, {} == {};",
        Measure::<Nanogram>::new(1.),
        Measure::<Nanogram>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Nanogram>::new(1.),
        MeasurePoint::<Nanogram>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  ImperialTon: {} == {}, {} == {};",
        Measure::<ImperialTon>::new(1.),
        Measure::<ImperialTon>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<ImperialTon>::new(1.),
        MeasurePoint::<ImperialTon>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  LongTon: {} == {}, {} == {};",
        Measure::<LongTon>::new(1.),
        Measure::<LongTon>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<LongTon>::new(1.),
        MeasurePoint::<LongTon>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  USTon: {} == {}, {} == {};",
        Measure::<USTon>::new(1.),
        Measure::<USTon>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<USTon>::new(1.),
        MeasurePoint::<USTon>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  ShortTon: {} == {}, {} == {};",
        Measure::<ShortTon>::new(1.),
        Measure::<ShortTon>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<ShortTon>::new(1.),
        MeasurePoint::<ShortTon>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  Stone: {} == {}, {} == {};",
        Measure::<Stone>::new(1.),
        Measure::<Stone>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Stone>::new(1.),
        MeasurePoint::<Stone>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  Pound: {} == {}, {} == {};",
        Measure::<Pound>::new(1.),
        Measure::<Pound>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Pound>::new(1.),
        MeasurePoint::<Pound>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  Ounce: {} == {}, {} == {};",
        Measure::<Ounce>::new(1.),
        Measure::<Ounce>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Ounce>::new(1.),
        MeasurePoint::<Ounce>::new(1.).convert::<Kilogram>(),
    );
    println!(
        "  Carat: {} == {}, {} == {};",
        Measure::<Carat>::new(1.),
        Measure::<Carat>::new(1.).convert::<Kilogram>(),
        MeasurePoint::<Carat>::new(1.),
        MeasurePoint::<Carat>::new(1.).convert::<Kilogram>(),
    );
    println!();
}

fn print_all_mass_density_units() {
    println!("* MassDensity units");
    println!(
        "  KilogramPerCubicMetre: {}, {};",
        Measure::<KilogramPerCubicMetre>::new(1.),
        MeasurePoint::<KilogramPerCubicMetre>::new(1.),
    );
    println!();
}

fn print_all_mass_flow_rate_units() {
    println!("* MassFlowRate units");
    println!(
        "  KilogramPerSecond: {}, {};",
        Measure::<KilogramPerSecond>::new(1.),
        MeasurePoint::<KilogramPerSecond>::new(1.),
    );
    println!(
        "  GramPerSecond: {} == {}, {} == {};",
        Measure::<GramPerSecond>::new(1.),
        Measure::<GramPerSecond>::new(1.).convert::<KilogramPerSecond>(),
        MeasurePoint::<GramPerSecond>::new(1.),
        MeasurePoint::<GramPerSecond>::new(1.).convert::<KilogramPerSecond>(),
    );
    println!();
}

fn print_all_molar_concentration_units() {
    println!("* MolarConcentration units");
    println!(
        "  MolePerCubicMetre: {}, {};",
        Measure::<MolePerCubicMetre>::new(1.),
        MeasurePoint::<MolePerCubicMetre>::new(1.),
    );
    println!();
}

fn print_all_molar_heat_capacity_units() {
    println!("* MolarHeatCapacity units");
    println!(
        "  JoulePerKelvinPerMole: {}, {};",
        Measure::<JoulePerKelvinPerMole>::new(1.),
        MeasurePoint::<JoulePerKelvinPerMole>::new(1.),
    );
    println!();
}

fn print_all_moment_of_inertia_units() {
    println!("* MomentOfInertia units");
    println!(
        "  KilogramSquareMetre: {}, {};",
        Measure::<KilogramSquareMetre>::new(1.),
        MeasurePoint::<KilogramSquareMetre>::new(1.),
    );
    println!(
        "  GramSquareCentimetre: {} == {}, {} == {};",
        Measure::<GramSquareCentimetre>::new(1.),
        Measure::<GramSquareCentimetre>::new(1.).convert::<KilogramSquareMetre>(),
        MeasurePoint::<GramSquareCentimetre>::new(1.),
        MeasurePoint::<GramSquareCentimetre>::new(1.).convert::<KilogramSquareMetre>(),
    );
    println!();
}

fn print_all_momentum_units() {
    println!("* Momentum units");
    println!(
        "  NewtonSecond: {}, {}, {}, {}, {}, {};",
        Measure::<NewtonSecond>::new(1.),
        MeasurePoint::<NewtonSecond>::new(1.),
        Measure2d::<NewtonSecond>::new([1., 2.]),
        MeasurePoint2d::<NewtonSecond>::new([1., 2.]),
        Measure3d::<NewtonSecond>::new([1., 2., 3.]),
        MeasurePoint3d::<NewtonSecond>::new([1., 2., 3.]),
    );
    println!(
        "  KilogramMetrePerSecond: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<KilogramMetrePerSecond>::new(1.),
        Measure::<KilogramMetrePerSecond>::new(1.).convert::<NewtonSecond>(),
        MeasurePoint::<KilogramMetrePerSecond>::new(1.),
        MeasurePoint::<KilogramMetrePerSecond>::new(1.).convert::<NewtonSecond>(),
        Measure2d::<KilogramMetrePerSecond>::new([1., 2.]),
        Measure2d::<KilogramMetrePerSecond>::new([1., 2.]).convert::<NewtonSecond>(),
        MeasurePoint2d::<KilogramMetrePerSecond>::new([1., 2.]),
        MeasurePoint2d::<KilogramMetrePerSecond>::new([1., 2.]).convert::<NewtonSecond>(),
        Measure3d::<KilogramMetrePerSecond>::new([1., 2., 3.]),
        Measure3d::<KilogramMetrePerSecond>::new([1., 2., 3.]).convert::<NewtonSecond>(),
        MeasurePoint3d::<KilogramMetrePerSecond>::new([1., 2., 3.]),
        MeasurePoint3d::<KilogramMetrePerSecond>::new([1., 2., 3.]).convert::<NewtonSecond>(),
    );
    println!(
        "  DynSecond: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<DyneSecond>::new(1.),
        Measure::<DyneSecond>::new(1.).convert::<NewtonSecond>(),
        MeasurePoint::<DyneSecond>::new(1.),
        MeasurePoint::<DyneSecond>::new(1.).convert::<NewtonSecond>(),
        Measure2d::<DyneSecond>::new([1., 2.]),
        Measure2d::<DyneSecond>::new([1., 2.]).convert::<NewtonSecond>(),
        MeasurePoint2d::<DyneSecond>::new([1., 2.]),
        MeasurePoint2d::<DyneSecond>::new([1., 2.]).convert::<NewtonSecond>(),
        Measure3d::<DyneSecond>::new([1., 2., 3.]),
        Measure3d::<DyneSecond>::new([1., 2., 3.]).convert::<NewtonSecond>(),
        MeasurePoint3d::<DyneSecond>::new([1., 2., 3.]),
        MeasurePoint3d::<DyneSecond>::new([1., 2., 3.]).convert::<NewtonSecond>(),
    );
    println!(
        "  GramCentimetrePerSecond: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<GramCentimetrePerSecond>::new(1.),
        Measure::<GramCentimetrePerSecond>::new(1.).convert::<NewtonSecond>(),
        MeasurePoint::<GramCentimetrePerSecond>::new(1.),
        MeasurePoint::<GramCentimetrePerSecond>::new(1.).convert::<NewtonSecond>(),
        Measure2d::<GramCentimetrePerSecond>::new([1., 2.]),
        Measure2d::<GramCentimetrePerSecond>::new([1., 2.]).convert::<NewtonSecond>(),
        MeasurePoint2d::<GramCentimetrePerSecond>::new([1., 2.]),
        MeasurePoint2d::<GramCentimetrePerSecond>::new([1., 2.]).convert::<NewtonSecond>(),
        Measure3d::<GramCentimetrePerSecond>::new([1., 2., 3.]),
        Measure3d::<GramCentimetrePerSecond>::new([1., 2., 3.]).convert::<NewtonSecond>(),
        MeasurePoint3d::<GramCentimetrePerSecond>::new([1., 2., 3.]),
        MeasurePoint3d::<GramCentimetrePerSecond>::new([1., 2., 3.]).convert::<NewtonSecond>(),
    );
    println!();
}

fn print_all_permittivity_units() {
    println!("* Permittivity units");
    println!(
        "  FaradPerMetre: {}, {};",
        Measure::<FaradPerMetre>::new(1.),
        MeasurePoint::<FaradPerMetre>::new(1.),
    );
    println!();
}

fn print_all_power_units() {
    println!("* Power units");
    println!(
        "  Watt: {}, {};",
        Measure::<Watt>::new(1.),
        MeasurePoint::<Watt>::new(1.),
    );
    println!(
        "  Milliwatt: {} == {}, {} == {};",
        Measure::<Milliwatt>::new(1.),
        Measure::<Milliwatt>::new(1.).convert::<Watt>(),
        MeasurePoint::<Milliwatt>::new(1.),
        MeasurePoint::<Milliwatt>::new(1.).convert::<Watt>(),
    );
    println!(
        "  Kilowatt: {} == {}, {} == {};",
        Measure::<Kilowatt>::new(1.),
        Measure::<Kilowatt>::new(1.).convert::<Watt>(),
        MeasurePoint::<Kilowatt>::new(1.),
        MeasurePoint::<Kilowatt>::new(1.).convert::<Watt>(),
    );
    println!(
        "  Megawatt: {} == {}, {} == {};",
        Measure::<Megawatt>::new(1.),
        Measure::<Megawatt>::new(1.).convert::<Watt>(),
        MeasurePoint::<Megawatt>::new(1.),
        MeasurePoint::<Megawatt>::new(1.).convert::<Watt>(),
    );
    println!(
        "  Gigawatt: {} == {}, {} == {};",
        Measure::<Gigawatt>::new(1.),
        Measure::<Gigawatt>::new(1.).convert::<Watt>(),
        MeasurePoint::<Gigawatt>::new(1.),
        MeasurePoint::<Gigawatt>::new(1.).convert::<Watt>(),
    );
    println!(
        "  HorsePower: {} == {}, {} == {};",
        Measure::<HorsePower>::new(1.),
        Measure::<HorsePower>::new(1.).convert::<Watt>(),
        MeasurePoint::<HorsePower>::new(1.),
        MeasurePoint::<HorsePower>::new(1.).convert::<Watt>(),
    );
    println!();
}

fn print_all_pressure_units() {
    println!("* Pressure units");
    println!(
        "  Pascal: {}, {};",
        Measure::<Pascal>::new(1.),
        MeasurePoint::<Pascal>::new(1.),
    );
    println!(
        "  Hectopascal: {} == {}, {} == {};",
        Measure::<Hectopascal>::new(1.),
        Measure::<Hectopascal>::new(1.).convert::<Pascal>(),
        MeasurePoint::<Hectopascal>::new(1.),
        MeasurePoint::<Hectopascal>::new(1.).convert::<Pascal>(),
    );
    println!(
        "  Atmosphere: {} == {}, {} == {};",
        Measure::<Atmosphere>::new(1.),
        Measure::<Atmosphere>::new(1.).convert::<Pascal>(),
        MeasurePoint::<Atmosphere>::new(1.),
        MeasurePoint::<Atmosphere>::new(1.).convert::<Pascal>(),
    );
    println!(
        "  Bar: {} == {}, {} == {};",
        Measure::<Bar>::new(1.),
        Measure::<Bar>::new(1.).convert::<Pascal>(),
        MeasurePoint::<Bar>::new(1.),
        MeasurePoint::<Bar>::new(1.).convert::<Pascal>(),
    );
    println!(
        "  Millibar: {} == {}, {} == {};",
        Measure::<Millibar>::new(1.),
        Measure::<Millibar>::new(1.).convert::<Pascal>(),
        MeasurePoint::<Millibar>::new(1.),
        MeasurePoint::<Millibar>::new(1.).convert::<Pascal>(),
    );
    println!(
        "  MmHg: {} == {}, {} == {};",
        Measure::<MmHg>::new(1.),
        Measure::<MmHg>::new(1.).convert::<Pascal>(),
        MeasurePoint::<MmHg>::new(1.),
        MeasurePoint::<MmHg>::new(1.).convert::<Pascal>(),
    );
    println!(
        "  PoundForcePerSquareInch: {} == {}, {} == {};",
        Measure::<PoundForcePerSquareInch>::new(1.),
        Measure::<PoundForcePerSquareInch>::new(1.).convert::<Pascal>(),
        MeasurePoint::<PoundForcePerSquareInch>::new(1.),
        MeasurePoint::<PoundForcePerSquareInch>::new(1.).convert::<Pascal>(),
    );
    println!();
}

fn print_all_radiance_units() {
    println!("* Radiance units");
    println!(
        "  WattPerSquareMetrePerSteradian: {}, {};",
        Measure::<WattPerSquareMetrePerSteradian>::new(1.),
        MeasurePoint::<WattPerSquareMetrePerSteradian>::new(1.),
    );
    println!();
}

fn print_all_radiant_intensity_units() {
    println!("* RadiantIntensity units");
    println!(
        "  WattPerSteradian: {}, {};",
        Measure::<WattPerSteradian>::new(1.),
        MeasurePoint::<WattPerSteradian>::new(1.),
    );
    println!();
}

fn print_all_radioactive_activity_units() {
    println!("* RadioactiveActivity units");
    println!(
        "  Becquerel: {}, {};",
        Measure::<Becquerel>::new(1.),
        MeasurePoint::<Becquerel>::new(1.),
    );
    println!(
        "  Kilobecquerel: {} == {}, {} == {};",
        Measure::<Kilobecquerel>::new(1.),
        Measure::<Kilobecquerel>::new(1.).convert::<Becquerel>(),
        MeasurePoint::<Kilobecquerel>::new(1.),
        MeasurePoint::<Kilobecquerel>::new(1.).convert::<Becquerel>(),
    );
    println!(
        "  Megabecquerel: {} == {}, {} == {};",
        Measure::<Megabecquerel>::new(1.),
        Measure::<Megabecquerel>::new(1.).convert::<Becquerel>(),
        MeasurePoint::<Megabecquerel>::new(1.),
        MeasurePoint::<Megabecquerel>::new(1.).convert::<Becquerel>(),
    );
    println!(
        "  Gigabecquerel: {} == {}, {} == {};",
        Measure::<Gigabecquerel>::new(1.),
        Measure::<Gigabecquerel>::new(1.).convert::<Becquerel>(),
        MeasurePoint::<Gigabecquerel>::new(1.),
        MeasurePoint::<Gigabecquerel>::new(1.).convert::<Becquerel>(),
    );
    println!();
}

fn print_all_radioactive_dose_units() {
    println!("* RadioactiveDose units");
    println!(
        "  Gray: {}, {};",
        Measure::<Gray>::new(1.),
        MeasurePoint::<Gray>::new(1.),
    );
    println!(
        "  Rad: {} == {}, {} == {};",
        Measure::<Rad>::new(1.),
        Measure::<Rad>::new(1.).convert::<Gray>(),
        MeasurePoint::<Rad>::new(1.),
        MeasurePoint::<Rad>::new(1.).convert::<Gray>(),
    );
    println!();
}

fn print_all_radioactive_dose_rate_units() {
    println!("* RadioactiveDoseRate units");
    println!(
        "  GrayPerSecond: {}, {};",
        Measure::<GrayPerSecond>::new(1.),
        MeasurePoint::<GrayPerSecond>::new(1.),
    );
    println!();
}

fn print_all_reaction_rate_units() {
    println!("* ReactionRate units");
    println!(
        "  MolePerCubicMetrePerSecond: {}, {};",
        Measure::<MolePerCubicMetrePerSecond>::new(1.),
        MeasurePoint::<MolePerCubicMetrePerSecond>::new(1.),
    );
    println!();
}

fn print_all_solid_angle_units() {
    println!("* SolidAngle units");
    println!(
        "  Steradian: {}, {};",
        Measure::<Steradian>::new(1.),
        MeasurePoint::<Steradian>::new(1.),
    );
    println!(
        "  Spat: {} == {}, {} == {};",
        Measure::<Spat>::new(1.),
        Measure::<Spat>::new(1.).convert::<Steradian>(),
        MeasurePoint::<Spat>::new(1.),
        MeasurePoint::<Spat>::new(1.).convert::<Steradian>(),
    );
    println!(
        "  SquareDegree: {} == {}, {} == {};",
        Measure::<SquareDegree>::new(1.),
        Measure::<SquareDegree>::new(1.).convert::<Steradian>(),
        MeasurePoint::<SquareDegree>::new(1.),
        MeasurePoint::<SquareDegree>::new(1.).convert::<Steradian>(),
    );
    println!();
}

fn print_all_specific_energy_units() {
    println!("* SpecificEnergy units");
    println!(
        "  JoulePerKilogram: {}, {};",
        Measure::<JoulePerKilogram>::new(1.),
        MeasurePoint::<JoulePerKilogram>::new(1.),
    );
    println!();
}

fn print_all_specific_heat_capacity_units() {
    println!("* SpecificHeatCapacity units");
    println!(
        "  JoulePerKilogramPerKelvin: {}, {};",
        Measure::<JoulePerKilogramPerKelvin>::new(1.),
        MeasurePoint::<JoulePerKilogramPerKelvin>::new(1.),
    );
    println!();
}

fn print_all_specific_volume_units() {
    println!("* SpecificVolume units");
    println!(
        "  CubicMetrePerKilogram: {}, {};",
        Measure::<CubicMetrePerKilogram>::new(1.),
        MeasurePoint::<CubicMetrePerKilogram>::new(1.),
    );
    println!();
}

fn print_all_square_time_units() {
    println!("* SquareTime units");
    println!(
        "  SquareSecond: {}, {};",
        Measure::<SquareSecond>::new(1.),
        MeasurePoint::<SquareSecond>::new(1.),
    );
    println!(
        "  HourSecond: {} == {}, {} == {};",
        Measure::<HourSecond>::new(1.),
        Measure::<HourSecond>::new(1.).convert::<SquareSecond>(),
        MeasurePoint::<HourSecond>::new(1.),
        MeasurePoint::<HourSecond>::new(1.).convert::<SquareSecond>(),
    );
    println!(
        "  HourHour: {} == {}, {} == {};",
        Measure::<HourHour>::new(1.),
        Measure::<HourHour>::new(1.).convert::<SquareSecond>(),
        MeasurePoint::<HourHour>::new(1.),
        MeasurePoint::<HourHour>::new(1.).convert::<SquareSecond>(),
    );
    println!();
}

fn print_all_surface_density_units() {
    println!("* SurfaceDensity units");
    println!(
        "  KilogramPerSquareMetre: {}, {};",
        Measure::<KilogramPerSquareMetre>::new(1.),
        MeasurePoint::<KilogramPerSquareMetre>::new(1.),
    );
    println!();
}

fn print_all_surface_tension_units() {
    println!("* SurfaceTension units");
    println!(
        "  JoulePerSquareMetre: {}, {};",
        Measure::<JoulePerSquareMetre>::new(1.),
        MeasurePoint::<JoulePerSquareMetre>::new(1.),
    );
    println!();
}

fn print_all_temperature_units() {
    println!("* Temperature units");
    println!(
        "  Kelvin: {}, {};",
        Measure::<Kelvin>::new(1.),
        MeasurePoint::<Kelvin>::new(1.),
    );
    println!(
        "  Celsius: {} == {}, {} == {};",
        Measure::<Celsius>::new(1.),
        Measure::<Celsius>::new(1.).convert::<Kelvin>(),
        MeasurePoint::<Celsius>::new(1.),
        MeasurePoint::<Celsius>::new(1.).convert::<Kelvin>(),
    );
    println!(
        "  Fahrenheit: {} == {}, {} == {};",
        Measure::<Fahrenheit>::new(1.),
        Measure::<Fahrenheit>::new(1.).convert::<Kelvin>(),
        MeasurePoint::<Fahrenheit>::new(1.),
        MeasurePoint::<Fahrenheit>::new(1.).convert::<Kelvin>(),
    );
    println!();
}

fn print_all_thermal_conductivity_units() {
    println!("* ThermalConductivity units");
    println!(
        "  WattPerMetrePerKelvin: {}, {};",
        Measure::<WattPerMetrePerKelvin>::new(1.),
        MeasurePoint::<WattPerMetrePerKelvin>::new(1.),
    );
    println!();
}

fn print_all_time_units() {
    println!("* Time units");
    println!(
        "  Second: {}, {};",
        Measure::<Second>::new(1.),
        MeasurePoint::<Second>::new(1.),
    );
    println!(
        "  Year: {} == {}, {} == {};",
        Measure::<Year>::new(1.),
        Measure::<Year>::new(1.).convert::<Second>(),
        MeasurePoint::<Year>::new(1.),
        MeasurePoint::<Year>::new(1.).convert::<Second>(),
    );
    println!(
        "  Week: {} == {}, {} == {};",
        Measure::<Week>::new(1.),
        Measure::<Week>::new(1.).convert::<Second>(),
        MeasurePoint::<Week>::new(1.),
        MeasurePoint::<Week>::new(1.).convert::<Second>(),
    );
    println!(
        "  Day: {} == {}, {} == {};",
        Measure::<Day>::new(1.),
        Measure::<Day>::new(1.).convert::<Second>(),
        MeasurePoint::<Day>::new(1.),
        MeasurePoint::<Day>::new(1.).convert::<Second>(),
    );
    println!(
        "  Hour: {} == {}, {} == {};",
        Measure::<Hour>::new(1.),
        Measure::<Hour>::new(1.).convert::<Second>(),
        MeasurePoint::<Hour>::new(1.),
        MeasurePoint::<Hour>::new(1.).convert::<Second>(),
    );
    println!(
        "  Minute: {} == {}, {} == {};",
        Measure::<Minute>::new(1.),
        Measure::<Minute>::new(1.).convert::<Second>(),
        MeasurePoint::<Minute>::new(1.),
        MeasurePoint::<Minute>::new(1.).convert::<Second>(),
    );
    println!(
        "  Millisecond: {} == {}, {} == {};",
        Measure::<Millisecond>::new(1.),
        Measure::<Millisecond>::new(1.).convert::<Second>(),
        MeasurePoint::<Millisecond>::new(1.),
        MeasurePoint::<Millisecond>::new(1.).convert::<Second>(),
    );
    println!(
        "  Microsecond: {} == {}, {} == {};",
        Measure::<Microsecond>::new(1.),
        Measure::<Microsecond>::new(1.).convert::<Second>(),
        MeasurePoint::<Microsecond>::new(1.),
        MeasurePoint::<Microsecond>::new(1.).convert::<Second>(),
    );
    println!(
        "  Nanosecond: {} == {}, {} == {};",
        Measure::<Nanosecond>::new(1.),
        Measure::<Nanosecond>::new(1.).convert::<Second>(),
        MeasurePoint::<Nanosecond>::new(1.),
        MeasurePoint::<Nanosecond>::new(1.).convert::<Second>(),
    );
    println!(
        "  Picosecond: {} == {}, {} == {};",
        Measure::<Picosecond>::new(1.),
        Measure::<Picosecond>::new(1.).convert::<Second>(),
        MeasurePoint::<Picosecond>::new(1.),
        MeasurePoint::<Picosecond>::new(1.).convert::<Second>(),
    );
    println!(
        "  Femtosecond: {} == {}, {} == {};",
        Measure::<Femtosecond>::new(1.),
        Measure::<Femtosecond>::new(1.).convert::<Second>(),
        MeasurePoint::<Femtosecond>::new(1.),
        MeasurePoint::<Femtosecond>::new(1.).convert::<Second>(),
    );
    println!();
}

fn print_all_torque_units() {
    println!("* Torque units");
    println!(
        "  NewtonMetre: {}, {}, {}, {}, {}, {};",
        Measure::<NewtonMetre>::new(1.),
        MeasurePoint::<NewtonMetre>::new(1.),
        Measure2d::<NewtonMetre>::new([1., 2.]),
        MeasurePoint2d::<NewtonMetre>::new([1., 2.]),
        Measure3d::<NewtonMetre>::new([1., 2., 3.]),
        MeasurePoint3d::<NewtonMetre>::new([1., 2., 3.]),
    );
    println!();
}

fn print_all_velocity_units() {
    println!("* Velocity units");
    println!(
        "  MetrePerSecond: {}, {}, {}, {}, {}, {};",
        Measure::<MetrePerSecond>::new(1.),
        MeasurePoint::<MetrePerSecond>::new(1.),
        Measure2d::<MetrePerSecond>::new([1., 2.]),
        MeasurePoint2d::<MetrePerSecond>::new([1., 2.]),
        Measure3d::<MetrePerSecond>::new([1., 2., 3.]),
        MeasurePoint3d::<MetrePerSecond>::new([1., 2., 3.]),
    );
    println!(
        "  Knot: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<Knot>::new(1.),
        Measure::<Knot>::new(1.).convert::<MetrePerSecond>(),
        MeasurePoint::<Knot>::new(1.),
        MeasurePoint::<Knot>::new(1.).convert::<MetrePerSecond>(),
        Measure2d::<Knot>::new([1., 2.]),
        Measure2d::<Knot>::new([1., 2.]).convert::<MetrePerSecond>(),
        MeasurePoint2d::<Knot>::new([1., 2.]),
        MeasurePoint2d::<Knot>::new([1., 2.]).convert::<MetrePerSecond>(),
        Measure3d::<Knot>::new([1., 2., 3.]),
        Measure3d::<Knot>::new([1., 2., 3.]).convert::<MetrePerSecond>(),
        MeasurePoint3d::<Knot>::new([1., 2., 3.]),
        MeasurePoint3d::<Knot>::new([1., 2., 3.]).convert::<MetrePerSecond>(),
    );
    println!(
        "  KilometrePerHour: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<KilometrePerHour>::new(1.),
        Measure::<KilometrePerHour>::new(1.).convert::<MetrePerSecond>(),
        MeasurePoint::<KilometrePerHour>::new(1.),
        MeasurePoint::<KilometrePerHour>::new(1.).convert::<MetrePerSecond>(),
        Measure2d::<KilometrePerHour>::new([1., 2.]),
        Measure2d::<KilometrePerHour>::new([1., 2.]).convert::<MetrePerSecond>(),
        MeasurePoint2d::<KilometrePerHour>::new([1., 2.]),
        MeasurePoint2d::<KilometrePerHour>::new([1., 2.]).convert::<MetrePerSecond>(),
        Measure3d::<KilometrePerHour>::new([1., 2., 3.]),
        Measure3d::<KilometrePerHour>::new([1., 2., 3.]).convert::<MetrePerSecond>(),
        MeasurePoint3d::<KilometrePerHour>::new([1., 2., 3.]),
        MeasurePoint3d::<KilometrePerHour>::new([1., 2., 3.]).convert::<MetrePerSecond>(),
    );
    println!(
        "  MilePerHour: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<MilePerHour>::new(1.),
        Measure::<MilePerHour>::new(1.).convert::<MetrePerSecond>(),
        MeasurePoint::<MilePerHour>::new(1.),
        MeasurePoint::<MilePerHour>::new(1.).convert::<MetrePerSecond>(),
        Measure2d::<MilePerHour>::new([1., 2.]),
        Measure2d::<MilePerHour>::new([1., 2.]).convert::<MetrePerSecond>(),
        MeasurePoint2d::<MilePerHour>::new([1., 2.]),
        MeasurePoint2d::<MilePerHour>::new([1., 2.]).convert::<MetrePerSecond>(),
        Measure3d::<MilePerHour>::new([1., 2., 3.]),
        Measure3d::<MilePerHour>::new([1., 2., 3.]).convert::<MetrePerSecond>(),
        MeasurePoint3d::<MilePerHour>::new([1., 2., 3.]),
        MeasurePoint3d::<MilePerHour>::new([1., 2., 3.]).convert::<MetrePerSecond>(),
    );
    println!(
        "  CentimetrePerSecond: {} == {}, {} == {}, {} == {}, {} == {}, {} == {}, {} == {};",
        Measure::<CentimetrePerSecond>::new(1.),
        Measure::<CentimetrePerSecond>::new(1.).convert::<MetrePerSecond>(),
        MeasurePoint::<CentimetrePerSecond>::new(1.),
        MeasurePoint::<CentimetrePerSecond>::new(1.).convert::<MetrePerSecond>(),
        Measure2d::<CentimetrePerSecond>::new([1., 2.]),
        Measure2d::<CentimetrePerSecond>::new([1., 2.]).convert::<MetrePerSecond>(),
        MeasurePoint2d::<CentimetrePerSecond>::new([1., 2.]),
        MeasurePoint2d::<CentimetrePerSecond>::new([1., 2.]).convert::<MetrePerSecond>(),
        Measure3d::<CentimetrePerSecond>::new([1., 2., 3.]),
        Measure3d::<CentimetrePerSecond>::new([1., 2., 3.]).convert::<MetrePerSecond>(),
        MeasurePoint3d::<CentimetrePerSecond>::new([1., 2., 3.]),
        MeasurePoint3d::<CentimetrePerSecond>::new([1., 2., 3.]).convert::<MetrePerSecond>(),
    );
    println!();
}

fn print_all_volume_units() {
    println!("* Volume units");
    println!(
        "  CubicMetre: {}, {};",
        Measure::<CubicMetre>::new(1.),
        MeasurePoint::<CubicMetre>::new(1.),
    );
    println!(
        "  CubicKilometre: {} == {}, {} == {};",
        Measure::<CubicKilometre>::new(1.),
        Measure::<CubicKilometre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<CubicKilometre>::new(1.),
        MeasurePoint::<CubicKilometre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  CubicInch: {} == {}, {} == {};",
        Measure::<CubicInch>::new(1.),
        Measure::<CubicInch>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<CubicInch>::new(1.),
        MeasurePoint::<CubicInch>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  CubicFoot: {} == {}, {} == {};",
        Measure::<CubicFoot>::new(1.),
        Measure::<CubicFoot>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<CubicFoot>::new(1.),
        MeasurePoint::<CubicFoot>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  CubicYard: {} == {}, {} == {};",
        Measure::<CubicYard>::new(1.),
        Measure::<CubicYard>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<CubicYard>::new(1.),
        MeasurePoint::<CubicYard>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  CubicMile: {} == {}, {} == {};",
        Measure::<CubicMile>::new(1.),
        Measure::<CubicMile>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<CubicMile>::new(1.),
        MeasurePoint::<CubicMile>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  Litre: {} == {}, {} == {};",
        Measure::<Litre>::new(1.),
        Measure::<Litre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<Litre>::new(1.),
        MeasurePoint::<Litre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  Millilitre: {} == {}, {} == {};",
        Measure::<Millilitre>::new(1.),
        Measure::<Millilitre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<Millilitre>::new(1.),
        MeasurePoint::<Millilitre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  Microlitre: {} == {}, {} == {};",
        Measure::<Microlitre>::new(1.),
        Measure::<Microlitre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<Microlitre>::new(1.),
        MeasurePoint::<Microlitre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  Nanolitre: {} == {}, {} == {};",
        Measure::<Nanolitre>::new(1.),
        Measure::<Nanolitre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<Nanolitre>::new(1.),
        MeasurePoint::<Nanolitre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  Picolitre: {} == {}, {} == {};",
        Measure::<Picolitre>::new(1.),
        Measure::<Picolitre>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<Picolitre>::new(1.),
        MeasurePoint::<Picolitre>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  Pint: {} == {}, {} == {};",
        Measure::<Pint>::new(1.),
        Measure::<Pint>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<Pint>::new(1.),
        MeasurePoint::<Pint>::new(1.).convert::<CubicMetre>(),
    );
    println!(
        "  Gallon: {} == {}, {} == {};",
        Measure::<Gallon>::new(1.),
        Measure::<Gallon>::new(1.).convert::<CubicMetre>(),
        MeasurePoint::<Gallon>::new(1.),
        MeasurePoint::<Gallon>::new(1.).convert::<CubicMetre>(),
    );
    println!();
}

fn print_all_volumetric_flow_rate_units() {
    println!("* VolumetricFlowRate units");
    println!(
        "  CubicMetrePerSecond: {}, {};",
        Measure::<CubicMetrePerSecond>::new(1.),
        MeasurePoint::<CubicMetrePerSecond>::new(1.),
    );
    println!(
        "  CubicCentimetrePerSecond: {} == {}, {} == {};",
        Measure::<MillilitrePerSecond>::new(1.),
        Measure::<MillilitrePerSecond>::new(1.).convert::<CubicMetrePerSecond>(),
        MeasurePoint::<MillilitrePerSecond>::new(1.),
        MeasurePoint::<MillilitrePerSecond>::new(1.).convert::<CubicMetrePerSecond>(),
    );
    println!();
}

fn print_all_wave_number_units() {
    println!("* WaveNumber units");
    println!(
        "  CyclePerMetre: {}, {};",
        Measure::<CyclePerMetre>::new(1.),
        MeasurePoint::<CyclePerMetre>::new(1.),
    );
    println!(
        "  RadianPerMetre: {} == {}, {} == {};",
        Measure::<RadianPerMetre>::new(1.),
        Measure::<RadianPerMetre>::new(1.).convert::<CyclePerMetre>(),
        MeasurePoint::<RadianPerMetre>::new(1.),
        MeasurePoint::<RadianPerMetre>::new(1.).convert::<CyclePerMetre>(),
    );
    println!();
}

fn print_all_units() {
    print_all_acceleration_units();
    print_all_action_units();
    print_all_amount_units();
    print_all_angle_units();
    print_all_angular_acceleration_units();
    print_all_angular_momentum_units();
    print_all_area_units();
    print_all_area_density_units();
    print_all_capacitance_units();
    print_all_catalytic_activity_units();
    print_all_chemical_potential_units();
    print_all_current_density_units();
    print_all_dimensionless_units();
    print_all_dose_equivalent_units();
    print_all_dynamic_viscosity_units();
    print_all_electrical_conductance_units();
    print_all_electrical_conductivity_units();
    print_all_electrical_resistance_units();
    print_all_electrical_resistivity_units();
    print_all_electric_charge_units();
    print_all_electric_charge_density_units();
    print_all_electric_current_units();
    print_all_electric_displacement_units();
    print_all_electric_field_strength_units();
    print_all_electric_potential_units();
    print_all_energy_units();
    print_all_energy_density_units();
    print_all_entropy_units();
    print_all_force_units();
    print_all_frequency_units();
    print_all_illuminance_units();
    print_all_inductance_units();
    print_all_information_units();
    print_all_information_rate_units();
    print_all_irradiance_units();
    print_all_kinematic_viscosity_units();
    print_all_length_units();
    print_all_linear_density_units();
    print_all_linear_electric_charge_density_units();
    print_all_luminance_units();
    print_all_luminous_flux_units();
    print_all_luminous_intensity_units();
    print_all_magnetic_field_strength_units();
    print_all_magnetic_flux_units();
    print_all_magnetic_flux_density_units();
    print_all_magnetic_permeability_units();
    print_all_magnetic_reluctance_units();
    print_all_mass_units();
    print_all_mass_density_units();
    print_all_mass_flow_rate_units();
    print_all_molar_concentration_units();
    print_all_molar_heat_capacity_units();
    print_all_moment_of_inertia_units();
    print_all_momentum_units();
    print_all_permittivity_units();
    print_all_power_units();
    print_all_pressure_units();
    print_all_radiance_units();
    print_all_radiant_intensity_units();
    print_all_radioactive_activity_units();
    print_all_radioactive_dose_units();
    print_all_radioactive_dose_rate_units();
    print_all_reaction_rate_units();
    print_all_solid_angle_units();
    print_all_specific_energy_units();
    print_all_specific_heat_capacity_units();
    print_all_specific_volume_units();
    print_all_square_time_units();
    print_all_surface_density_units();
    print_all_surface_tension_units();
    print_all_temperature_units();
    print_all_thermal_conductivity_units();
    print_all_time_units();
    print_all_torque_units();
    print_all_velocity_units();
    print_all_volume_units();
    print_all_volumetric_flow_rate_units();
    print_all_wave_number_units();
}

fn print_all_single_unit_operations_for_measure_1d() {
    println!("* Single unit operations for measure 1d");
    {
        let m1 = Measure::<KilometrePerHour>::new(12.);
        println!(
            "{m1} can be converted to {m2}.",
            m2 = m1.convert::<MilePerHour>()
        );
    }
    {
        let m1 = Measure::<KilometrePerHour>::new(1.234_567_890_123_456_7);
        println!(
            "{m1} can be lossy-converted to {m2}.",
            m2 = m1.lossy_into::<f32>()
        );
    }
    {
        let m1 = Measure::<KilometrePerHour, f32>::new(1.234_567_9);
        let m2: Measure<KilometrePerHour, f64> = m1.into();
        println!("{m1} can be lossless-converted to {m2}.");
    }
    {
        let m1 = Measure::<KilometrePerHour>::new(-12.);
        println!("The squared norm of {m1} is {n}.", n = m1.squared_norm(),);
    }
    {
        let m1 = Measure::<KilometrePerHour>::new(-12.);
        println!("{m1} normalized is {n}.", n = m1.normalized());
    }
    {
        let m1 = Measure::<KilometrePerHour>::new(12.);
        println!("The opposite of {m1} is {m2}.", m2 = -m1);
    }
    {
        let mut m1 = Measure::<KilometrePerHour>::new(12.);
        let m2 = Measure::<KilometrePerHour>::new(13.);
        print!("{m1} plus {m2} is {m3},", m3 = m1 + m2);

        m1 += m2;
        println!(" and if incremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure::<KilometrePerHour>::new(12.);
        let m2 = Measure::<KilometrePerHour>::new(13.);
        print!("{m1} minus {m2} is {m3},", m3 = m1 - m2);

        m1 -= m2;
        println!(" and if decremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure::<KilometrePerHour>::new(12.);
        let multiplier = 2.;
        print!("{m1} times {multiplier} is {m2},", m2 = m1 * multiplier);

        m1 *= multiplier;
        println!(" and if multiplied by {multiplier}, it becomes {m1}.");
    }
    {
        let m1 = Measure::<KilometrePerHour>::new(12.);
        let multiplier = 2.;
        println!("{multiplier} times {m1} is {m2}.", m2 = multiplier * m1);
    }
    {
        let mut m1 = Measure::<KilometrePerHour>::new(12.);
        let divisor = 2.;
        print!("{m1} divided by {divisor} is {m2},", m2 = m1 / divisor);

        m1 /= divisor;
        println!(" and if divided by {divisor}, it becomes {m1}.");
    }
    {
        let m1 = Measure::<KilometrePerHour>::new(12.);
        let m2 = Measure::<KilometrePerHour>::new(4.);
        println!("{m1} divided by {m2} is {m3}.", m3 = m1 / m2);
    }
    {
        let m1 = Measure::<KilometrePerHour>::new(12.);
        let m2 = m1;
        println!("{m1} == {m1} is {result}.", result = m1 == m2);
        println!("{m1} < {m1} is {result}.", result = m1 < m2);
    }
    println!();
}

fn print_all_single_unit_operations_for_measure_point_1d() {
    println!("* Single unit operations for measure point 1d");
    {
        let mp1 = MeasurePoint::<Celsius>::new(12.);
        println!(
            "{mp1} can be converted to {mp2}.",
            mp2 = mp1.convert::<Fahrenheit>()
        );
    }
    {
        let mp1 = MeasurePoint::<Celsius>::new(1.234_567_890_123_456_7);
        println!(
            "{mp1} can be lossy-converted to {mp2}.",
            mp2 = mp1.lossy_into::<f32>()
        );
    }
    {
        let mp1 = MeasurePoint::<Celsius, f32>::new(1.234_567_9);
        let mp2: MeasurePoint<Celsius, f64> = mp1.into();
        println!("{mp1} can be lossless-converted to {mp2}.",);
    }
    {
        let mut mp1 = MeasurePoint::<Celsius>::new(12.);
        let m2 = Measure::<Celsius>::new(13.);
        print!("{mp1} plus {m2} is {mp3},", mp3 = mp1 + m2);

        mp1 += m2;
        println!(" and if incremented by {m2}, it becomes {mp1}.");
    }
    {
        let mut mp1 = MeasurePoint::<Celsius>::new(12.);
        let m2 = Measure::<Celsius>::new(13.);
        print!("{mp1} minus {m2} is {mp3},", mp3 = mp1 - m2);

        mp1 -= m2;
        println!(" and if decremented by {m2}, it becomes {mp1}.");
    }
    {
        let mp1 = MeasurePoint::<Celsius>::new(12.);
        let mp2 = MeasurePoint::<Celsius>::new(13.);
        println!("{mp1} minus {mp2} is {m3}.", m3 = mp1 - mp2);
    }
    {
        let mp1 = MeasurePoint::<Celsius>::new(10.);
        let mp2 = MeasurePoint::<Celsius>::new(20.);
        println!("The weighted midpoint between {mp1} (with weight 40%) and {mp2} (with weight 60%) is {mp3}.", mp3 = weighted_midpoint(mp1, mp2, 0.4));
        println!(
            "The midpoint between {mp1} and {mp2} is {mp3}.",
            mp3 = midpoint(mp1, mp2)
        );
    }
    {
        let mp1 = MeasurePoint::<Celsius>::new(10.);
        let mp2 = MeasurePoint::<Celsius>::new(20.);
        let mp3 = MeasurePoint::<Celsius>::new(40.);
        println!("The barycentric combination among {mp1} (with weight 10%), {mp2} (with weight 20%), and {mp3} (with weight 70%) is {mp4}.", mp4 = barycentric_combination(&[mp1, mp2, mp3], &[0.1, 0.2, 0.7]));
    }
    {
        let mp1 = MeasurePoint::<Celsius>::new(12.);
        let mp2 = mp1;
        println!("{mp1} == {mp1} is {result}.", result = mp1 == mp2);
        println!("{mp1} < {mp1} is {result}.", result = mp1 < mp2);
    }
    println!();
}

fn print_all_single_unit_operations_for_unsigned_directions() {
    println!("* Single unit operations for unsigned directions");
    {
        let mp1 = MeasurePoint::<Degree>::new(12.);
        println!(
            "{ud2} can be created from {mp1}.",
            ud2 = UnsignedDirection::<Degree>::from_measure_point(mp1)
        );
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(12.);
        println!(
            "{ud1} can be converted to {mp2}.",
            mp2 = ud1.to_measure_point()
        );
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(12.);
        println!(
            "{ud1} can be converted to {sd2}.",
            sd2 = ud1.to_signed_direction()
        );
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(12.);
        println!(
            "{ud1} can be converted to {ud2}.",
            ud2 = ud1.convert::<Radian>()
        );
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(1.234_567_890_123_456_7);
        println!(
            "{ud1} can be lossy-converted to {ud2}.",
            ud2 = ud1.lossy_into::<f32>()
        );
    }
    {
        let ud1 = UnsignedDirection::<Degree, f32>::new(1.234_567_9);
        let ud2: UnsignedDirection<Degree, f64> = ud1.into();
        println!("{ud1} can be lossless-converted to {ud2}.");
    }
    {
        let mut ud1 = UnsignedDirection::<Degree>::new(12.);
        let m2 = Measure::<Degree>::new(13.);
        print!("{ud1} plus {m2} is {ud3},", ud3 = ud1 + m2);

        ud1 += m2;
        println!(" and if incremented by {m2}, it becomes {ud1}.");
    }
    {
        let mut ud1 = UnsignedDirection::<Degree>::new(12.);
        let m2 = Measure::<Degree>::new(13.);
        print!("{ud1} minus {m2} is {ud3},", ud3 = ud1 - m2);

        ud1 -= m2;
        println!(" and if decremented by {m2}, it becomes {ud1}.");
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(12.);
        let ud2 = UnsignedDirection::<Degree>::new(13.);
        println!("{ud1} minus {ud2} is {m3}.", m3 = ud1 - ud2);
    }
    {
        let ud1 = UnsignedDirection::<Degree>::new(12.);
        let ud2 = ud1;
        println!("{ud1} == {ud1} is {result}.", result = ud1 == ud2);
        println!("{ud1} < {ud1} is {result}.", result = ud1 < ud2);
    }
    println!();
}

fn print_all_single_unit_operations_for_signed_directions() {
    println!("* Single unit operations for signed directions");
    {
        let mp1 = MeasurePoint::<Degree>::new(12.);
        println!(
            "{sd2} can be created from {mp1}.",
            sd2 = SignedDirection::<Degree>::from_measure_point(mp1)
        );
    }
    {
        let sd1 = SignedDirection::<Degree>::new(12.);
        println!(
            "{sd1} can be converted to {mp2}.",
            mp2 = sd1.to_measure_point()
        );
    }
    {
        let sd1 = SignedDirection::<Degree>::new(12.);
        println!(
            "{sd1} can be converted to {ud2}.",
            ud2 = sd1.to_unsigned_direction()
        );
    }
    {
        let sd1 = SignedDirection::<Degree>::new(12.);
        println!(
            "{sd1} can be converted to {sd2}.",
            sd2 = sd1.convert::<Radian>()
        );
    }
    {
        let sd1 = SignedDirection::<Degree>::new(1.234_567_890_123_456_7);
        println!(
            "{sd1} can be lossy-converted to {sd2}.",
            sd2 = sd1.lossy_into::<f32>()
        );
    }
    {
        let sd1 = SignedDirection::<Degree, f32>::new(1.234_567_9);
        let sd2: SignedDirection<Degree, f64> = sd1.into();
        println!("{sd1} can be lossless-converted to {sd2}.",);
    }
    {
        let mut sd1 = SignedDirection::<Degree>::new(12.);
        let m2 = Measure::<Degree>::new(13.);
        print!("{sd1} plus {m2} is {sd3},", sd3 = sd1 + m2);

        sd1 += m2;
        println!(" and if incremented by {m2}, it becomes {sd1}.");
    }
    {
        let mut sd1 = SignedDirection::<Degree>::new(12.);
        let m2 = Measure::<Degree>::new(13.);
        print!("{sd1} minus {m2} is {sd3},", sd3 = sd1 - m2);

        sd1 -= m2;
        println!(" and if decremented by {m2}, it becomes {sd1}.");
    }
    {
        let sd1 = SignedDirection::<Degree>::new(12.);
        let sd2 = SignedDirection::<Degree>::new(13.);
        println!("{sd1} minus {sd2} is {m3}.", m3 = sd1 - sd2);
    }
    {
        let sd1 = SignedDirection::<Degree>::new(12.);
        let sd2 = sd1;
        println!("{sd1} == {sd1} is {result}.", result = sd1 == sd2);
        println!("{sd1} < {sd1} is {result}.", result = sd1 < sd2);
    }
    println!();
}

fn print_all_single_unit_operations_for_measure_2d() {
    println!("* Single unit operations for measure 2d");
    {
        let m1 = Measure2d::<KilometrePerHour>::new([12., 13.]);
        println!(
            "{m1} has components X={m2} and Y={m3}.",
            m2 = m1.x(),
            m3 = m1.y(),
        );
    }
    {
        let m1 = Measure2d::<KilometrePerHour>::new([12., 13.]);
        println!(
            "{m1} can be converted to {m2}.",
            m2 = m1.convert::<MilePerHour>()
        );
    }
    {
        let m1 =
            Measure2d::<KilometrePerHour>::new([1.234_567_890_123_456_7, 2.345_678_901_234_568]);
        println!(
            "{m1} can be lossy-converted to {m2}.",
            m2 = m1.lossy_into::<f32>()
        );
    }
    {
        let m1 = Measure2d::<KilometrePerHour, f32>::new([1.234_567_9, 2.345_678_8]);
        let m2: Measure2d<KilometrePerHour, f64> = m1.into();
        println!("{m1} can be lossless-converted to {m2}.",);
    }
    {
        let m1 = Measure2d::<KilometrePerHour>::new([-12., -13.]);
        println!("The squared norm of {m1} is {n}.", n = m1.squared_norm());
    }
    {
        let m1 = Measure2d::<KilometrePerHour>::new([-12., -13.]);
        println!("{m1} normalized is {n}.", n = m1.normalized());
    }
    {
        let mp1 = MeasurePoint::<Degree>::new(12.);
        println!(
            "{m1} can be created from angle {mp1}.",
            m1 = Measure2d::<KilometrePerHour>::from_angle(mp1)
        );
    }
    {
        let m1 = Measure2d::<KilometrePerHour>::new([12., 13.]);
        println!(
            "{m1} has signed direction {sd2}.",
            sd2 = m1.signed_direction::<Degree>()
        );
    }
    {
        let m1 = Measure2d::<KilometrePerHour>::new([12., 13.]);
        println!(
            "{m1} has unsigned direction {ud2}.",
            ud2 = m1.unsigned_direction::<Degree>()
        );
    }
    {
        let m1 = Measure2d::<KilometrePerHour>::new([12., -13.]);
        println!("The opposite of {m1} is {m2}.", m2 = -m1);
    }
    {
        let mut m1 = Measure2d::<KilometrePerHour>::new([12., 13.]);
        let m2 = Measure2d::<KilometrePerHour>::new([15., 19.]);
        print!("{m1} plus {m2} is {m3},", m3 = m1 + m2);

        m1 += m2;
        println!(" and if incremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure2d::<KilometrePerHour>::new([12., 13.]);
        let m2 = Measure2d::<KilometrePerHour>::new([15., 19.]);
        print!("{m1} minus {m2} is {m3},", m3 = m1 - m2);

        m1 -= m2;
        println!(" and if decremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure2d::<KilometrePerHour>::new([12., 13.]);
        let multiplier = 2.;
        print!("{m1} times {multiplier} is {m2},", m2 = m1 * multiplier);

        m1 *= multiplier;
        println!(" and if multiplied by {multiplier}, it becomes {m1}.");
    }
    {
        let m1 = Measure2d::<KilometrePerHour>::new([12., 13.]);
        let multiplier = 2.;
        println!("{multiplier} times {m1} is {m2}.", m2 = multiplier * m1);
    }
    {
        let mut m1 = Measure2d::<KilometrePerHour>::new([12., 13.]);
        let divisor = 2.;
        print!("{m1} divided by {divisor} is {m2},", m2 = m1 / divisor);

        m1 /= divisor;
        println!(" and if divided by {divisor}, it becomes {m1}.");
    }
    {
        let m1 = Measure2d::<KilometrePerHour>::new([12., 13.]);
        let m2 = m1;
        println!("{m1} == {m1} is {result}.", result = m1 == m2);
    }
    println!();
}

fn print_all_single_unit_operations_for_measure_point_2d() {
    println!("* Single unit operations for measure point 2d");
    {
        let mp1 = MeasurePoint2d::<KilometrePerHour>::new([12., 13.]);
        println!(
            "{mp1} has components X={mp2} and Y={mp3}.",
            mp2 = mp1.x(),
            mp3 = mp1.y(),
        );
    }
    {
        let mp1 = MeasurePoint2d::<KilometrePerHour>::new([12., 13.]);
        println!(
            "{mp1} can be converted to {mp2}.",
            mp2 = mp1.convert::<MilePerHour>()
        );
    }
    {
        let mp1 = MeasurePoint2d::<KilometrePerHour>::new([
            1.234_567_890_123_456_7,
            2.345_678_901_234_568,
        ]);
        println!(
            "{mp1} can be lossy-converted to {mp2}.",
            mp2 = mp1.lossy_into::<f32>()
        );
    }
    {
        let mp1 = MeasurePoint2d::<KilometrePerHour, f32>::new([1.234_567_9, 2.345_678_8]);
        let mp2: MeasurePoint2d<KilometrePerHour, f64> = mp1.into();
        println!("{mp1} can be lossless-converted to {mp2}.",);
    }
    {
        let mut mp1 = MeasurePoint2d::<KilometrePerHour>::new([12., 13.]);
        let m2 = Measure2d::<KilometrePerHour>::new([15., 19.]);
        print!("{mp1} plus {m2} is {mp3},", mp3 = mp1 + m2);

        mp1 += m2;
        println!(" and if incremented by {m2}, it becomes {mp1}.");
    }
    {
        let mut mp1 = MeasurePoint2d::<KilometrePerHour>::new([12., 13.]);
        let m2 = Measure2d::<KilometrePerHour>::new([15., 19.]);
        print!("{mp1} minus {m2} is {mp3},", mp3 = mp1 - m2);

        mp1 -= m2;
        println!(" and if decremented by {m2}, it becomes {mp1}.");
    }
    {
        let mp1 = MeasurePoint2d::<KilometrePerHour>::new([12., 13.]);
        let mp2 = MeasurePoint2d::<KilometrePerHour>::new([15., 19.]);
        println!("{mp1} minus {mp2} is {m3},", m3 = mp1 - mp2);
    }
    {
        let mp1 = MeasurePoint2d::<KilometrePerHour>::new([10., -100.]);
        let mp2 = MeasurePoint2d::<KilometrePerHour>::new([20., -200.]);
        println!("The weighted midpoint between {mp1} (with weight 40%) and {mp2} (with weight 60%) is {mp3}.", mp3 = weighted_midpoint_2d(mp1, mp2, 0.4));
        println!(
            "The midpoint between {mp1} and {mp2} is {mp3}.",
            mp3 = midpoint_2d(mp1, mp2)
        );
    }
    {
        let mp1 = MeasurePoint2d::<KilometrePerHour>::new([10., -100.]);
        let mp2 = MeasurePoint2d::<KilometrePerHour>::new([20., -200.]);
        let mp3 = MeasurePoint2d::<KilometrePerHour>::new([40., -400.]);
        println!("The barycentric combination among {mp1} (with weight 10%), {mp2} (with weight 20%), and {mp3} (with weight 70%) is {mp4}.", mp4 = barycentric_combination_2d(&[mp1, mp2, mp3], &[0.1, 0.2, 0.7]));
    }
    {
        let mp1 = MeasurePoint2d::<KilometrePerHour>::new([12., 13.]);
        let mp2 = mp1;
        println!("{mp1} == {mp1} is {result}.", result = mp1 == mp2);
    }
    println!();
}

fn print_all_single_unit_operations_for_measure_3d() {
    println!("* Single unit operations for measure 3d");
    {
        let m1 = Measure3d::<KilometrePerHour>::new([12., 13., 14.]);
        println!(
            "{m1} has components X={m2}, Y={m3}, and Z={m4}.",
            m2 = m1.x(),
            m3 = m1.y(),
            m4 = m1.z(),
        );
    }
    {
        let m1 = Measure3d::<KilometrePerHour>::new([12., 13., 14.]);
        println!(
            "{m1} can be converted to {m2}.",
            m2 = m1.convert::<MilePerHour>()
        );
    }
    {
        let m1 = Measure3d::<KilometrePerHour>::new([
            1.234_567_890_123_456_7,
            2.345_678_901_234_568,
            3.456_789_012_345_679,
        ]);
        println!(
            "{m1} can be lossy-converted to {m2}.",
            m2 = m1.lossy_into::<f32>()
        );
    }
    {
        let m1 = Measure3d::<KilometrePerHour, f32>::new([1.234_567_9, 2.345_678_8, 3.456_789]);
        let m2: Measure3d<KilometrePerHour, f64> = m1.into();
        println!("{m1} can be lossless-converted to {m2}.",);
    }
    {
        let m1 = Measure3d::<KilometrePerHour>::new([-12., -13., -14.]);
        println!("The squared norm of {m1} is {n}.", n = m1.squared_norm());
    }
    {
        let m1 = Measure3d::<KilometrePerHour>::new([-12., -13., -14.]);
        println!("{m1} normalized is {n}.", n = m1.normalized());
    }
    {
        let m1 = Measure3d::<KilometrePerHour>::new([12., -13., -14.]);
        println!("The opposite of {m1} is {m2}.", m2 = -m1);
    }
    {
        let mut m1 = Measure3d::<KilometrePerHour>::new([12., 13., 14.]);
        let m2 = Measure3d::<KilometrePerHour>::new([15., 19., 26.]);
        print!("{m1} plus {m2} is {m3},", m3 = m1 + m2);

        m1 += m2;
        println!(" and if incremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure3d::<KilometrePerHour>::new([12., 13., 14.]);
        let m2 = Measure3d::<KilometrePerHour>::new([15., 19., 26.]);
        print!("{m1} minus {m2} is {m3},", m3 = m1 - m2);

        m1 -= m2;
        println!(" and if decremented by {m2}, it becomes {m1}.");
    }
    {
        let mut m1 = Measure3d::<KilometrePerHour>::new([12., 13., 14.]);
        let multiplier = 2.;
        print!("{m1} times {multiplier} is {m2},", m2 = m1 * multiplier);

        m1 *= multiplier;
        println!(" and if multiplied by {multiplier}, it becomes {m1}.");
    }
    {
        let m1 = Measure3d::<KilometrePerHour>::new([12., 13., 14.]);
        let multiplier = 2.;
        println!("{multiplier} times {m1} is {m2}.", m2 = multiplier * m1);
    }
    {
        let mut m1 = Measure3d::<KilometrePerHour>::new([12., 13., 14.]);
        let divisor = 2.;
        print!("{m1} divided by {divisor} is {m2},", m2 = m1 / divisor);

        m1 /= divisor;
        println!(" and if divided by {divisor}, it becomes {m1}.");
    }
    {
        let m1 = Measure3d::<KilometrePerHour>::new([12., 13., 14.]);
        let m2 = m1;
        println!("{m1} == {m1} is {result}.", result = m1 == m2);
    }
    println!();
}

fn print_all_single_unit_operations_for_measure_point_3d() {
    println!("* Single unit operations for measure point 3d");
    {
        let mp1 = MeasurePoint3d::<KilometrePerHour>::new([12., 13., 14.]);
        println!(
            "{mp1} has components X={mp2} and Y={mp3}.",
            mp2 = mp1.x(),
            mp3 = mp1.y(),
        );
    }
    {
        let mp1 = MeasurePoint3d::<KilometrePerHour>::new([12., 13., 14.]);
        println!(
            "{mp1} can be converted to {mp2}.",
            mp2 = mp1.convert::<MilePerHour>()
        );
    }
    {
        let mp1 = MeasurePoint3d::<KilometrePerHour>::new([
            1.234_567_890_123_456_7,
            2.345_678_901_234_568,
            3.456_789_012_345_679,
        ]);
        println!(
            "{mp1} can be lossy-converted to {mp2}.",
            mp2 = mp1.lossy_into::<f32>()
        );
    }
    {
        let mp1 =
            MeasurePoint3d::<KilometrePerHour, f32>::new([1.234_567_9, 2.345_678_8, 3.456_789]);
        let mp2: MeasurePoint3d<KilometrePerHour, f64> = mp1.into();
        println!("{mp1} can be lossless-converted to {mp2}.",);
    }
    {
        let mut mp1 = MeasurePoint3d::<KilometrePerHour>::new([12., 13., 14.]);
        let m2 = Measure3d::<KilometrePerHour>::new([15., 19., 26.]);
        print!("{mp1} plus {m2} is {mp3},", mp3 = mp1 + m2);

        mp1 += m2;
        println!(" and if incremented by {m2}, it becomes {mp1}.");
    }
    {
        let mut mp1 = MeasurePoint3d::<KilometrePerHour>::new([12., 13., 14.]);
        let m2 = Measure3d::<KilometrePerHour>::new([15., 19., 26.]);
        print!("{mp1} minus {m2} is {mp3},", mp3 = mp1 - m2);

        mp1 -= m2;
        println!(" and if decremented by {m2}, it becomes {mp1}.");
    }
    {
        let mp1 = MeasurePoint3d::<KilometrePerHour>::new([12., 13., 14.]);
        let mp2 = MeasurePoint3d::<KilometrePerHour>::new([15., 19., 26.]);
        println!("{mp1} minus {mp2} is {m3},", m3 = mp1 - mp2);
    }
    {
        let mp1 = MeasurePoint3d::<KilometrePerHour>::new([10., -100., 40.]);
        let mp2 = MeasurePoint3d::<KilometrePerHour>::new([20., -200., 80.]);
        println!("The weighted midpoint between {mp1} (with weight 40%) and {mp2} (with weight 60%) is {mp3}.", mp3 = weighted_midpoint_3d(mp1, mp2, 0.4));
        println!(
            "The midpoint between {mp1} and {mp2} is {mp3}.",
            mp3 = midpoint_3d(mp1, mp2)
        );
    }
    {
        let mp1 = MeasurePoint3d::<KilometrePerHour>::new([10., -100., 40.]);
        let mp2 = MeasurePoint3d::<KilometrePerHour>::new([20., -200., 80.]);
        let mp3 = MeasurePoint3d::<KilometrePerHour>::new([40., -400., 160.]);
        println!("The barycentric combination among {mp1} (with weight 10%), {mp2} (with weight 20%), and {mp3} (with weight 70%) is {mp4}.", mp4 = barycentric_combination_3d(&[mp1, mp2, mp3], &[0.1, 0.2, 0.7]));
    }
    {
        let mp1 = MeasurePoint3d::<KilometrePerHour>::new([12., 13., 14.]);
        let mp2 = mp1;
        println!("{mp1} == {mp1} is {result}.", result = mp1 == mp2);
    }
    println!();
}

fn print_all_single_unit_operations() {
    print_all_single_unit_operations_for_measure_1d();
    print_all_single_unit_operations_for_measure_point_1d();
    print_all_single_unit_operations_for_unsigned_directions();
    print_all_single_unit_operations_for_signed_directions();
    print_all_single_unit_operations_for_measure_2d();
    print_all_single_unit_operations_for_measure_point_2d();
    print_all_single_unit_operations_for_measure_3d();
    print_all_single_unit_operations_for_measure_point_3d();
}

fn print_all_computer_science_mixed_operations() {
    println!("* Computer science mixed operations");
    {
        let information = Measure::<Bit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<BitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Byte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<BytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Kilobit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<KilobitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Kilobyte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<KilobytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Kibibit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<KibibitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Kibibyte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<KibibytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Megabit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<MegabitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Megabyte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<MegabytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}")
    }
    {
        let information = Measure::<Mebibit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<MebibitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Mebibyte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<MebibytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Gigabit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<GigabitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Gigabyte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<GigabytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Gibibit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<GibibitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Gibibyte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<GibibytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Terabit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<TerabitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Terabyte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<TerabytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Tebibit>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<TebibitPerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    {
        let information = Measure::<Tebibyte>::new(6.);
        let time = Measure::<Second>::new(2.);
        let information_rate: Measure<TebibytePerSecond> = information / time;
        println!("{information} every {time} is a rate of {information_rate}.")
    }
    println!();
}

fn print_all_geometry_mixed_operations() {
    println!("* Geometry mixed operations");
    {
        let length = Measure::<Metre>::new(6.);
        let width = Measure::<Metre>::new(2.);
        let area: Measure<SquareMetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Kilometre>::new(6.);
        let width = Measure::<Kilometre>::new(2.);
        let area: Measure<SquareKilometre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Hectometre>::new(6.);
        let width = Measure::<Hectometre>::new(2.);
        let area: Measure<Hectare> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Decametre>::new(6.);
        let width = Measure::<Decametre>::new(2.);
        let area: Measure<Are> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Decimetre>::new(6.);
        let width = Measure::<Decimetre>::new(2.);
        let area: Measure<SquareDecimetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Centimetre>::new(6.);
        let width = Measure::<Centimetre>::new(2.);
        let area: Measure<SquareCentimetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Millimetre>::new(6.);
        let width = Measure::<Millimetre>::new(2.);
        let area: Measure<SquareMillimetre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Micrometre>::new(6.);
        let width = Measure::<Micrometre>::new(2.);
        let area: Measure<SquareMicrometre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Nanometre>::new(6.);
        let width = Measure::<Nanometre>::new(2.);
        let area: Measure<SquareNanometre> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Inch>::new(6.);
        let width = Measure::<Inch>::new(2.);
        let area: Measure<SquareInch> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Foot>::new(6.);
        let width = Measure::<Foot>::new(2.);
        let area: Measure<SquareFoot> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Yard>::new(6.);
        let width = Measure::<Yard>::new(2.);
        let area: Measure<SquareYard> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let length = Measure::<Mile>::new(6.);
        let width = Measure::<Mile>::new(2.);
        let area: Measure<SquareMile> = length * width;
        println!("A rectangle {length} long and {width} wide has an area of {area}.")
    }
    {
        let base = Measure::<SquareMetre>::new(6.);
        let height = Measure::<Metre>::new(2.);
        let volume: Measure<CubicMetre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareKilometre>::new(6.);
        let height = Measure::<Kilometre>::new(2.);
        let volume: Measure<CubicKilometre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareDecimetre>::new(6.);
        let height = Measure::<Decimetre>::new(2.);
        let volume: Measure<Litre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareCentimetre>::new(6.);
        let height = Measure::<Centimetre>::new(2.);
        let volume: Measure<Millilitre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareMillimetre>::new(6.);
        let height = Measure::<Millimetre>::new(2.);
        let volume: Measure<Microlitre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareMicrometre>::new(6.);
        let height = Measure::<Micrometre>::new(2.);
        let volume: Measure<CubicMicrometre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareNanometre>::new(6.);
        let height = Measure::<Nanometre>::new(2.);
        let volume: Measure<CubicNanometre> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareInch>::new(6.);
        let height = Measure::<Inch>::new(2.);
        let volume: Measure<CubicInch> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareFoot>::new(6.);
        let height = Measure::<Foot>::new(2.);
        let volume: Measure<CubicFoot> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareYard>::new(6.);
        let height = Measure::<Yard>::new(2.);
        let volume: Measure<CubicYard> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }
    {
        let base = Measure::<SquareMile>::new(6.);
        let height = Measure::<Mile>::new(2.);
        let volume: Measure<CubicMile> = base * height;
        println!(
            "A prism having a base of {base} and a height of {height} has a volume of {volume}."
        )
    }

    {
        let phase = Measure::<Cycle>::new(6.);
        let length = Measure::<Metre>::new(2.);
        let wave_number: Measure<CyclePerMetre> = phase / length;
        println!("A portion of wave having a phase of {phase} for a length of {length} has a wave number of {wave_number}.")
    }
    {
        let phase = Measure::<Radian>::new(6.);
        let length = Measure::<Metre>::new(2.);
        let wave_number: Measure<RadianPerMetre> = phase / length;
        println!("A portion of wave having a phase of {phase} for a length of {length} has a wave number of {wave_number}.")
    }
    println!();
}

fn print_all_kinematics_mixed_operations() {
    println!("* Kinematics mixed operations");
    {
        let velocity = Measure::<MetrePerSecond>::new(6.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure<MetrePerSquareSecond> = velocity / time;
        println!("If an object in a line changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure2d::<MetrePerSecond>::new([6., 20.]);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure2d<MetrePerSquareSecond> = velocity / time;
        println!("If an object in a plane changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure3d::<MetrePerSecond>::new([6., 20., 36.]);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure3d<MetrePerSquareSecond> = velocity / time;
        println!("If an object in the space changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure::<CentimetrePerSecond>::new(6.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure<CentimetrePerSquareSecond> = velocity / time;
        println!("If an object in a line changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure2d::<CentimetrePerSecond>::new([6., 20.]);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure2d<CentimetrePerSquareSecond> = velocity / time;
        println!("If an object in a plane changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure3d::<CentimetrePerSecond>::new([6., 20., 36.]);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure3d<CentimetrePerSquareSecond> = velocity / time;
        println!("If an object in the space changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure::<KilometrePerHour>::new(6.);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure<KilometrePerHourPerSecond> = velocity / time;
        println!("If an object in a line changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure2d::<KilometrePerHour>::new([6., 20.]);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure2d<KilometrePerHourPerSecond> = velocity / time;
        println!("If an object in a plane changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let velocity = Measure3d::<KilometrePerHour>::new([6., 20., 36.]);
        let time = Measure::<Second>::new(2.);
        let acceleration: Measure3d<KilometrePerHourPerSecond> = velocity / time;
        println!("If an object in the space changes its velocity of {velocity} in a time interval of {time}, it has an average acceleration of {acceleration}.")
    }
    {
        let length = Measure::<Metre>::new(6.);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure<MetrePerSecond> = length / time;
        println!("If an object moves in a line by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure2d::<Metre>::new([6., 20.]);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure2d<MetrePerSecond> = length / time;
        println!("If an object moves in a plane by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure3d::<Metre>::new([6., 20., 36.]);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure3d<MetrePerSecond> = length / time;
        println!("If an object moves in the space by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure::<NauticalMile>::new(6.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure<Knot> = length / time;
        println!("If an object moves in a line by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure2d::<NauticalMile>::new([6., 20.]);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure2d<Knot> = length / time;
        println!("If an object moves in a plane by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure3d::<NauticalMile>::new([6., 20., 36.]);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure3d<Knot> = length / time;
        println!("If an object moves in the space by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure::<Kilometre>::new(6.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure<KilometrePerHour> = length / time;
        println!("If an object moves in a line by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure2d::<Kilometre>::new([6., 20.]);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure2d<KilometrePerHour> = length / time;
        println!("If an object moves in a plane by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure3d::<Kilometre>::new([6., 20., 36.]);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure3d<KilometrePerHour> = length / time;
        println!("If an object moves in the space by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure::<Mile>::new(6.);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure<MilePerHour> = length / time;
        println!("If an object moves in a line by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure2d::<Mile>::new([6., 20.]);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure2d<MilePerHour> = length / time;
        println!("If an object moves in a plane by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure3d::<Mile>::new([6., 20., 36.]);
        let time = Measure::<Hour>::new(2.);
        let velocity: Measure3d<MilePerHour> = length / time;
        println!("If an object moves in the space by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure::<Centimetre>::new(6.);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure<CentimetrePerSecond> = length / time;
        println!("If an object moves in a line by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure2d::<Centimetre>::new([6., 20.]);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure2d<CentimetrePerSecond> = length / time;
        println!("If an object moves in a plane by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let length = Measure3d::<Centimetre>::new([6., 20., 36.]);
        let time = Measure::<Second>::new(2.);
        let velocity: Measure3d<CentimetrePerSecond> = length / time;
        println!("If an object moves in the space by a distance of {length} in a time interval of {time}, it has an average velocity of {velocity}.")
    }
    {
        let angular_velocity = Measure::<RadianPerSecond>::new(6.);
        let time = Measure::<Second>::new(2.);
        let angular_acceleration: Measure<RadianPerSquareSecond> = angular_velocity / time;
        println!("If an object changes its angular velocity by {angular_velocity} in a time interval of {time}, it has an average angular acceleration of {angular_acceleration}.")
    }
    {
        let angular_position = Measure::<Cycle>::new(6.);
        let time = Measure::<Second>::new(2.);
        let angular_velocity: Measure<Hertz> = angular_position / time;
        println!("If an object changes its angular position (or phase) by {angular_position} in a time interval of {time}, it has an average angular velocity (or average frequency) of {angular_velocity}.")
    }
    {
        let angular_position = Measure::<Radian>::new(6.);
        let time = Measure::<Second>::new(2.);
        let angular_velocity: Measure<RadianPerSecond> = angular_position / time;
        println!("If an object changes its angular position (or phase) by {angular_position} in a time interval of {time}, it has an average angular velocity (or average frequency) of {angular_velocity}.")
    }
    {
        let angular_position = Measure::<Cycle>::new(6.);
        let time = Measure::<Minute>::new(2.);
        let angular_velocity: Measure<CyclePerMinute> = angular_position / time;
        println!("If an object changes its angular position (or phase) by {angular_position} in a time interval of {time}, it has an average angular velocity (or average frequency) of {angular_velocity}.")
    }
    // TODO KinematicViscosity
    // TODO SquareTime
    {
        let volume = Measure::<CubicMetre>::new(6.);
        let time = Measure::<Second>::new(2.);
        let flow_rate: Measure<CubicMetrePerSecond> = volume / time;
        println!("If some fluid having a volume of {volume} crosses a surface in a time interval of {time}, that fluid has an average volumetric flow rate of {flow_rate}.")
    }
    {
        let volume = Measure::<Millilitre>::new(6.);
        let time = Measure::<Second>::new(2.);
        let flow_rate: Measure<MillilitrePerSecond> = volume / time;
        println!("If some fluid having a volume of {volume} crosses a surface in a time interval of {time}, that fluid has an average volumetric flow rate of {flow_rate}.")
    }
    {
        let area = Measure::<SquareMetre>::new(6.);
        let velocity = Measure::<MetrePerSecond>::new(2.);
        let flow_rate: Measure<CubicMetrePerSecond> = area * velocity;
        println!("If some fluid crosses a surface with area {area} having a velocity of {velocity}, that fluid has an average volumetric flow rate of {flow_rate}.")
    }
    {
        let area = Measure::<SquareCentimetre>::new(6.);
        let velocity = Measure::<CentimetrePerSecond>::new(2.);
        let flow_rate: Measure<MillilitrePerSecond> = area * velocity;
        println!("If some fluid crosses a surface with area {area} having a velocity of {velocity}, that fluid has an average volumetric flow rate of {flow_rate}.")
    }
    println!();
}

fn print_all_dynamics_mixed_operations() {
    println!("* Dynamics mixed operations");
    // TODO Action == Energy * Time
    // TODO Action == Power * SquareTime
    {
        let impulse = Measure2d::<KilogramMetrePerSecond>::new([6., 2.]);
        let arm = Measure2d::<Metre>::new([1., 4.]);
        let angular_momentum: Measure<KilogramSquareMetrePerSecond> = impulse.cross_product(arm);
        println!("If some object constrained to rotate around an axis receives an impulse of {impulse} laying in the plane orthogonal to the axis, applied at a position from the rotation axis of {arm}, that causes a variation of the angular momentum by {angular_momentum}.")
    }
    {
        let impulse = Measure3d::<KilogramMetrePerSecond>::new([6., 2., -3.]);
        let arm = Measure3d::<Metre>::new([1., 4., 6.]);
        let angular_momentum: Measure3d<KilogramSquareMetrePerSecond> = impulse.cross_product(arm);
        println!("If some object constrained to rotate around a point receives an impulse of {impulse}, applied at a position from the rotation point of {arm}, that causes a variation of the angular momentum by {angular_momentum}.")
    }
    // TODO AngularMomentum == MomentOfInertia / Time
    // TODO DynamicViscosity == Pressure * Time
    {
        let force = Measure::<Newton>::new(6.);
        let movement = Measure::<Metre>::new(2.);
        let work: Measure<Joule> = force * movement;
        println!("If a force of {force} is applied to an object while it moves in the same direction by {movement}, that force gives to that object an energy of {work}.")
    }
    {
        let force = Measure2d::<Newton>::new([6., -3.]);
        let movement = Measure2d::<Metre>::new([2., 5.]);
        let work: Measure<Joule> = force * movement;
        println!("If a planar force of {force} is applied to an object while it moves in the same plane by {movement}, that force gives to that object an energy of {work}.")
    }
    {
        let force = Measure3d::<Newton>::new([6., -3., 5.]);
        let movement = Measure3d::<Metre>::new([2., 5., 7.]);
        let work: Measure<Joule> = force * movement;
        println!("If a force of {force} is applied to an object while it moves in space by {movement}, that force gives to that object an energy of {work}.")
    }
    {
        let force = Measure::<Dyne>::new(6.);
        let movement = Measure::<Centimetre>::new(2.);
        let work: Measure<Erg> = force * movement;
        println!("If a force of {force} is applied to an object while it moves in the same direction by {movement}, that force gives to that object an energy of {work}.")
    }
    {
        let force = Measure2d::<Dyne>::new([6., -3.]);
        let movement = Measure2d::<Centimetre>::new([2., 5.]);
        let work: Measure<Erg> = force * movement;
        println!("If a planar force of {force} is applied to an object while it moves in the same plane by {movement}, that force gives to that object an energy of {work}.")
    }
    {
        let force = Measure3d::<Dyne>::new([6., -3., 5.]);
        let movement = Measure3d::<Centimetre>::new([2., 5., 7.]);
        let work: Measure<Erg> = force * movement;
        println!("If a force of {force} is applied to an object while it moves in space by {movement}, that force gives to that object an energy of {work}.")
    }
    // TODO Energy == Momentum * Velocity
    // TODO Energy == MomentOfInertia / SquareTime
    // EnergyDensity == Energy / Volume
    {
        let energy = Measure::<Joule>::new(6.);
        let volume = Measure::<CubicMetre>::new(2.);
        let energy_density: Measure<JoulePerCubicMetre> = energy / volume;
        println!("If an energy of {energy} is contained in a volume of {volume}, it has an average energy density of {energy_density}.")
    }

    {
        let force = Measure::<Newton>::new(6.);
        let mass = Measure::<Kilogram>::new(2.);
        let acceleration: Measure<MetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied to an object having mass {mass}, that object is accelerated along the direction of the force by {acceleration}.")
    }
    {
        let force = Measure2d::<Newton>::new([6., -3.]);
        let mass = Measure::<Kilogram>::new(2.);
        let acceleration: Measure2d<MetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied in a plane to an object having mass {mass}, that object is accelerated in that plane by {acceleration}.")
    }
    {
        let force = Measure3d::<Newton>::new([6., -3., 5.]);
        let mass = Measure::<Kilogram>::new(2.);
        let acceleration: Measure3d<MetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied in space to an object having mass {mass}, that object is accelerated by {acceleration}.")
    }
    {
        let force = Measure::<Dyne>::new(6.);
        let mass = Measure::<Gram>::new(2.);
        let acceleration: Measure<CentimetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied to an object having mass {mass}, that object is accelerated along the direction of the force by {acceleration}.")
    }
    {
        let force = Measure2d::<Dyne>::new([6., -3.]);
        let mass = Measure::<Gram>::new(2.);
        let acceleration: Measure2d<CentimetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied in a plane to an object having mass {mass}, that object is accelerated in that plane by {acceleration}.")
    }
    {
        let force = Measure3d::<Dyne>::new([6., -3., 5.]);
        let mass = Measure::<Gram>::new(2.);
        let acceleration: Measure3d<CentimetrePerSquareSecond> = force / mass;
        println!("If a force of {force} is applied in space to an object having mass {mass}, that object is accelerated by {acceleration}.")
    }
    {
        let force = Measure::<KilogramForce>::new(6.);
        let mass = Measure::<Kilogram>::new(2.);
        let acceleration: Measure<GForce> = force / mass;
        println!("If a force of {force} is applied to an object having mass {mass}, that object is accelerated along the direction of the force by {acceleration}.")
    }
    {
        let force = Measure2d::<KilogramForce>::new([6., -3.]);
        let mass = Measure::<Kilogram>::new(2.);
        let acceleration: Measure2d<GForce> = force / mass;
        println!("If a force of {force} is applied in a plane to an object having mass {mass}, that object is accelerated in that plane by {acceleration}.")
    }
    {
        let force = Measure3d::<KilogramForce>::new([6., -3., 5.]);
        let mass = Measure::<Kilogram>::new(2.);
        let acceleration: Measure3d<GForce> = force / mass;
        println!("If a force of {force} is applied in space to an object having mass {mass}, that object is accelerated by {acceleration}.")
    }

    {
        let mass = Measure::<Kilogram>::new(6.);
        let length = Measure::<Metre>::new(2.);
        let linear_density: Measure<KilogramPerMetre> = mass / length;
        println!("If a mass of {mass} is distributed along a length of {length}, that mass has a linear density by {linear_density}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let length = Measure::<Centimetre>::new(2.);
        let linear_density: Measure<GramPerCentimetre> = mass / length;
        println!("If a mass of {mass} is distributed along a length of {length}, that mass has a linear density by {linear_density}.")
    }
    {
        let mass = Measure::<Kilogram>::new(6.);
        let volume = Measure::<CubicMetre>::new(2.);
        let space_density: Measure<KilogramPerCubicMetre> = mass / volume;
        println!("If a mass of {mass} occupies a volume of {volume}, it has a density by {space_density}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let volume = Measure::<Millilitre>::new(2.);
        let space_density: Measure<GramPerMillilitre> = mass / volume;
        println!("If a mass of {mass} occupies a volume of {volume}, it has a density by {space_density}.")
    }
    {
        let mass = Measure::<Kilogram>::new(6.);
        let time = Measure::<Second>::new(2.);
        let mass_flow_rate: Measure<KilogramPerSecond> = mass / time;
        println!("If a mass of {mass} crosses a surface in a time interval of {time}, it has a mass flow rate of {mass_flow_rate}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let time = Measure::<Second>::new(2.);
        let mass_flow_rate: Measure<GramPerSecond> = mass / time;
        println!("If a mass of {mass} crosses a surface in a time interval of {time}, it has a mass flow rate of {mass_flow_rate}.")
    }
    // TODO MomentOfInertia == Mass * Area
    // TODO Momentum == Force * Time
    {
        let mass = Measure::<Kilogram>::new(6.);
        let velocity = Measure::<MetrePerSecond>::new(2.);
        let momentum: Measure<NewtonSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in a line of {velocity} has a momentum of {momentum}.")
    }
    {
        let mass = Measure::<Kilogram>::new(6.);
        let velocity = Measure2d::<MetrePerSecond>::new([2., 5.]);
        let momentum: Measure2d<NewtonSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in a plane of {velocity} has a momentum of {momentum}.")
    }
    {
        let mass = Measure::<Kilogram>::new(6.);
        let velocity = Measure3d::<MetrePerSecond>::new([2., 5., -3.]);
        let momentum: Measure3d<NewtonSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in space of {velocity} has a momentum of {momentum}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let velocity = Measure::<CentimetrePerSecond>::new(2.);
        let momentum: Measure<DyneSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in a line of {velocity} has a momentum of {momentum}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let velocity = Measure2d::<CentimetrePerSecond>::new([2., 5.]);
        let momentum: Measure2d<DyneSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in a plane of {velocity} has a momentum of {momentum}.")
    }
    {
        let mass = Measure::<Gram>::new(6.);
        let velocity = Measure3d::<CentimetrePerSecond>::new([2., 5., -3.]);
        let momentum: Measure3d<DyneSecond> = mass * velocity;
        println!("A mass of {mass} having a velocity in space of {velocity} has a momentum of {momentum}.")
    }
    {
        let energy = Measure::<Joule>::new(6.);
        let time = Measure::<Second>::new(2.);
        let power: Measure<Watt> = energy / time;
        println!("A system generating or consuming the energy of {energy} every time interval of {time} has a power of {power}.")
    }
    {
        let energy = Measure::<WattHour>::new(6.);
        let time = Measure::<Hour>::new(2.);
        let power: Measure<Watt> = energy / time;
        println!("A system generating or consuming the energy of {energy} every time interval of {time} has a power of {power}.")
    }
    {
        let energy = Measure::<KilowattHour>::new(6.);
        let time = Measure::<Hour>::new(2.);
        let power: Measure<Kilowatt> = energy / time;
        println!("A system generating or consuming the energy of {energy} every time interval of {time} has a power of {power}.")
    }
    {
        let energy = Measure::<Erg>::new(6.);
        let time = Measure::<Second>::new(2.);
        let power: Measure<ErgPerSecond> = energy / time;
        println!("A system generating or consuming the energy of {energy} every time interval of {time} has a power of {power}.")
    }
    {
        let force = Measure::<Newton>::new(6.);
        let area = Measure::<SquareMetre>::new(2.);
        let pressure: Measure<Pascal> = force / area;
        println!("A force of {force} applied to an area of {area} exerts a pressure of {pressure}.")
    }
    {
        let force = Measure::<PoundForce>::new(6.);
        let area = Measure::<SquareInch>::new(2.);
        let pressure: Measure<PoundForcePerSquareInch> = force / area;
        println!("A force of {force} applied to an area of {area} exerts a pressure of {pressure}.")
    }
    {
        let force = Measure::<Newton>::new(6.);
        let area = Measure::<SquareDecimetre>::new(2.);
        let pressure: Measure<Hectopascal> = force / area;
        println!("A force of {force} applied to an area of {area} exerts a pressure of {pressure}.")
    }
    {
        let energy = Measure::<Joule>::new(6.);
        let mass = Measure::<Kilogram>::new(2.);
        let specific_energy: Measure<JoulePerKilogram> = energy / mass;
        println!("An energy of {energy} contained in a mass of {mass} has a specific energy of {specific_energy}.")
    }
    {
        let volume = Measure::<CubicMetre>::new(6.);
        let mass = Measure::<Kilogram>::new(2.);
        let specific_volume: Measure<CubicMetrePerKilogram> = volume / mass;
        println!("A volume of {volume} containing a mass of {mass} has a specific volume of {specific_volume}.")
    }
    {
        let mass_density = Measure::<KilogramPerCubicMetre>::new(2.);
        let specific_volume: Measure<CubicMetrePerKilogram> =
            Measure::<One>::new(1.) / mass_density;
        println!("A mass density of {mass_density} is equivalent to a specific volume of {specific_volume}.")
    }
    {
        let mass = Measure::<Kilogram>::new(6.);
        let area = Measure::<SquareMetre>::new(6.);
        let surface_density: Measure<KilogramPerSquareMetre> = mass / area;
        println!("A mass of {mass} distributed over an area of {area} has a surface density of {surface_density}.")
    }
    {
        let energy = Measure::<Joule>::new(6.);
        let area = Measure::<SquareMetre>::new(6.);
        let surface_tension: Measure<JoulePerSquareMetre> = energy / area;
        println!("An energy of {energy} distributed over an area of {area} has a surface tension of {surface_tension}.")
    }
    {
        let force = Measure2d::<Newton>::new([6., 2.]);
        let arm = Measure2d::<Metre>::new([1., 4.]);
        let torque: Measure<NewtonMetre> = force.cross_product(arm);
        println!("If some object constrained to rotate around an axis receives a force of {force} laying in the plane orthogonal to the axis, applied at a position from the rotation axis of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }
    {
        let force = Measure3d::<Newton>::new([6., 2., -3.]);
        let arm = Measure3d::<Metre>::new([1., 4., 6.]);
        let torque: Measure3d<NewtonMetre> = force.cross_product(arm);
        println!("If some object constrained to rotate around a point receives a force of {force}, applied at a position from the rotation point of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }

    {
        let force = Measure2d::<PoundForce>::new([6., 2.]);
        let arm = Measure2d::<Foot>::new([1., 4.]);
        let torque: Measure<PoundFoot> = force.cross_product(arm);
        println!("If some object constrained to rotate around an axis receives a force of {force} laying in the plane orthogonal to the axis, applied at a position from the rotation axis of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }
    {
        let force = Measure3d::<PoundForce>::new([6., 2., -3.]);
        let arm = Measure3d::<Foot>::new([1., 4., 6.]);
        let torque: Measure3d<PoundFoot> = force.cross_product(arm);
        println!("If some object constrained to rotate around a point receives a force of {force}, applied at a position from the rotation point of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }

    {
        let force = Measure2d::<PoundForce>::new([6., 2.]);
        let arm = Measure2d::<Inch>::new([1., 4.]);
        let torque: Measure<PoundInch> = force.cross_product(arm);
        println!("If some object constrained to rotate around an axis receives a force of {force} laying in the plane orthogonal to the axis, applied at a position from the rotation axis of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }
    {
        let force = Measure3d::<PoundForce>::new([6., 2., -3.]);
        let arm = Measure3d::<Inch>::new([1., 4., 6.]);
        let torque: Measure3d<PoundInch> = force.cross_product(arm);
        println!("If some object constrained to rotate around a point receives a force of {force}, applied at a position from the rotation point of {arm}, that causes a torque (a.k.a. moment of force) of {torque}.")
    }
    println!();
}

fn print_all_thermodynamics_mixed_operations() {
    println!("* Thermodynamics mixed operations");
    // TODO Entropy == Energy / Temperature
    // TODO SpecificHeatCapacity == Entropy / Mass
    // TODO ThermalConductivity == Power / Length / Temperature
    println!();
}

fn print_all_chemistry_mixed_operations() {
    println!("* Chemistry mixed operations");
    //TODO
    println!();
}

fn print_all_radioactivity_mixed_operations() {
    println!("* Radioactivity mixed operations");
    //TODO
    println!();
}

fn print_all_optics_mixed_operations() {
    println!("* Optics mixed operations");
    //TODO
    println!();
}

fn print_all_electricity_mixed_operations() {
    println!("* Electricity mixed operations");
    //TODO
    println!();
}

fn print_all_magnetism_mixed_operations() {
    println!("* Magnetism mixed operations");
    //TODO
    println!();
}

fn print_all_other_mixed_operations() {
    println!("* Other mixed operations");
    //TODO
    println!();
}

fn print_all_mixed_operation() {
    print_all_computer_science_mixed_operations();
    print_all_geometry_mixed_operations();
    print_all_kinematics_mixed_operations();
    print_all_dynamics_mixed_operations();
    print_all_thermodynamics_mixed_operations();
    print_all_chemistry_mixed_operations();
    print_all_radioactivity_mixed_operations();
    print_all_optics_mixed_operations();
    print_all_electricity_mixed_operations();
    print_all_magnetism_mixed_operations();
    print_all_other_mixed_operations();
}

fn print_all_transformations() {
    // TODO
}

fn main() {
    println!("*** full ***");
    print_all_units();
    print_all_single_unit_operations();
    print_all_mixed_operation();
    print_all_transformations();
}
