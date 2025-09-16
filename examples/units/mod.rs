measures::define_measure_types! {
    with_points with_directions with_2d with_3d with_transformations exact,

    // Relationships among units
    [
        // Computer science

        // Properties:
        // * Information
        // * InformationRate

        // InformationRate == Information / Time
        Bit 1 == BitPerSecond 1 * Second 1,
        Byte 1 == BytePerSecond 1 * Second 1,
        KiloBit 1 == KiloBitPerSecond 1 * Second 1,
        KiloByte 1 == KiloBytePerSecond 1 * Second 1,
        KibiBit 1 == KibiBitPerSecond 1 * Second 1,
        KibiByte 1 == KibiBytePerSecond 1 * Second 1,
        MegaBit 1 == MegaBitPerSecond 1 * Second 1,
        MegaByte 1 == MegaBytePerSecond 1 * Second 1,
        MebiBit 1 == MebiBitPerSecond 1 * Second 1,
        MebiByte 1 == MebiBytePerSecond 1 * Second 1,
        GigaBit 1 == GigaBitPerSecond 1 * Second 1,
        GigaByte 1 == GigaBytePerSecond 1 * Second 1,
        GibiBit 1 == GibiBitPerSecond 1 * Second 1,
        GibiByte 1 == GibiBytePerSecond 1 * Second 1,
        TeraBit 1 == TeraBitPerSecond 1 * Second 1,
        TeraByte 1 == TeraBytePerSecond 1 * Second 1,
        TebiBit 1 == TebiBitPerSecond 1 * Second 1,
        TebiByte 1 == TebiBytePerSecond 1 * Second 1,

        // Geometry

        // Properties:
        // * Angle
        // * Area
        // * Length
        // * SolidAngle
        // * Volume
        // * WaveNumber

        // Area == Length * Length
        SquareMetre 1 == Metre 1 * __ 1,
        SquareKiloMetre 1 == KiloMetre 1 * __ 1,
        Hectare 1 == HectoMetre 1 * __ 1,
        Are 1 == DecaMetre 1 * __ 1,
        SquareDeciMetre 1 == DeciMetre 1 * __ 1,
        SquareCentiMetre 1 == CentiMetre 1 * __ 1,
        SquareMilliMetre 1 == MilliMetre 1 * __ 1,
        SquareMicroMetre 1 == MicroMetre 1 * __ 1,
        SquareNanoMetre 1 == NanoMetre 1 * __ 1,
        SquareInch 1 == Inch 1 * __ 1,
        SquareFoot 1 == Foot 1 * __ 1,
        SquareYard 1 == Yard 1 * __ 1,
        SquareMile 1 == Mile 1 * __ 1,

        // Volume == Area * Length
        CubicMetre 1 == SquareMetre 1 * Metre 1,
        CubicKiloMetre 1 == SquareKiloMetre 1 * KiloMetre 1,
        Litre 1 == SquareDeciMetre 1 * DeciMetre 1,
        MilliLitre 1 == SquareCentiMetre 1 * CentiMetre 1,
        MicroLitre 1 == SquareMilliMetre 1 * MilliMetre 1,
        CubicMicroMetre 1 == SquareMicroMetre 1 * MicroMetre 1,
        CubicNanoMetre 1 == SquareNanoMetre 1 * NanoMetre 1,
        CubicInch 1 == SquareInch 1 * Inch 1,
        CubicFoot 1 == SquareFoot 1 * Foot 1,
        CubicYard 1 == SquareYard 1 * Yard 1,
        CubicMile 1 == SquareMile 1 * Mile 1,

        // WaveNumber == Angle / Length
        Cycle 1 == CyclePerMetre 1 * Metre 1,
        Radian 1 == RadianPerMetre 1 * Metre 1,

        // Kinematics

        // Properties:
        // * Acceleration
        // * AngularAcceleration
        // * Frequency
        // * KinematicViscosity
        // * SquareTime
        // * Time
        // * Velocity
        // * VolumetricFlowRate

        // Acceleration == Velocity / Time
        MetrePerSecond 1 == MetrePerSquareSecond 1 * Second 1,
        MetrePerSecond 2 == MetrePerSquareSecond 2 * Second 1,
        MetrePerSecond 3 == MetrePerSquareSecond 3 * Second 1,
        CentiMetrePerSecond 1 == CentiMetrePerSquareSecond 1 * Second 1,
        CentiMetrePerSecond 2 == CentiMetrePerSquareSecond 2 * Second 1,
        CentiMetrePerSecond 3 == CentiMetrePerSquareSecond 3 * Second 1,
        KiloMetrePerHour 1 == KiloMetrePerHourPerSecond 1 * Second 1,
        KiloMetrePerHour 2 == KiloMetrePerHourPerSecond 2 * Second 1,
        KiloMetrePerHour 3 == KiloMetrePerHourPerSecond 3 * Second 1,

        // Velocity == Length / Time
        Metre 1 == MetrePerSecond 1 * Second 1,
        Metre 2 == MetrePerSecond 2 * Second 1,
        Metre 3 == MetrePerSecond 3 * Second 1,
        NauticalMile 1 == Knot 1 * Hour 1,
        NauticalMile 2 == Knot 2 * Hour 1,
        NauticalMile 3 == Knot 3 * Hour 1,
        KiloMetre 1 == KiloMetrePerHour 1 * Hour 1,
        KiloMetre 2 == KiloMetrePerHour 2 * Hour 1,
        KiloMetre 3 == KiloMetrePerHour 3 * Hour 1,
        Mile 1 == MilePerHour 1 * Hour 1,
        Mile 2 == MilePerHour 2 * Hour 1,
        Mile 3 == MilePerHour 3 * Hour 1,
        CentiMetre 1 == CentiMetrePerSecond 1 * Second 1,
        CentiMetre 2 == CentiMetrePerSecond 2 * Second 1,
        CentiMetre 3 == CentiMetrePerSecond 3 * Second 1,

        // AngularAcceleration == Frequency / Time
        RadianPerSecond 1 == RadianPerSquareSecond 1 * Second 1,

        // Frequency == Angle / Time
        Cycle 1 == Hertz 1 * Second 1,
        Radian 1 == RadianPerSecond 1 * Second 1,
        Cycle 1 == CyclePerMinute 1 * Minute 1,

        // KinematicViscosity == Area / Time
        SquareMetre 1 == SquareMetrePerSecond 1 * Second 1,
        SquareMilliMetre 1 == SquareMetrePerSecond 1 * MicroSecond 1,
        SquareMicroMetre 1 == SquareMetrePerSecond 1 * PicoSecond 1,
        SquareCentiMetre 1 == Stoke 1 * Second 1,
        SquareMilliMetre 1 == CentiStoke 1 * Second 1,
        SquareMicroMetre 1 == CentiStoke 1 * MicroSecond 1,
        SquareNanoMetre 1 == CentiStoke 1 * PicoSecond 1,

        // KinematicViscosity == Length * Velocity
        SquareMetrePerSecond 1 == Metre 1 * MetrePerSecond 1,
        SquareMetrePerSecond 1 == HectoMetre 1 * CentiMetrePerSecond 1,
        Stoke 1 == CentiMetre 1 * CentiMetrePerSecond 1,

        // SquareTime == Time * Time
        SquareSecond 1 == Second 1 * __ 1,
        HourSecond 1 == Hour 1 * Second 1,
        HourHour 1 == Hour 1 * __ 1,

        // VolumetricFlowRate == Volume / Time
        CubicMetre 1 == CubicMetrePerSecond 1 * Second 1,
        MilliLitre 1 == MilliLitrePerSecond 1 * Second 1,

        // VolumetricFlowRate == Area * Velocity
        CubicMetrePerSecond 1 == SquareMetre 1 * MetrePerSecond 1,
        MilliLitrePerSecond 1 == SquareCentiMetre 1 * CentiMetrePerSecond 1,

        // Dynamics

        // Properties
        // * Action
        // * AngularMomentum
        // * DynamicViscosity
        // * Energy
        // * EnergyDensity
        // * Force
        // * KinematicViscosity
        // * LinearDensity
        // * Mass
        // * MassDensity
        // * MassFlowRate
        // * MomentOfInertia
        // * Momentum
        // * Power
        // * Pressure
        // * SpecificEnergy
        // * SpecificVolume
        // * SurfaceDensity
        // * SurfaceTension
        // * Torque

        // Action == Energy * Time
        JouleSecond 1 == Joule 1 * Second 1,

        // Action == Power * SquareTime
        JouleSecond 1 == Watt 1 * SquareSecond 1,

        // AngularMomentum == Momentum * Length
        KiloGramSquareMetrePerSecond 1 == KiloGramMetrePerSecond 2 X Metre 2,
        KiloGramSquareMetrePerSecond 3 == KiloGramMetrePerSecond 3 X Metre 3,

        // AngularMomentum == MomentOfInertia / Time
        KiloGramSquareMetre 1 == KiloGramSquareMetrePerSecond 1 * Second 1,

        // DynamicViscosity == Pressure * Time
        PascalSecond 1 == Pascal 1 * Second 1,

        // Energy == Force * Length
        Joule 1 == Newton 1 * Metre 1,
        Joule 1 == Newton 2 * Metre 2,
        Joule 1 == Newton 3 * Metre 3,
        Erg 1 == Dyne 1 * CentiMetre 1,
        Erg 1 == Dyne 2 * CentiMetre 2,
        Erg 1 == Dyne 3 * CentiMetre 3,

        // Energy == Momentum * Velocity
        Joule 1 == NewtonSecond 1 * MetrePerSecond 1,
        Joule 1 == NewtonSecond 2 * MetrePerSecond 2,
        Joule 1 == NewtonSecond 3 * MetrePerSecond 3,
        Erg 1 == DyneSecond 1 * CentiMetrePerSecond 1,
        Erg 1 == DyneSecond 2 * CentiMetrePerSecond 2,
        Erg 1 == DyneSecond 3 * CentiMetrePerSecond 3,

        // Energy == MomentOfInertia / SquareTime
        KiloGramSquareMetre 1 == Joule 1 * SquareSecond 1,

        // EnergyDensity == Energy / Volume
        Joule 1 == JoulePerCubicMetre 1 * CubicMetre 1,

        // Force == Mass * Acceleration
        Newton 1 == KiloGram 1 * MetrePerSquareSecond 1,
        Newton 2 == KiloGram 1 * MetrePerSquareSecond 2,
        Newton 3 == KiloGram 1 * MetrePerSquareSecond 3,
        Dyne 1 == Gram 1 * CentiMetrePerSquareSecond 1,
        Dyne 2 == Gram 1 * CentiMetrePerSquareSecond 2,
        Dyne 3 == Gram 1 * CentiMetrePerSquareSecond 3,
        KiloGramForce 1 == KiloGram 1 * GForce 1,
        KiloGramForce 2 == KiloGram 1 * GForce 2,
        KiloGramForce 3 == KiloGram 1 * GForce 3,

        // LinearDensity == Mass / Length
        KiloGram 1 == KiloGramPerMetre 1 * Metre 1,
        Gram 1 == GramPerCentiMetre 1 * CentiMetre 1,

        // MassDensity == Mass / Volume
        KiloGram 1 == KiloGramPerCubicMetre 1 * CubicMetre 1,
        Gram 1 == GramPerMilliLitre 1 * MilliLitre 1,

        // MassFlowRate == Mass / Time
        KiloGram 1 == KiloGramPerSecond 1 * Second 1,
        Gram 1 == GramPerSecond 1 * Second 1,

        // MomentOfInertia == Mass * Area
        KiloGramSquareMetre 1 == KiloGram 1 * SquareMetre 1,
        GramSquareCentiMetre 1 == Gram 1 * SquareCentiMetre 1,

        // Momentum == Force * Time
        NewtonSecond 1 == Newton 1 * Second 1,
        NewtonSecond 2 == Newton 2 * Second 1,
        NewtonSecond 3 == Newton 3 * Second 1,
        DyneSecond 1 == Dyne 1 * Second 1,
        DyneSecond 2 == Dyne 2 * Second 1,
        DyneSecond 3 == Dyne 3 * Second 1,

        // Momentum == Mass * Velocity
        NewtonSecond 1 == KiloGram 1 * MetrePerSecond 1,
        NewtonSecond 2 == KiloGram 1 * MetrePerSecond 2,
        NewtonSecond 3 == KiloGram 1 * MetrePerSecond 3,
        DyneSecond 1 == Gram 1 * CentiMetrePerSecond 1,
        DyneSecond 2 == Gram 1 * CentiMetrePerSecond 2,
        DyneSecond 3 == Gram 1 * CentiMetrePerSecond 3,

        // Power == Energy / Time
        Joule 1 == Watt 1 * Second 1,
        WattHour 1 == Watt 1 * Hour 1,
        KiloWattHour 1 == KiloWatt 1 * Hour 1,
        Erg 1 == ErgPerSecond 1 * Second 1,

        // Pressure == Force / Area
        Newton 1 == Pascal 1 * SquareMetre 1,
        PoundForce 1 == PoundForcePerSquareInch 1 * SquareInch 1,
        Newton 1 == HectoPascal 1 * SquareDeciMetre 1,

        // SpecificEnergy == Energy / Mass
        Joule 1 == JoulePerKiloGram 1 * KiloGram 1,

        // SpecificVolume == Volume / Mass
        CubicMetre 1 == CubicMetrePerKiloGram 1 * KiloGram 1,

        // SpecificVolume == 1 / MassDensity
        One 1 == CubicMetrePerKiloGram 1 * KiloGramPerCubicMetre 1,

        // SurfaceDensity == Mass / Area
        KiloGram 1 == KiloGramPerSquareMetre 1 * SquareMetre 1,

        // SurfaceTension == Energy / Area
        Joule 1 == JoulePerSquareMetre 1 * SquareMetre 1,

        // Torque == Force * Length
        NewtonMetre 1 == Newton 2 X Metre 2,
        NewtonMetre 3 == Newton 3 X Metre 3,
        PoundFoot 1 == PoundForce 2 X Foot 2,
        PoundFoot 3 == PoundForce 3 X Foot 3,
        PoundInch 1 == PoundForce 2 X Inch 2,
        PoundInch 3 == PoundForce 3 X Inch 3,

        // Thermodynamics

        // Properties
        // * Entropy
        // * SpecificHeatCapacity
        // * Temperature
        // * ThermalConductivity

        // Entropy == Energy / Temperature
        Joule 1 == JoulePerKelvin 1 * Kelvin 1,

        // SpecificHeatCapacity == Entropy / Mass
        JoulePerKelvin 1 == JoulePerKiloGramPerKelvin 1 * KiloGram 1,

        // TODO  ThermalConductivity == Power / Length / Temperature
        // TODO  WattPerMetrePerKelvin == ?,

        // Chemistry

        // Properties
        // * CatalyticActivity
        // * ChemicalPotential
        // * MolarConcentration
        // * MolarHeatCapacity
        // * ReactionRate

        // CatalyticActivity == Amount / Time
        Mole 1 == Katal 1 * Second 1,

        // ChemicalPotential == Energy / Amount
        Joule 1 == JoulePerMole 1 * Mole 1,

        // MolarConcentration == Amount / Volume
        Mole 1 == MolePerCubicMetre 1 * CubicMetre 1,

        // MolarHeatCapacity == ChemicalPotential / Temperature
        JoulePerMole 1 == JoulePerKelvinPerMole 1 * Kelvin 1,

        // ReactionRate == MolarConcentration / Time
        MolePerCubicMetre 1 == MolePerCubicMetrePerSecond 1 * Second 1,

        // ReactionRate == CatalyticActivity / Volume
        Katal 1 == MolePerCubicMetrePerSecond 1 * CubicMetre 1,

        // Radioactivity

        // Properties
        // * DoseEquivalent
        // * RadioactiveActivity
        // * RadioactiveDose
        // * RadioactiveDoseRate

        // RadioactiveDoseRate == RadioactiveDose / Time
        Gray 1 == GrayPerSecond 1 * Second 1,

        // Optics

        // Properties
        // * Illuminance
        // * Irradiance
        // * Luminance
        // * LuminousFlux
        // * LuminousIntensity
        // * Radiance
        // * RadiantIntensity

        // Illuminance == LuminousFlux / Area
        Lumen 1 == Lux 1 * SquareMetre 1,
        Lux 1 == CandelaPerSquareMetre 1 * Steradian 1,
        Lumen 1 == Phot 1 * SquareCentiMetre 1,
        Phot 1 == Stilb 1 * Steradian 1,
        Lumen 1 == FootCandle 1 * SquareFoot 1,
        FootCandle 1 == CandelaPerSquareFoot 1 * Steradian 1,

        // Irradiance == Power / Area
        Watt 1 == WattPerSquareMetre 1 * SquareMetre 1,

        // Luminance == LuminousIntensity / Area
        Candela 1 == CandelaPerSquareMetre 1 * SquareMetre 1,
        Candela 1 == Stilb 1 * SquareCentiMetre 1,
        Candela 1 == CandelaPerSquareFoot 1 * SquareFoot 1,

        // LuminousFlux == LuminousIntensity * SolidAngle
        Lumen 1 == Candela 1 * Steradian 1,

        // Radiance == RadiantIntensity / Area
        WattPerSteradian 1 == WattPerSquareMetrePerSteradian 1 * SquareMetre 1,

        // Radiance == Irradiance / SolidAngle
        WattPerSquareMetre 1 == WattPerSquareMetrePerSteradian 1 * Steradian 1,

        // RadiantIntensity == Power / SolidAngle
        Watt 1 == WattPerSteradian 1 * Steradian 1,

        // Electricity

        // Properties
        // * Capacitance
        // * CurrentDensity
        // * ElectricalConductance
        // * ElectricalConductivity
        // * ElectricalResistance
        // * ElectricCharge
        // * ElectricChargeDensity
        // * ElectricCurrent
        // * ElectricDisplacement
        // * ElectricFieldStrength
        // * ElectricPotential
        // * LinearElectricChargeDensity
        // * Permittivity

        // Capacitance == ElectricCharge / ElectricPotential
        Coulomb 1 == Farad 1 * Volt 1,
        MilliCoulomb 1 == MilliFarad 1 * Volt 1,
        Coulomb 1 == MilliFarad 1 * KiloVolt 1,
        MicroCoulomb 1 == MicroFarad 1 * Volt 1,
        MilliCoulomb 1 == MicroFarad 1 * KiloVolt 1,
        NanoCoulomb 1 == NanoFarad 1 * Volt 1,
        MicroCoulomb 1 == NanoFarad 1 * KiloVolt 1,
        PicoCoulomb 1 == PicoFarad 1 * Volt 1,
        NanoCoulomb 1 == PicoFarad 1 * KiloVolt 1,

        // CurrentDensity == ElectricCurrent * Area
        Ampere 1 == AmperePerSquareMetre 1 * SquareMetre 1,

        // ElectricalConductance == ElectricCurrent / ElectricPotential
        Ampere 1 == Siemens 1 * Volt 1,
        MilliAmpere 1 == Siemens 1 * KiloVolt 1,

        // ElectricalConductance == 1 / ElectricalResistance
        One 1 == Siemens 1 * Ohm 1,

        // ElectricalConductivity == ElectricalConductance / Length
        Siemens 1 == SiemensPerMetre 1 * Metre 1,

        // ElectricalResistance == ElectricPotential / ElectricCurrent
        Volt 1 == Ohm 1 * Ampere 1,

        // ElectricCurrent == ElectricCharge / Time
        Coulomb 1 == Ampere 1 * Second 1,
        MilliCoulomb 1 == Ampere 1 * MilliSecond 1,
        MilliCoulomb 1 == MilliAmpere 1 * Second 1,
        MicroCoulomb 1 == MilliAmpere 1 * MilliSecond 1,
        MicroCoulomb 1 == MicroAmpere 1 * Second 1,

        // ElectricChargeDensity == ElectricCharge / Volume
        Coulomb 1 == CoulombPerCubicMetre 1 * CubicMetre 1,
        MilliCoulomb 1 == CoulombPerCubicMetre 1 * Litre 1,
        MicroCoulomb 1 == CoulombPerCubicMetre 1 * MilliLitre 1,

        // ElectricDisplacement == ElectricCharge / Area
        Coulomb 1 == CoulombPerSquareMetre 1 * SquareMetre 1,
        MicroCoulomb 1 == CoulombPerSquareMetre 1 * SquareMilliMetre 1,

        // ElectricFieldStrength == ElectricPotential / Length
        Volt 1 == VoltPerMetre 1 * Metre 1,

        // ElectricFieldStrength == Force / ElectricCharge
        Newton 1 == NewtonPerCoulomb 1 * Coulomb 1,

        // ElectricPotential == Power / ElectricCurrent
        Watt 1 == Volt 1 * Ampere 1,
        MilliWatt 1 == Volt 1 * MilliAmpere 1,
        Watt 1 == KiloVolt 1 * MilliAmpere 1,
        MilliWatt 1 == KiloVolt 1 * MicroAmpere 1,
        KiloWatt 1 == KiloVolt 1 * Ampere 1,
        MilliWatt 1 == MilliVolt 1 * Ampere 1,

        // LinearElectricChargeDensity == ElectricCharge / Length
        Coulomb 1 == CoulombPerMetre 1 * Metre 1,
        MilliCoulomb 1 == CoulombPerMetre 1 * MilliMetre 1,
        MicroCoulomb 1 == CoulombPerMetre 1 * MicroMetre 1,

        // Permittivity == Capacitance / Length
        Farad 1 == FaradPerMetre 1 * Metre 1,
        MilliFarad 1 == FaradPerMetre 1 * MilliMetre 1,
        MicroFarad 1 == FaradPerMetre 1 * MicroMetre 1,
        NanoFarad 1 == FaradPerMetre 1 * NanoMetre 1,

        // Magnetism

        // Properties
        // * Inductance
        // * MagneticFieldStrength
        // * MagneticFlux
        // * MagneticFluxDensity
        // * MagneticPermeability
        // * MagneticReluctance

        // Inductance == MagneticFlux / ElectricCurrent
        Weber 1 == Henry 1 * Ampere 1,

        // MagneticFieldStrength == ElectricCurrent / Length
        Ampere 1 == AmperePerMetre 1 * Metre 1,
        MilliAmpere 1 == AmperePerMetre 1 * MilliMetre 1,
        MicroAmpere 1 == AmperePerMetre 1 * MicroMetre 1,

        // TODO  MagneticFlux == Mass * Area / SquareTime / Current

        // ElectricFieldStrength == Velocity X MagneticFlux
        VoltPerMetre 1 == MetrePerSecond 1 * Weber 1,
        VoltPerMetre 1 == MetrePerSecond 2 X Weber 2,
        VoltPerMetre 3 == MetrePerSecond 3 X Weber 3,

        // MagneticFluxDensity == MagneticFlux / Area
        // TODO  MagneticFluxDensity == Mass / SquareTime / Current
        // TODO  MagneticFluxDensity == Force / Length / Current

        // MagneticPermeability == Inductance / Length

        // MagneticReluctance == 1 / Inductance
        One 1 == InverseHenry 1 * Henry 1,

        // Others

        // Properties
        // * Amount
        // * Dimensionless

        // Dimensionless == Dimensionless * Dimensionless
        One 1 == One 2 * __ 2,
        One 1 == One 2 X __ 2,
        One 1 == One 3 * __ 3,
        One 3 == One 3 X __ 3,
        // N.B.: The following definition is not allowed, because already defined:
        // ```
        // One == One * One,
        // ```
    ]
}

// Property: acceleration
pub struct Acceleration;
impl VectorProperty for Acceleration {}

pub struct MetrePerSquareSecond;
impl MeasurementUnit for MetrePerSquareSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s\u{b2}";
}

pub struct CentiMetrePerSquareSecond;
impl MeasurementUnit for CentiMetrePerSquareSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1e-2;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s\u{b2}";
}

pub struct GForce;
impl MeasurementUnit for GForce {
    type Property = Acceleration;
    const RATIO: f64 = 9.80665;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g";
}

pub struct KiloMetrePerHourPerSecond;
impl MeasurementUnit for KiloMetrePerHourPerSecond {
    type Property = Acceleration;
    const RATIO: f64 = 1000. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km/h/s";
}

// Property: action
pub struct Action;

pub struct JouleSecond;
impl MeasurementUnit for JouleSecond {
    type Property = Action;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J\u{b7}s";
}

// Property: amount of substance, count
pub struct Amount;

#[allow(dead_code)]
pub struct Unit;
impl MeasurementUnit for Unit {
    type Property = Amount;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " u.";
}

#[allow(dead_code)]
pub struct Dozen;
impl MeasurementUnit for Dozen {
    type Property = Amount;
    const RATIO: f64 = 12.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dz.";
}

pub struct Mole;
impl MeasurementUnit for Mole {
    type Property = Amount;
    const RATIO: f64 = 6.0221413e23;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mol";
}

// Property: angle
pub struct Cycle;
impl MeasurementUnit for Cycle {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cycles";
}
impl AngleMeasurementUnit for Cycle {
    const CYCLE_FRACTION: f64 = 1.;
}

#[allow(dead_code)]
pub struct Gradian;
impl MeasurementUnit for Gradian {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " grad";
}
impl AngleMeasurementUnit for Gradian {
    const CYCLE_FRACTION: f64 = 400.;
}

#[allow(dead_code)]
pub struct Degree;
impl MeasurementUnit for Degree {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg";
}
impl AngleMeasurementUnit for Degree {
    const CYCLE_FRACTION: f64 = 360.;
}

#[allow(dead_code)]
pub struct ArcMinute;
impl MeasurementUnit for ArcMinute {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 360. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg'";
}
impl AngleMeasurementUnit for ArcMinute {
    const CYCLE_FRACTION: f64 = 360. * 60.;
}

#[allow(dead_code)]
pub struct ArcSecond;
impl MeasurementUnit for ArcSecond {
    type Property = Angle;
    const RATIO: f64 = core::f64::consts::TAU / 360. / 60. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg\"";
}
impl AngleMeasurementUnit for ArcSecond {
    const CYCLE_FRACTION: f64 = 360. * 60. * 60.;
}

// Property: angular acceleration
pub struct AngularAcceleration;

pub struct RadianPerSquareSecond;
impl MeasurementUnit for RadianPerSquareSecond {
    type Property = AngularAcceleration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/s\u{b2}";
}

// Property: angular momentum, spin
pub struct AngularMomentum;
impl VectorProperty for AngularMomentum {}

pub struct KiloGramSquareMetrePerSecond;
impl MeasurementUnit for KiloGramSquareMetrePerSecond {
    type Property = AngularMomentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m\u{b2}/s";
}

#[allow(dead_code)]
pub struct GramSquareCentiMetrePerSecond;
impl MeasurementUnit for GramSquareCentiMetrePerSecond {
    type Property = AngularMomentum;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm\u{b2}/s";
}

// Property: area
pub struct Area;

pub struct SquareMetre;
impl MeasurementUnit for SquareMetre {
    type Property = Area;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b2}";
}

pub struct SquareKiloMetre;
impl MeasurementUnit for SquareKiloMetre {
    type Property = Area;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km\u{b2}";
}

pub struct Hectare;
impl MeasurementUnit for Hectare {
    type Property = Area;
    const RATIO: f64 = 1e4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ha";
}

pub struct Are;
impl MeasurementUnit for Are {
    type Property = Area;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " are";
}

pub struct SquareDeciMetre;
impl MeasurementUnit for SquareDeciMetre {
    type Property = Area;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dm\u{b2}";
}

pub struct SquareCentiMetre;
impl MeasurementUnit for SquareCentiMetre {
    type Property = Area;
    const RATIO: f64 = 1e-4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm\u{b2}";
}

pub struct SquareMilliMetre;
impl MeasurementUnit for SquareMilliMetre {
    type Property = Area;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm\u{b2}";
}

pub struct SquareMicroMetre;
impl MeasurementUnit for SquareMicroMetre {
    type Property = Area;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}m\u{b2}";
}

pub struct SquareNanoMetre;
impl MeasurementUnit for SquareNanoMetre {
    type Property = Area;
    const RATIO: f64 = 1e-18;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nm\u{b2}";
}

pub struct SquareInch;
impl MeasurementUnit for SquareInch {
    type Property = Area;
    const RATIO: f64 = 0.0254 * 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in\u{b2}";
}

pub struct SquareFoot;
impl MeasurementUnit for SquareFoot {
    type Property = Area;
    const RATIO: f64 = 0.3048 * 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft\u{b2}";
}

pub struct SquareYard;
impl MeasurementUnit for SquareYard {
    type Property = Area;
    const RATIO: f64 = 0.9144 * 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd\u{b2}";
}

pub struct SquareMile;
impl MeasurementUnit for SquareMile {
    type Property = Area;
    const RATIO: f64 = 1609. * 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi\u{b2}";
}

// Property: capacitance
pub struct Capacitance;

pub struct Farad;
impl MeasurementUnit for Farad {
    type Property = Capacitance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " F";
}

pub struct MilliFarad;
impl MeasurementUnit for MilliFarad {
    type Property = Capacitance;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mF";
}

pub struct MicroFarad;
impl MeasurementUnit for MicroFarad {
    type Property = Capacitance;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}F";
}

pub struct NanoFarad;
impl MeasurementUnit for NanoFarad {
    type Property = Capacitance;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nF";
}

pub struct PicoFarad;
impl MeasurementUnit for PicoFarad {
    type Property = Capacitance;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pF";
}

// Property: catalytic activity
pub struct CatalyticActivity;

pub struct Katal;
impl MeasurementUnit for Katal {
    type Property = CatalyticActivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kat";
}

// Property: chemical potential, molar energy
pub struct ChemicalPotential;

pub struct JoulePerMole;
impl MeasurementUnit for JoulePerMole {
    type Property = ChemicalPotential;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/mol";
}

// Property: current density
pub struct CurrentDensity;
impl VectorProperty for CurrentDensity {}

pub struct AmperePerSquareMetre;
impl MeasurementUnit for AmperePerSquareMetre {
    type Property = CurrentDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A/m\u{b2}";
}

// Property: dimensionless
#[allow(dead_code)]
pub struct Mach;
impl MeasurementUnit for Mach {
    type Property = Dimensionless;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mach";
}

// Property: dose equivalent
pub struct DoseEquivalent;

#[allow(dead_code)]
pub struct Sievert;
impl MeasurementUnit for Sievert {
    type Property = DoseEquivalent;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Sv";
}

#[allow(dead_code)]
pub struct Rem;
impl MeasurementUnit for Rem {
    type Property = DoseEquivalent;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rem";
}

// Property: dynamic viscosity, absolute viscosity
pub struct DynamicViscosity;

pub struct PascalSecond;
impl MeasurementUnit for PascalSecond {
    type Property = DynamicViscosity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Pa\u{b7}s";
}

// Property: electrical conductance, electric susceptance, electric admittance
pub struct ElectricalConductance;

pub struct Siemens;
impl MeasurementUnit for Siemens {
    type Property = ElectricalConductance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " S";
}

// Property: electrical conductivity
pub struct ElectricalConductivity;

pub struct SiemensPerMetre;
impl MeasurementUnit for SiemensPerMetre {
    type Property = ElectricalConductivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " S/m";
}

// Property: electrical resistance, electrical impedance
pub struct ElectricalResistance;

pub struct Ohm;
impl MeasurementUnit for Ohm {
    type Property = ElectricalResistance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{3a9}";
}

#[allow(dead_code)]
pub struct MilliOhm;
impl MeasurementUnit for MilliOhm {
    type Property = ElectricalResistance;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{3a9}";
}

#[allow(dead_code)]
pub struct KiloOhm;
impl MeasurementUnit for KiloOhm {
    type Property = ElectricalResistance;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " k\u{3a9}";
}

// Property: electrical resistivity
pub struct ElectricalResistivity;

#[allow(dead_code)]
pub struct OhmMetre;
impl MeasurementUnit for OhmMetre {
    type Property = ElectricalResistivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{3a9}\u{b7}m";
}

// Property: electric charge
pub struct ElectricCharge;

pub struct Coulomb;
impl MeasurementUnit for Coulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C";
}

pub struct MilliCoulomb;
impl MeasurementUnit for MilliCoulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mC";
}

pub struct MicroCoulomb;
impl MeasurementUnit for MicroCoulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}C";
}

pub struct NanoCoulomb;
impl MeasurementUnit for NanoCoulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nC";
}

pub struct PicoCoulomb;
impl MeasurementUnit for PicoCoulomb {
    type Property = ElectricCharge;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pC";
}

// Property: electric charge density
pub struct ElectricChargeDensity;

pub struct CoulombPerCubicMetre;
impl MeasurementUnit for CoulombPerCubicMetre {
    type Property = ElectricChargeDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C/m\u{b3}";
}

// Property: electric current
pub struct ElectricCurrent;

pub struct Ampere;
impl MeasurementUnit for Ampere {
    type Property = ElectricCurrent;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A";
}

pub struct MilliAmpere;
impl MeasurementUnit for MilliAmpere {
    type Property = ElectricCurrent;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mA";
}

pub struct MicroAmpere;
impl MeasurementUnit for MicroAmpere {
    type Property = ElectricCurrent;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}A";
}

// Property: electric displacement, surface electric charge density
pub struct ElectricDisplacement;

pub struct CoulombPerSquareMetre;
impl MeasurementUnit for CoulombPerSquareMetre {
    type Property = ElectricDisplacement;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C/m\u{b2}";
}

// Property: electric field strength
pub struct ElectricFieldStrength;
impl VectorProperty for ElectricFieldStrength {}

pub struct VoltPerMetre;
impl MeasurementUnit for VoltPerMetre {
    type Property = ElectricFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " V/m";
}

pub struct NewtonPerCoulomb;
impl MeasurementUnit for NewtonPerCoulomb {
    type Property = ElectricFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N/C";
}

// Property: electric potential
pub struct ElectricPotential;

pub struct Volt;
impl MeasurementUnit for Volt {
    type Property = ElectricPotential;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " V";
}

pub struct KiloVolt;
impl MeasurementUnit for KiloVolt {
    type Property = ElectricPotential;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kV";
}

pub struct MilliVolt;
impl MeasurementUnit for MilliVolt {
    type Property = ElectricPotential;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mV";
}

#[allow(dead_code)]
pub struct MicroVolt;
impl MeasurementUnit for MicroVolt {
    type Property = ElectricPotential;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}V";
}

// Property: energy, work, heat
pub struct Energy;

pub struct Joule;
impl MeasurementUnit for Joule {
    type Property = Energy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J";
}

pub struct Erg;
impl MeasurementUnit for Erg {
    type Property = Energy;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " erg";
}

pub struct WattHour;
impl MeasurementUnit for WattHour {
    type Property = Energy;
    const RATIO: f64 = 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W\u{b7}h";
}

pub struct KiloWattHour;
impl MeasurementUnit for KiloWattHour {
    type Property = Energy;
    const RATIO: f64 = 3.6e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kW\u{b7}h";
}

#[allow(dead_code)]
pub struct MegaWattHour;
impl MeasurementUnit for MegaWattHour {
    type Property = Energy;
    const RATIO: f64 = 3.6e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MW\u{b7}h";
}

#[allow(dead_code)]
pub struct Calorie;
impl MeasurementUnit for Calorie {
    type Property = Energy;
    const RATIO: f64 = 4.187;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cal";
}

#[allow(dead_code)]
pub struct KiloCalorie;
impl MeasurementUnit for KiloCalorie {
    type Property = Energy;
    const RATIO: f64 = 4187.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kcal";
}

#[allow(dead_code)]
pub struct ElectronVolt;
impl MeasurementUnit for ElectronVolt {
    type Property = Energy;
    const RATIO: f64 = 1.602176634e-19;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " eV";
}

#[allow(dead_code)]
pub struct KiloElectronVolt;
impl MeasurementUnit for KiloElectronVolt {
    type Property = Energy;
    const RATIO: f64 = 1.602176634e-16;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " keV";
}

#[allow(dead_code)]
pub struct MegaElectronVolt;
impl MeasurementUnit for MegaElectronVolt {
    type Property = Energy;
    const RATIO: f64 = 1.602176634e-13;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MeV";
}

#[allow(dead_code)]
pub struct GigaElectronVolt;
impl MeasurementUnit for GigaElectronVolt {
    type Property = Energy;
    const RATIO: f64 = 1.602176634e-10;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GeV";
}

#[allow(dead_code)]
pub struct TeraElectronVolt;
impl MeasurementUnit for TeraElectronVolt {
    type Property = Energy;
    const RATIO: f64 = 1.602176634e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TeV";
}

// Property: energy density
pub struct EnergyDensity;

pub struct JoulePerCubicMetre;
impl MeasurementUnit for JoulePerCubicMetre {
    type Property = EnergyDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/m\u{b3}";
}

// Property: entropy, heat capacity
pub struct Entropy;

pub struct JoulePerKelvin;
impl MeasurementUnit for JoulePerKelvin {
    type Property = Entropy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/\u{b0}K";
}

// Property: force, weight
pub struct Force;
impl VectorProperty for Force {}

pub struct Newton;
impl MeasurementUnit for Newton {
    type Property = Force;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N";
}

pub struct Dyne;
impl MeasurementUnit for Dyne {
    type Property = Force;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dyn";
}

pub struct KiloGramForce;
impl MeasurementUnit for KiloGramForce {
    type Property = Force;
    const RATIO: f64 = 9.80665;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kgf";
}

pub struct PoundForce;
impl MeasurementUnit for PoundForce {
    type Property = Force;
    const RATIO: f64 = 4.448222;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lbf";
}

#[allow(dead_code)]
pub struct Poundal;
impl MeasurementUnit for Poundal {
    type Property = Force;
    const RATIO: f64 = 0.138255;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pdl";
}

// Property: frequency, angular speed, angular velocity
pub struct Frequency;

pub struct Hertz;
impl MeasurementUnit for Hertz {
    type Property = Frequency;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Hz";
}

#[allow(dead_code)]
pub struct CyclePerSecond;
impl MeasurementUnit for CyclePerSecond {
    type Property = Frequency;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " c/s";
}

#[allow(dead_code)]
pub struct KiloHertz;
impl MeasurementUnit for KiloHertz {
    type Property = Frequency;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kHz";
}

#[allow(dead_code)]
pub struct MegaHertz;
impl MeasurementUnit for MegaHertz {
    type Property = Frequency;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MHz";
}

#[allow(dead_code)]
pub struct GigaHertz;
impl MeasurementUnit for GigaHertz {
    type Property = Frequency;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GHz";
}

pub struct RadianPerSecond;
impl MeasurementUnit for RadianPerSecond {
    type Property = Frequency;
    const RATIO: f64 = 1. / core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/s";
}

pub struct CyclePerMinute;
impl MeasurementUnit for CyclePerMinute {
    type Property = Frequency;
    const RATIO: f64 = 1. / 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rpm";
}

pub struct Illuminance;

pub struct Lux;
impl MeasurementUnit for Lux {
    type Property = Illuminance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lx";
}

pub struct Phot;
impl MeasurementUnit for Phot {
    type Property = Illuminance;
    const RATIO: f64 = 10000.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " phot";
}

pub struct FootCandle;
impl MeasurementUnit for FootCandle {
    type Property = Illuminance;
    const RATIO: f64 = 10.764;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " fc";
}

// Property: inductance
pub struct Inductance;

pub struct Henry;
impl MeasurementUnit for Henry {
    type Property = Inductance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " H";
}

// Property: information
pub struct Information;

pub struct Bit;
impl MeasurementUnit for Bit {
    type Property = Information;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " b";
}

pub struct Byte;
impl MeasurementUnit for Byte {
    type Property = Information;
    const RATIO: f64 = 8.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " B";
}

pub struct KiloBit;
impl MeasurementUnit for KiloBit {
    type Property = Information;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kb";
}

pub struct KiloByte;
impl MeasurementUnit for KiloByte {
    type Property = Information;
    const RATIO: f64 = 8e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kB";
}

pub struct KibiBit;
impl MeasurementUnit for KibiBit {
    type Property = Information;
    const RATIO: f64 = 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kib";
}

pub struct KibiByte;
impl MeasurementUnit for KibiByte {
    type Property = Information;
    const RATIO: f64 = 8. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kiB";
}

pub struct MegaBit;
impl MeasurementUnit for MegaBit {
    type Property = Information;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mb";
}

pub struct MegaByte;
impl MeasurementUnit for MegaByte {
    type Property = Information;
    const RATIO: f64 = 8e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MB";
}

pub struct MebiBit;
impl MeasurementUnit for MebiBit {
    type Property = Information;
    const RATIO: f64 = 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mib";
}

pub struct MebiByte;
impl MeasurementUnit for MebiByte {
    type Property = Information;
    const RATIO: f64 = 8. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MiB";
}

pub struct GigaBit;
impl MeasurementUnit for GigaBit {
    type Property = Information;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gb";
}

pub struct GigaByte;
impl MeasurementUnit for GigaByte {
    type Property = Information;
    const RATIO: f64 = 8e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GB";
}

pub struct GibiBit;
impl MeasurementUnit for GibiBit {
    type Property = Information;
    const RATIO: f64 = 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gib";
}

pub struct GibiByte;
impl MeasurementUnit for GibiByte {
    type Property = Information;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GiB";
}

pub struct TeraBit;
impl MeasurementUnit for TeraBit {
    type Property = Information;
    const RATIO: f64 = 1e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tb";
}

pub struct TeraByte;
impl MeasurementUnit for TeraByte {
    type Property = Information;
    const RATIO: f64 = 8e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TB";
}

pub struct TebiBit;
impl MeasurementUnit for TebiBit {
    type Property = Information;
    const RATIO: f64 = 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tib";
}

pub struct TebiByte;
impl MeasurementUnit for TebiByte {
    type Property = Information;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TiB";
}

// Property: information rate
pub struct InformationRate;

pub struct BitPerSecond;
impl MeasurementUnit for BitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " b/s";
}

pub struct BytePerSecond;
impl MeasurementUnit for BytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " B/s";
}

pub struct KiloBitPerSecond;
impl MeasurementUnit for KiloBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kb/s";
}

pub struct KiloBytePerSecond;
impl MeasurementUnit for KiloBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kB/s";
}

pub struct KibiBitPerSecond;
impl MeasurementUnit for KibiBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kib/s";
}

pub struct KibiBytePerSecond;
impl MeasurementUnit for KibiBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kiB/s";
}

pub struct MegaBitPerSecond;
impl MeasurementUnit for MegaBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mb/s";
}

pub struct MegaBytePerSecond;
impl MeasurementUnit for MegaBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MB/s";
}

pub struct MebiBitPerSecond;
impl MeasurementUnit for MebiBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mib/s";
}

pub struct MebiBytePerSecond;
impl MeasurementUnit for MebiBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MiB/s";
}

pub struct GigaBitPerSecond;
impl MeasurementUnit for GigaBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gb/s";
}

pub struct GigaBytePerSecond;
impl MeasurementUnit for GigaBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GB/s";
}

pub struct GibiBitPerSecond;
impl MeasurementUnit for GibiBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gib/s";
}

pub struct GibiBytePerSecond;
impl MeasurementUnit for GibiBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GiB/s";
}

pub struct TeraBitPerSecond;
impl MeasurementUnit for TeraBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tb/s";
}

pub struct TeraBytePerSecond;
impl MeasurementUnit for TeraBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8e12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TB/s";
}

pub struct TebiBitPerSecond;
impl MeasurementUnit for TebiBitPerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Tib/s";
}

pub struct TebiBytePerSecond;
impl MeasurementUnit for TebiBytePerSecond {
    type Property = InformationRate;
    const RATIO: f64 = 8. * 1024. * 1024. * 1024. * 1024.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " TiB/s";
}

// Property: irradiance, heat flux density
pub struct Irradiance;

pub struct WattPerSquareMetre;
impl MeasurementUnit for WattPerSquareMetre {
    type Property = Irradiance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/m\u{b2}";
}

// Property: kinematic viscosity
pub struct KinematicViscosity;

pub struct SquareMetrePerSecond;
impl MeasurementUnit for SquareMetrePerSecond {
    type Property = KinematicViscosity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b2}/s";
}

pub struct Stoke;
impl MeasurementUnit for Stoke {
    type Property = KinematicViscosity;
    const RATIO: f64 = 1e-4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " St";
}

pub struct CentiStoke;
impl MeasurementUnit for CentiStoke {
    type Property = KinematicViscosity;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cSt";
}

// Property: length, width, height, depth, space, wavelength
pub struct Length;
impl VectorProperty for Length {}

pub struct Metre;
impl MeasurementUnit for Metre {
    type Property = Length;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m";
}

#[allow(dead_code)]
pub struct AstronomicalUnit;
impl MeasurementUnit for AstronomicalUnit {
    type Property = Length;
    const RATIO: f64 = 149597870691.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " a.u.";
}

#[allow(dead_code)]
pub struct Parsec;
impl MeasurementUnit for Parsec {
    type Property = Length;
    const RATIO: f64 = 3.0856775813e16;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " psc";
}

#[allow(dead_code)]
pub struct LightYear;
impl MeasurementUnit for LightYear {
    type Property = Length;
    const RATIO: f64 = 31557600. * 2.99792458e8;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ly";
}

pub struct KiloMetre;
impl MeasurementUnit for KiloMetre {
    type Property = Length;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km";
}

pub struct HectoMetre;
impl MeasurementUnit for HectoMetre {
    type Property = Length;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " hm";
}

pub struct DecaMetre;
impl MeasurementUnit for DecaMetre {
    type Property = Length;
    const RATIO: f64 = 10.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dam";
}

pub struct DeciMetre;
impl MeasurementUnit for DeciMetre {
    type Property = Length;
    const RATIO: f64 = 0.1;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dm";
}

pub struct CentiMetre;
impl MeasurementUnit for CentiMetre {
    type Property = Length;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm";
}

pub struct MilliMetre;
impl MeasurementUnit for MilliMetre {
    type Property = Length;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm";
}

pub struct MicroMetre;
impl MeasurementUnit for MicroMetre {
    type Property = Length;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}m";
}

pub struct NanoMetre;
impl MeasurementUnit for NanoMetre {
    type Property = Length;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nm";
}

#[allow(dead_code)]
pub struct Angstrom;
impl MeasurementUnit for Angstrom {
    type Property = Length;
    const RATIO: f64 = 1e-10;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{212b}";
}

pub struct Inch;
impl MeasurementUnit for Inch {
    type Property = Length;
    const RATIO: f64 = 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in";
}

pub struct Foot;
impl MeasurementUnit for Foot {
    type Property = Length;
    const RATIO: f64 = 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft";
}

pub struct Yard;
impl MeasurementUnit for Yard {
    type Property = Length;
    const RATIO: f64 = 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd";
}

pub struct Mile;
impl MeasurementUnit for Mile {
    type Property = Length;
    const RATIO: f64 = 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi";
}

pub struct NauticalMile;
impl MeasurementUnit for NauticalMile {
    type Property = Length;
    const RATIO: f64 = 1852.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " naut.mi";
}

// Property: linear density
pub struct LinearDensity;

pub struct KiloGramPerMetre;
impl MeasurementUnit for KiloGramPerMetre {
    type Property = LinearDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/m";
}

pub struct GramPerCentiMetre;
impl MeasurementUnit for GramPerCentiMetre {
    type Property = LinearDensity;
    const RATIO: f64 = 1e-1;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g/cm";
}

// Property: linear electric charge density
pub struct LinearElectricChargeDensity;

pub struct CoulombPerMetre;
impl MeasurementUnit for CoulombPerMetre {
    type Property = LinearElectricChargeDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " C/m";
}

// Property: luminance
pub struct Luminance;

pub struct CandelaPerSquareMetre;
impl MeasurementUnit for CandelaPerSquareMetre {
    type Property = Luminance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cd/m\u{b2}";
}

#[allow(dead_code)]
pub struct Nit;
impl MeasurementUnit for Nit {
    type Property = Luminance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nt";
}

pub struct Stilb;
impl MeasurementUnit for Stilb {
    type Property = Luminance;
    const RATIO: f64 = 10000.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " stilb";
}

pub struct CandelaPerSquareFoot;
impl MeasurementUnit for CandelaPerSquareFoot {
    type Property = Luminance;
    const RATIO: f64 = 10.764;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " stilb";
}

// Property: luminous flux, luminous power
pub struct LuminousFlux;
impl VectorProperty for LuminousFlux {}

pub struct Lumen;
impl MeasurementUnit for Lumen {
    type Property = LuminousFlux;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lm";
}

// Property: luminous intensity
pub struct LuminousIntensity;

pub struct Candela;
impl MeasurementUnit for Candela {
    type Property = LuminousIntensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cd";
}

// Property: magnetic field strength, magnetic field intensity, magnetization
pub struct MagneticFieldStrength;
impl VectorProperty for MagneticFieldStrength {}

pub struct AmperePerMetre;
impl MeasurementUnit for AmperePerMetre {
    type Property = MagneticFieldStrength;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " A/m";
}

// Property: magnetic flux
pub struct MagneticFlux;

pub struct Weber;
impl MeasurementUnit for Weber {
    type Property = MagneticFlux;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Wb";
}

// Property: magnetic flux density
pub struct MagneticFluxDensity;
impl VectorProperty for MagneticFluxDensity {}

#[allow(dead_code)]
pub struct Tesla;
impl MeasurementUnit for Tesla {
    type Property = MagneticFluxDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " T";
}

#[allow(dead_code)]
pub struct Gauss;
impl MeasurementUnit for Gauss {
    type Property = MagneticFluxDensity;
    const RATIO: f64 = 1e-4;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " G";
}

// Property: magnetic permeability
pub struct MagneticPermeability;

#[allow(dead_code)]
pub struct HenryPerMetre;
impl MeasurementUnit for HenryPerMetre {
    type Property = MagneticPermeability;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " H/m";
}

// Property: magnetic reluctance, magnetic resistance
pub struct MagneticReluctance;

pub struct InverseHenry;
impl MeasurementUnit for InverseHenry {
    type Property = MagneticReluctance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " 1/H";
}

// Property: mass
pub struct Mass;

pub struct KiloGram;
impl MeasurementUnit for KiloGram {
    type Property = Mass;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg";
}

pub struct Tonne;
impl MeasurementUnit for Tonne {
    type Property = Mass;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
}

#[allow(dead_code)]
pub type MetricTon = Tonne;

#[allow(dead_code)]
pub struct MegaGram;
impl MeasurementUnit for MegaGram {
    type Property = Mass;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Mg";
}

#[allow(dead_code)]
pub struct HectoGram;
impl MeasurementUnit for HectoGram {
    type Property = Mass;
    const RATIO: f64 = 0.1;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " hg";
}

#[allow(dead_code)]
pub struct DecaGram;
impl MeasurementUnit for DecaGram {
    type Property = Mass;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dag";
}

pub struct Gram;
impl MeasurementUnit for Gram {
    type Property = Mass;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g";
}

#[allow(dead_code)]
pub struct MilliGram;
impl MeasurementUnit for MilliGram {
    type Property = Mass;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mg";
}

#[allow(dead_code)]
pub struct MicroGram;
impl MeasurementUnit for MicroGram {
    type Property = Mass;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}g";
}

#[allow(dead_code)]
pub struct NanoGram;
impl MeasurementUnit for NanoGram {
    type Property = Mass;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ng";
}

pub struct ImperialTon;
impl MeasurementUnit for ImperialTon {
    type Property = Mass;
    const RATIO: f64 = 1016.0469;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
}

#[allow(dead_code)]
pub type LongTon = ImperialTon;

pub struct USTon;
impl MeasurementUnit for USTon {
    type Property = Mass;
    const RATIO: f64 = 907.18474;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " t";
}

#[allow(dead_code)]
pub type ShortTon = USTon;

#[allow(dead_code)]
pub struct Stone;
impl MeasurementUnit for Stone {
    type Property = Mass;
    const RATIO: f64 = 6.35029;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " st.";
}

#[allow(dead_code)]
pub struct Pound;
impl MeasurementUnit for Pound {
    type Property = Mass;
    const RATIO: f64 = 0.45359237;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lb";
}

#[allow(dead_code)]
pub struct Ounce;
impl MeasurementUnit for Ounce {
    type Property = Mass;
    const RATIO: f64 = 0.028349523;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " oz";
}

#[allow(dead_code)]
pub struct Carat;
impl MeasurementUnit for Carat {
    type Property = Mass;
    const RATIO: f64 = 0.0002;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ct";
}

// Property: mass density
pub struct MassDensity;

pub struct KiloGramPerCubicMetre;
impl MeasurementUnit for KiloGramPerCubicMetre {
    type Property = MassDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/m\u{b3}";
}

pub struct GramPerMilliLitre;
impl MeasurementUnit for GramPerMilliLitre {
    type Property = MassDensity;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g/ml";
}

// Property: mass flow rate
pub struct MassFlowRate;

pub struct KiloGramPerSecond;
impl MeasurementUnit for KiloGramPerSecond {
    type Property = MassFlowRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/s";
}

pub struct GramPerSecond;
impl MeasurementUnit for GramPerSecond {
    type Property = MassFlowRate;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g/s";
}

// Property: molar concentration
pub struct MolarConcentration;

pub struct MolePerCubicMetre;
impl MeasurementUnit for MolePerCubicMetre {
    type Property = MolarConcentration;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mol/m\u{b3}";
}

// Property: molar heat capacity, molar entropy
pub struct MolarHeatCapacity;

pub struct JoulePerKelvinPerMole;
impl MeasurementUnit for JoulePerKelvinPerMole {
    type Property = MolarHeatCapacity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/\u{b0}K/mol";
}

// Property: moment of inertia, rotational inertia
pub struct MomentOfInertia;

pub struct KiloGramSquareMetre;
impl MeasurementUnit for KiloGramSquareMetre {
    type Property = MomentOfInertia;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m\u{b2}";
}

pub struct GramSquareCentiMetre;
impl MeasurementUnit for GramSquareCentiMetre {
    type Property = MomentOfInertia;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm\u{b2}";
}

// Property: momentum, impulse
pub struct Momentum;
impl VectorProperty for Momentum {}

pub struct NewtonSecond;
impl MeasurementUnit for NewtonSecond {
    type Property = Momentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}s";
}

pub struct KiloGramMetrePerSecond;
impl MeasurementUnit for KiloGramMetrePerSecond {
    type Property = Momentum;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg\u{b7}m/s";
}

pub struct DyneSecond;
impl MeasurementUnit for DyneSecond {
    type Property = Momentum;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dyn\u{b7}s";
}

#[allow(dead_code)]
pub struct GramCentiMetrePerSecond;
impl MeasurementUnit for GramCentiMetrePerSecond {
    type Property = Momentum;
    const RATIO: f64 = 1e-5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " g\u{b7}cm/s";
}

// Property: permittivity
pub struct Permittivity;

pub struct FaradPerMetre;
impl MeasurementUnit for FaradPerMetre {
    type Property = Permittivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " F/m";
}

// Property: power
pub struct Power;

pub struct Watt;
impl MeasurementUnit for Watt {
    type Property = Power;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W";
}

pub struct MilliWatt;
impl MeasurementUnit for MilliWatt {
    type Property = Power;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mW";
}

pub struct KiloWatt;
impl MeasurementUnit for KiloWatt {
    type Property = Power;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kW";
}

#[allow(dead_code)]
pub struct MegaWatt;
impl MeasurementUnit for MegaWatt {
    type Property = Power;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MW";
}

#[allow(dead_code)]
pub struct GigaWatt;
impl MeasurementUnit for GigaWatt {
    type Property = Power;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GW";
}

pub struct ErgPerSecond;
impl MeasurementUnit for ErgPerSecond {
    type Property = Power;
    const RATIO: f64 = 1e-7;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " erg/s";
}

#[allow(dead_code)]
pub struct HorsePower;
impl MeasurementUnit for HorsePower {
    type Property = Power;
    const RATIO: f64 = 745.699872;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " HP";
}

// Property: pressure, stress
pub struct Pressure;

pub struct Pascal;
impl MeasurementUnit for Pascal {
    type Property = Pressure;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Pa";
}

pub struct HectoPascal;
impl MeasurementUnit for HectoPascal {
    type Property = Pressure;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " hPa";
}

#[allow(dead_code)]
pub struct Atmosphere;
impl MeasurementUnit for Atmosphere {
    type Property = Pressure;
    const RATIO: f64 = 1.013e5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " atm";
}

#[allow(dead_code)]
pub struct Bar;
impl MeasurementUnit for Bar {
    type Property = Pressure;
    const RATIO: f64 = 1e5;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " bar";
}

#[allow(dead_code)]
pub struct MilliBar;
impl MeasurementUnit for MilliBar {
    type Property = Pressure;
    const RATIO: f64 = 100.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mbar";
}

#[allow(dead_code)]
pub struct MmHg;
impl MeasurementUnit for MmHg {
    type Property = Pressure;
    const RATIO: f64 = 133.322;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " torr";
}

pub struct PoundForcePerSquareInch;
impl MeasurementUnit for PoundForcePerSquareInch {
    type Property = Pressure;
    const RATIO: f64 = 6894.757;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lb/in\u{b2}";
}

// Property: radiance
pub struct Radiance;

pub struct WattPerSquareMetrePerSteradian;
impl MeasurementUnit for WattPerSquareMetrePerSteradian {
    type Property = Radiance;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/m\u{b2}/sr";
}

// Property: radiant intensity
pub struct RadiantIntensity;

pub struct WattPerSteradian;
impl MeasurementUnit for WattPerSteradian {
    type Property = RadiantIntensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/sr";
}

// Property: radioactive activity
pub struct RadioactiveActivity;

#[allow(dead_code)]
pub struct Becquerel;
impl MeasurementUnit for Becquerel {
    type Property = RadioactiveActivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Bq";
}

#[allow(dead_code)]
pub struct KiloBecquerel;
impl MeasurementUnit for KiloBecquerel {
    type Property = RadioactiveActivity;
    const RATIO: f64 = 1e3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kBq";
}

#[allow(dead_code)]
pub struct MegaBecquerel;
impl MeasurementUnit for MegaBecquerel {
    type Property = RadioactiveActivity;
    const RATIO: f64 = 1e6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " MBq";
}

#[allow(dead_code)]
pub struct GigaBecquerel;
impl MeasurementUnit for GigaBecquerel {
    type Property = RadioactiveActivity;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " GBq";
}

// Property: radioactive dose
pub struct RadioactiveDose;

pub struct Gray;
impl MeasurementUnit for Gray {
    type Property = RadioactiveDose;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gy";
}

#[allow(dead_code)]
pub struct Rad;
impl MeasurementUnit for Rad {
    type Property = RadioactiveDose;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad";
}

// Property: radioactive dose rate
pub struct RadioactiveDoseRate;

pub struct GrayPerSecond;
impl MeasurementUnit for GrayPerSecond {
    type Property = RadioactiveDoseRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Gy/s";
}

// Property: reaction rate, catalytic activity concentration
pub struct ReactionRate;

pub struct MolePerCubicMetrePerSecond;
impl MeasurementUnit for MolePerCubicMetrePerSecond {
    type Property = ReactionRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mol/m\u{b3}/s";
}

// Property: solid angle
pub struct SolidAngle;

pub struct Steradian;
impl MeasurementUnit for Steradian {
    type Property = SolidAngle;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " sr";
}

#[allow(dead_code)]
pub struct Spat;
impl MeasurementUnit for Spat {
    type Property = SolidAngle;
    const RATIO: f64 = 2. * core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " sp";
}

#[allow(dead_code)]
pub struct Sphere;
impl MeasurementUnit for Sphere {
    type Property = SolidAngle;
    const RATIO: f64 = 2. * core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " sphere";
}

#[allow(dead_code)]
pub struct SquareDegree;
impl MeasurementUnit for SquareDegree {
    type Property = SolidAngle;
    const RATIO: f64 = core::f64::consts::TAU * core::f64::consts::TAU / 360. / 360.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " deg\u{b2}";
}

// Property: specific energy
pub struct SpecificEnergy;

pub struct JoulePerKiloGram;
impl MeasurementUnit for JoulePerKiloGram {
    type Property = SpecificEnergy;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/kg";
}

// Property: specific heat capacity
pub struct SpecificHeatCapacity;

pub struct JoulePerKiloGramPerKelvin;
impl MeasurementUnit for JoulePerKiloGramPerKelvin {
    type Property = SpecificHeatCapacity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/kg/\u{b0}K";
}

// Property: specific volume
pub struct SpecificVolume;

pub struct CubicMetrePerKiloGram;
impl MeasurementUnit for CubicMetrePerKiloGram {
    type Property = SpecificVolume;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b3}/kg";
}

// Property: square time
pub struct SquareTime;

pub struct SquareSecond;
impl MeasurementUnit for SquareSecond {
    type Property = SquareTime;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s\u{b2}";
}

pub struct HourSecond;
impl MeasurementUnit for HourSecond {
    type Property = SquareTime;
    const RATIO: f64 = 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " h\u{b7}s";
}

pub struct HourHour;
impl MeasurementUnit for HourHour {
    type Property = SquareTime;
    const RATIO: f64 = 3600. * 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " h\u{b7}h";
}

// Property: surface density
pub struct SurfaceDensity;

pub struct KiloGramPerSquareMetre;
impl MeasurementUnit for KiloGramPerSquareMetre {
    type Property = SurfaceDensity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kg/m\u{b2}";
}

// Property: surface tension
pub struct SurfaceTension;

pub struct JoulePerSquareMetre;
impl MeasurementUnit for JoulePerSquareMetre {
    type Property = SurfaceTension;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " J/m\u{b2}";
}

// Property: temperature
pub struct Temperature;

pub struct Kelvin;
impl MeasurementUnit for Kelvin {
    type Property = Temperature;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b0}K";
}

#[allow(dead_code)]
pub struct Celsius;
impl MeasurementUnit for Celsius {
    type Property = Temperature;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 273.15;
    const SUFFIX: &'static str = " \u{b0}C";
}

#[allow(dead_code)]
pub struct Fahrenheit;
impl MeasurementUnit for Fahrenheit {
    type Property = Temperature;
    const RATIO: f64 = 5. / 9.;
    const OFFSET: f64 = 273.15 - 32. * 5. / 9.;
    const SUFFIX: &'static str = " \u{b0}F";
}

// Property: thermal conductivity
pub struct ThermalConductivity;

#[allow(dead_code)]
pub struct WattPerMetrePerKelvin;
impl MeasurementUnit for WattPerMetrePerKelvin {
    type Property = ThermalConductivity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W/m/\u{b0}K";
}

// Property: time, mean lifetime
pub struct Time;

pub struct Second;
impl MeasurementUnit for Second {
    type Property = Time;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " s";
}

#[allow(dead_code)]
pub struct Year;
impl MeasurementUnit for Year {
    type Property = Time;
    const RATIO: f64 = 365.25 * 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " Y";
}

#[allow(dead_code)]
pub struct Month;
impl MeasurementUnit for Month {
    type Property = Time;
    const RATIO: f64 = 30. * 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " M";
}

#[allow(dead_code)]
pub struct Week;
impl MeasurementUnit for Week {
    type Property = Time;
    const RATIO: f64 = 7. * 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " W";
}

#[allow(dead_code)]
pub struct Day;
impl MeasurementUnit for Day {
    type Property = Time;
    const RATIO: f64 = 86400.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " D";
}

pub struct Hour;
impl MeasurementUnit for Hour {
    type Property = Time;
    const RATIO: f64 = 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " h";
}

pub struct Minute;
impl MeasurementUnit for Minute {
    type Property = Time;
    const RATIO: f64 = 60.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " min";
}

pub struct MilliSecond;
impl MeasurementUnit for MilliSecond {
    type Property = Time;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ms";
}

pub struct MicroSecond;
impl MeasurementUnit for MicroSecond {
    type Property = Time;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}s";
}

#[allow(dead_code)]
pub struct NanoSecond;
impl MeasurementUnit for NanoSecond {
    type Property = Time;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ns";
}

pub struct PicoSecond;
impl MeasurementUnit for PicoSecond {
    type Property = Time;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ps";
}

#[allow(dead_code)]
pub struct FemtoSecond;
impl MeasurementUnit for FemtoSecond {
    type Property = Time;
    const RATIO: f64 = 1e-15;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " fs";
}

// Property: torque
pub struct Torque;
impl VectorProperty for Torque {}

pub struct NewtonMetre;
impl MeasurementUnit for NewtonMetre {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " N\u{b7}m";
}

pub struct PoundFoot;
impl MeasurementUnit for PoundFoot {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lbf-ft";
}

pub struct PoundInch;
impl MeasurementUnit for PoundInch {
    type Property = Torque;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " lbf-in";
}

// Property: velocity, speed
pub struct Velocity;
impl VectorProperty for Velocity {}

pub struct MetrePerSecond;
impl MeasurementUnit for MetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m/s";
}

pub struct Knot;
impl MeasurementUnit for Knot {
    type Property = Velocity;
    const RATIO: f64 = 1852. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " kt";
}

pub struct KiloMetrePerHour;
impl MeasurementUnit for KiloMetrePerHour {
    type Property = Velocity;
    const RATIO: f64 = 1000. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km/h";
}

pub struct MilePerHour;
impl MeasurementUnit for MilePerHour {
    type Property = Velocity;
    const RATIO: f64 = 1609. / 3600.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi/h";
}

pub struct CentiMetrePerSecond;
impl MeasurementUnit for CentiMetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 0.01;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm/s";
}

#[allow(dead_code)]
pub struct KiloMetrePerSecond;
impl MeasurementUnit for KiloMetrePerSecond {
    type Property = Velocity;
    const RATIO: f64 = 1000.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km/s";
}

// Property: volume
pub struct Volume;

pub struct CubicMetre;
impl MeasurementUnit for CubicMetre {
    type Property = Volume;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b3}";
}

pub struct CubicKiloMetre;
impl MeasurementUnit for CubicKiloMetre {
    type Property = Volume;
    const RATIO: f64 = 1e9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " km\u{b3}";
}

pub struct CubicMicroMetre;
impl MeasurementUnit for CubicMicroMetre {
    type Property = Volume;
    const RATIO: f64 = 1e-18;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}m\u{b3}?";
}

pub struct CubicNanoMetre;
impl MeasurementUnit for CubicNanoMetre {
    type Property = Volume;
    const RATIO: f64 = 1e-27;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nm\u{b3}?";
}

pub struct CubicInch;
impl MeasurementUnit for CubicInch {
    type Property = Volume;
    const RATIO: f64 = 0.0254 * 0.0254 * 0.0254;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " in\u{b3}";
}

pub struct CubicFoot;
impl MeasurementUnit for CubicFoot {
    type Property = Volume;
    const RATIO: f64 = 0.3048 * 0.3048 * 0.3048;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ft\u{b3}";
}

pub struct CubicYard;
impl MeasurementUnit for CubicYard {
    type Property = Volume;
    const RATIO: f64 = 0.9144 * 0.9144 * 0.9144;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " yd\u{b3}";
}

pub struct CubicMile;
impl MeasurementUnit for CubicMile {
    type Property = Volume;
    const RATIO: f64 = 1609. * 1609. * 1609.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mi\u{b3}";
}

pub struct Litre;
impl MeasurementUnit for Litre {
    type Property = Volume;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " l";
}

#[allow(dead_code)]
pub struct CubicDecimetre;
impl MeasurementUnit for CubicDecimetre {
    type Property = Volume;
    const RATIO: f64 = 1e-3;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " dm\u{b3}";
}

pub struct MilliLitre;
impl MeasurementUnit for MilliLitre {
    type Property = Volume;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ml";
}

#[allow(dead_code)]
pub struct CubicCentimetre;
impl MeasurementUnit for CubicCentimetre {
    type Property = Volume;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm\u{b3}";
}

pub struct MicroLitre;
impl MeasurementUnit for MicroLitre {
    type Property = Volume;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " \u{b5}l";
}

#[allow(dead_code)]
pub struct CubicMillimetre;
impl MeasurementUnit for CubicMillimetre {
    type Property = Volume;
    const RATIO: f64 = 1e-9;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " mm\u{b3}";
}

#[allow(dead_code)]
pub struct NanoLitre;
impl MeasurementUnit for NanoLitre {
    type Property = Volume;
    const RATIO: f64 = 1e-12;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " nl";
}

#[allow(dead_code)]
pub struct PicoLitre;
impl MeasurementUnit for PicoLitre {
    type Property = Volume;
    const RATIO: f64 = 1e-15;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pl";
}

#[allow(dead_code)]
pub struct Pint;
impl MeasurementUnit for Pint {
    type Property = Volume;
    const RATIO: f64 = 473.2e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " pt";
}

#[allow(dead_code)]
pub struct Gallon;
impl MeasurementUnit for Gallon {
    type Property = Volume;
    const RATIO: f64 = 4546e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " gal";
}

// Property: volumetric flow rate
pub struct VolumetricFlowRate;

pub struct CubicMetrePerSecond;
impl MeasurementUnit for CubicMetrePerSecond {
    type Property = VolumetricFlowRate;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " m\u{b3}/s";
}

pub struct MilliLitrePerSecond;
impl MeasurementUnit for MilliLitrePerSecond {
    type Property = VolumetricFlowRate;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " ml/s";
}

#[allow(dead_code)]
pub struct CubicCentimetrePerSecond;
impl MeasurementUnit for CubicCentimetrePerSecond {
    type Property = VolumetricFlowRate;
    const RATIO: f64 = 1e-6;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " cm\u{b3}/s";
}

// Property: wave number
pub struct WaveNumber;

pub struct CyclePerMetre;
impl MeasurementUnit for CyclePerMetre {
    type Property = WaveNumber;
    const RATIO: f64 = 1.;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " 1/m";
}

pub struct RadianPerMetre;
impl MeasurementUnit for RadianPerMetre {
    type Property = WaveNumber;
    const RATIO: f64 = 1. / core::f64::consts::TAU;
    const OFFSET: f64 = 0.;
    const SUFFIX: &'static str = " rad/m";
}
