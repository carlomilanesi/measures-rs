measures::define_measure_types! {
    with_points with_directions with_2d with_3d with_transformations exact with_approx,

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
        // One 1 == One 1 * One 1,
        // ```
    ]
}

// Property: acceleration
measures::measurement_vector_property! { Acceleration}

measures::measurement_unit! {
    name: MetrePerSquareSecond,
    property: Acceleration,
    suffix: " m/s\u{b2}", // m/s²
}

measures::measurement_unit! {
    name: CentiMetrePerSquareSecond,
    property: Acceleration,
    ratio: 1e-2,
    suffix: " cm/s\u{b2}", // cm/s²
}

measures::measurement_unit! {
    name: GForce,
    property: Acceleration,
    ratio: 9.80665,
    suffix: " g",
}

measures::measurement_unit! {
    name: KiloMetrePerHourPerSecond,
    property: Acceleration,
    ratio: 1. / 3.6,
    suffix: " km/h/s",
}

// Property: action
measures::measurement_scalar_property! { Action }

measures::measurement_unit! {
    name: JouleSecond,
    property: Action,
    suffix: " J\u{b7}s", // J·s
}

// Property: amount of substance, count
measures::measurement_scalar_property! { Amount }

measures::measurement_unit! {
    name: Unit,
    property: Amount,
    suffix: " u.",
}

measures::measurement_unit! {
    name: Dozen,
    property: Amount,
    ratio: 12.,
    suffix: " dz.",
}

measures::measurement_unit! {
    name: Mole,
    property: Amount,
    ratio: 6.0221413e23,
    suffix: " mol",
}

// Property: angle
measures::angle_measurement_unit! {
    name: Cycle,
    suffix: " rev",
    cycle_fraction: 1.,
}

measures::angle_measurement_unit! {
    name: Gradian,
    suffix: " grad",
    cycle_fraction: 400.,
}

measures::angle_measurement_unit! {
    name: Degree,
    suffix: " deg",
    cycle_fraction: 360.,
}

measures::angle_measurement_unit! {
    name: ArcMinute,
    suffix: " deg'",
    cycle_fraction: 360. * 60.,
}

measures::angle_measurement_unit! {
    name: ArcSecond,
    suffix: " deg\"",
    cycle_fraction: 360. * 60. * 60.,
}

// Property: angular acceleration
measures::measurement_vector_property! { AngularAcceleration }

measures::measurement_unit! {
    name: RadianPerSquareSecond,
    property: AngularAcceleration,
    suffix: " rad/s\u{b2}", // rad/s²
}

// Property: angular momentum, spin
measures::measurement_vector_property! { AngularMomentum }

measures::measurement_unit! {
    name: KiloGramSquareMetrePerSecond,
    property: AngularMomentum,
    suffix: " kg\u{b7}m\u{b2}/s", // kg·m²/s
}

measures::measurement_unit! {
    name: GramSquareCentiMetrePerSecond,
    property: AngularMomentum,
    ratio: 1e-7,
    suffix: " g\u{b7}cm\u{b2}/s", // g·cm²/s
}

// Property: area
measures::measurement_scalar_property! { Area }

measures::measurement_unit! {
    name: SquareMetre,
    property: Area,
    suffix: " m\u{b2}", // m²
}

measures::measurement_unit! {
    name: SquareKiloMetre,
    property: Area,
    ratio: 1e6,
    suffix: " km\u{b2}", // km²
}

measures::measurement_unit! {
    name: Hectare,
    property: Area,
    ratio: 1e4,
    suffix: " ha",
}

measures::measurement_unit! {
    name: Are,
    property: Area,
    ratio: 100.,
    suffix: " are",
}

measures::measurement_unit! {
    name: SquareDeciMetre,
    property: Area,
    ratio: 0.01,
    suffix: " dm\u{b2}", // dm²
}

measures::measurement_unit! {
    name: SquareCentiMetre,
    property: Area,
    ratio: 1e-4,
    suffix: " cm\u{b2}", // cm²
}

measures::measurement_unit! {
    name: SquareMilliMetre,
    property: Area,
    ratio: 1e-6,
    suffix: " mm\u{b2}", // mm²
}

measures::measurement_unit! {
    name: SquareMicroMetre,
    property: Area,
    ratio: 1e-12,
    suffix: " \u{b5}m\u{b2}", // µm²
}

measures::measurement_unit! {
    name: SquareNanoMetre,
    property: Area,
    ratio: 1e-18,
    suffix: " nm\u{b2}", // nm²
}

measures::measurement_unit! {
    name: SquareInch,
    property: Area,
    ratio: 0.0254 * 0.0254,
    suffix: " in\u{b2}", // in²
}

measures::measurement_unit! {
    name: SquareFoot,
    property: Area,
    ratio: 0.3048 * 0.3048,
    suffix: " ft\u{b2}", // ft²
}

measures::measurement_unit! {
    name: SquareYard,
    property: Area,
    ratio: 0.9144 * 0.9144,
    suffix: " yd\u{b2}", // yd²
}

measures::measurement_unit! {
    name: SquareMile,
    property: Area,
    ratio: 1609. * 1609.,
    suffix: " mi\u{b2}", // mi²
}

// Property: capacitance
measures::measurement_scalar_property! { Capacitance }

measures::measurement_unit! {
    name: Farad,
    property: Capacitance,
    suffix: " F",
}

measures::measurement_unit! {
    name: MilliFarad,
    property: Capacitance,
    ratio: 1e-3,
    suffix: " mF",
}

measures::measurement_unit! {
    name: MicroFarad,
    property: Capacitance,
    ratio: 1e-6,
    suffix: " \u{b5}F", // µF
}

measures::measurement_unit! {
    name: NanoFarad,
    property: Capacitance,
    ratio: 1e-9,
    suffix: " nF",
}

measures::measurement_unit! {
    name: PicoFarad,
    property: Capacitance,
    ratio: 1e-12,
    suffix: " pF",
}

// Property: catalytic activity
measures::measurement_scalar_property! { CatalyticActivity }

measures::measurement_unit! {
    name: Katal,
    property: CatalyticActivity,
    suffix: " kat",
}

// Property: chemical potential, molar energy
measures::measurement_scalar_property! { ChemicalPotential }

measures::measurement_unit! {
    name: JoulePerMole,
    property: ChemicalPotential,
    suffix: " J/mol",
}

// Property: current density
measures::measurement_vector_property! { CurrentDensity }

measures::measurement_unit! {
    name: AmperePerSquareMetre,
    property: CurrentDensity,
    suffix: " A/m\u{b2}", // A/m²
}

// Property: dimensionless
measures::measurement_unit! {
    name: Mach,
    property: Dimensionless,
    suffix: " mach",
}

// Property: dose equivalent
measures::measurement_scalar_property! { DoseEquivalent }

measures::measurement_unit! {
    name: Sievert,
    property: DoseEquivalent,
    suffix: " Sv",
}

measures::measurement_unit! {
    name: Rem,
    property: DoseEquivalent,
    ratio: 0.01,
    suffix: " rem",
}

// Property: dynamic viscosity, absolute viscosity
measures::measurement_scalar_property! { DynamicViscosity }

measures::measurement_unit! {
    name: PascalSecond,
    property: DynamicViscosity,
    suffix: " Pa\u{b7}s", // Pa·s
}

// Property: electrical conductance, electric susceptance, electric admittance
measures::measurement_scalar_property! { ElectricalConductance }

measures::measurement_unit! {
    name: Siemens,
    property: ElectricalConductance,
    suffix: " S",
}

// Property: electrical conductivity
measures::measurement_scalar_property! { ElectricalConductivity }

measures::measurement_unit! {
    name: SiemensPerMetre,
    property: ElectricalConductivity,
    suffix: " S/m",
}

// Property: electrical resistance, electrical impedance
measures::measurement_scalar_property! { ElectricalResistance }

measures::measurement_unit! {
    name: Ohm,
    property: ElectricalResistance,
    suffix: " \u{3a9}", // Ω
}

measures::measurement_unit! {
    name: MilliOhm,
    property: ElectricalResistance,
    ratio: 1e-3,
    suffix: " m\u{3a9}", // mΩ
}

measures::measurement_unit! {
    name: KiloOhm,
    property: ElectricalResistance,
    ratio: 1e3,
    suffix: " k\u{3a9}", // kΩ
}

// Property: electrical resistivity
measures::measurement_scalar_property! { ElectricalResistivity }

measures::measurement_unit! {
    name: OhmMetre,
    property: ElectricalResistivity,
    suffix: " \u{3a9}\u{b7}m", // Ω·m
}

// Property: electric charge
measures::measurement_scalar_property! { ElectricCharge }

measures::measurement_unit! {
    name: Coulomb,
    property: ElectricCharge,
    suffix: " C",
}

measures::measurement_unit! {
    name: MilliCoulomb,
    property: ElectricCharge,
    ratio: 1e-3,
    suffix: " mC",
}

measures::measurement_unit! {
    name: MicroCoulomb,
    property: ElectricCharge,
    ratio: 1e-6,
    suffix: " \u{b5}C", // µC
}

measures::measurement_unit! {
    name: NanoCoulomb,
    property: ElectricCharge,
    ratio: 1e-9,
    suffix: " ",
}

measures::measurement_unit! {
    name: PicoCoulomb,
    property: ElectricCharge,
    ratio: 1e-12,
    suffix: " pC",
}

// Property: electric charge density
measures::measurement_scalar_property! { ElectricChargeDensity }

measures::measurement_unit! {
    name: CoulombPerCubicMetre,
    property: ElectricChargeDensity,
    suffix: " C/m\u{b3}", // C/m³
}

// Property: electric current
measures::measurement_scalar_property! { ElectricCurrent }

measures::measurement_unit! {
    name: Ampere,
    property: ElectricCurrent,
    suffix: " A",
}

measures::measurement_unit! {
    name: MilliAmpere,
    property: ElectricCurrent,
    ratio: 1e-3,
    suffix: " mA",
}

measures::measurement_unit! {
    name: MicroAmpere,
    property: ElectricCurrent,
    ratio: 1e-6,
    suffix: " \u{b5}A", // µA
}

// Property: electric displacement, surface electric charge density
measures::measurement_scalar_property! { ElectricDisplacement }

measures::measurement_unit! {
    name: CoulombPerSquareMetre,
    property: ElectricDisplacement,
    suffix: " C/m\u{b2}", // C/m²
}

// Property: electric field strength
measures::measurement_vector_property! { ElectricFieldStrength }

measures::measurement_unit! {
    name: VoltPerMetre,
    property: ElectricFieldStrength,
    suffix: " V/m",
}

measures::measurement_unit! {
    name: NewtonPerCoulomb,
    property: ElectricFieldStrength,
    suffix: " N/C",
}

// Property: electric potential
measures::measurement_scalar_property! { ElectricPotential }

measures::measurement_unit! {
    name: Volt,
    property: ElectricPotential,
    suffix: " V",
}

measures::measurement_unit! {
    name: KiloVolt,
    property: ElectricPotential,
    ratio: 1e3,
    suffix: " kV",
}

measures::measurement_unit! {
    name: MilliVolt,
    property: ElectricPotential,
    ratio: 1e-3,
    suffix: " mV",
}

measures::measurement_unit! {
    name: MicroVolt,
    property: ElectricPotential,
    suffix: " \u{b5}V", // µV
}

// Property: energy, work, heat
measures::measurement_scalar_property! { Energy }

measures::measurement_unit! {
    name: Joule,
    property: Energy,
    suffix: " J",
}

measures::measurement_unit! {
    name: Erg,
    property: Energy,
    ratio: 1e-7,
    suffix: " erg",
}

measures::measurement_unit! {
    name: WattHour,
    property: Energy,
    ratio: 3600.,
    suffix: " W\u{b7}h", // W·h
}

measures::measurement_unit! {
    name: KiloWattHour,
    property: Energy,
    ratio: 3.6e6,
    suffix: " kW\u{b7}h", // kW·h
}

measures::measurement_unit! {
    name: MegaWattHour,
    property: Energy,
    ratio: 3.6e9,
    suffix: " MW\u{b7}h", // MW·h
}

measures::measurement_unit! {
    name: Calorie,
    property: Energy,
    ratio: 4.187,
    suffix: " cal",
}

measures::measurement_unit! {
    name: KiloCalorie,
    property: Energy,
    ratio: 4187.,
    suffix: " kcal",
}

measures::measurement_unit! {
    name: ElectronVolt,
    property: Energy,
    ratio: 1.602176634e-19,
    suffix: " eV",
}

measures::measurement_unit! {
    name: KiloElectronVolt,
    property: Energy,
    ratio: 1.602176634e-16,
    suffix: " keV",
}

measures::measurement_unit! {
    name: MegaElectronVolt,
    property: Energy,
    ratio: 1.602176634e-13,
    suffix: " MeV",
}

measures::measurement_unit! {
    name: GigaElectronVolt,
    property: Energy,
    ratio: 1.602176634e-10,
    suffix: " GeV",
}

measures::measurement_unit! {
    name: TeraElectronVolt,
    property: Energy,
    ratio: 1.602176634e-7,
    suffix: " TeV",
}

// Property: energy density
measures::measurement_scalar_property! { EnergyDensity }

measures::measurement_unit! {
    name: JoulePerCubicMetre,
    property: EnergyDensity,
    suffix: " J/m\u{b3}", // J/m³
}

// Property: entropy, heat capacity
measures::measurement_scalar_property! { Entropy }

measures::measurement_unit! {
    name: JoulePerKelvin,
    property: Entropy,
    suffix: " J/\u{b0}K", // J/°K
}

// Property: force, weight
measures::measurement_vector_property! { Force }

measures::measurement_unit! {
    name: Newton,
    property: Force,
    suffix: " N",
}

measures::measurement_unit! {
    name: Dyne,
    property: Force,
    ratio: 1e-5,
    suffix: " dyn",
}

measures::measurement_unit! {
    name: KiloGramForce,
    property: Force,
    ratio: 9.80665,
    suffix: " kgf",
}

measures::measurement_unit! {
    name: PoundForce,
    property: Force,
    ratio: 4.448222,
    suffix: " lbf",
}

measures::measurement_unit! {
    name: Poundal,
    property: Force,
    ratio: 0.138255,
    suffix: " pdl",
}

// Property: frequency, angular speed, angular velocity
measures::measurement_scalar_property! { Frequency }

measures::measurement_unit! {
    name: Hertz,
    property: Frequency,
    suffix: " Hz",
}

measures::measurement_unit! {
    name: CyclePerSecond,
    property: Frequency,
    suffix: " c/s",
}

measures::measurement_unit! {
    name: KiloHertz,
    property: Frequency,
    ratio: 1e3,
    suffix: " kHz",
}

measures::measurement_unit! {
    name: MegaHertz,
    property: Frequency,
    ratio: 1e6,
    suffix: " MHz",
}

measures::measurement_unit! {
    name: GigaHertz,
    property: Frequency,
    ratio: 1e9,
    suffix: " GHz",
}

measures::measurement_unit! {
    name: RadianPerSecond,
    property: Frequency,
    ratio: 1. / core::f64::consts::TAU,
    suffix: " rad/s",
}

measures::measurement_unit! {
    name: CyclePerMinute,
    property: Frequency,
    ratio: 1. / 60.,
    suffix: " rpm",
}

measures::measurement_scalar_property! { Illuminance }

measures::measurement_unit! {
    name: Lux,
    property: Illuminance,
    suffix: " lx",
}

measures::measurement_unit! {
    name: Phot,
    property: Illuminance,
    ratio: 1e4,
    suffix: " phot",
}

measures::measurement_unit! {
    name: FootCandle,
    property: Illuminance,
    ratio: 10.764,
    suffix: " fc",
}

// Property: inductance
measures::measurement_scalar_property! { Inductance }

measures::measurement_unit! {
    name: Henry,
    property: Inductance,
    suffix: " H",
}

// Property: information
measures::measurement_scalar_property! { Information }

measures::measurement_unit! {
    name: Bit,
    property: Information,
    suffix: " b",
}

measures::measurement_unit! {
    name: Byte,
    property: Information,
    ratio: 8.,
    suffix: " B",
}

measures::measurement_unit! {
    name: KiloBit,
    property: Information,
    ratio: 1e3,
    suffix: " kb",
}

measures::measurement_unit! {
    name: KiloByte,
    property: Information,
    ratio: 8e3,
    suffix: " kB",
}

measures::measurement_unit! {
    name: KibiBit,
    property: Information,
    ratio: 1024.,
    suffix: " kib",
}

measures::measurement_unit! {
    name: KibiByte,
    property: Information,
    ratio: 8. * 1024.,
    suffix: " kiB",
}

measures::measurement_unit! {
    name: MegaBit,
    property: Information,
    ratio: 1e6,
    suffix: " Mb",
}

measures::measurement_unit! {
    name: MegaByte,
    property: Information,
    ratio: 8e6,
    suffix: " MB",
}

measures::measurement_unit! {
    name: MebiBit,
    property: Information,
    ratio: 1024. * 1024.,
    suffix: " Mib",
}

measures::measurement_unit! {
    name: MebiByte,
    property: Information,
    ratio: 8. * 1024. * 1024.,
    suffix: " MiB",
}

measures::measurement_unit! {
    name: GigaBit,
    property: Information,
    ratio: 1e9,
    suffix: " Gb",
}

measures::measurement_unit! {
    name: GigaByte,
    property: Information,
    ratio: 8e9,
    suffix: " GB",
}

measures::measurement_unit! {
    name: GibiBit,
    property: Information,
    ratio: 1024. * 1024. * 1024.,
    suffix: " Gib",
}

measures::measurement_unit! {
    name: GibiByte,
    property: Information,
    ratio: 8. * 1024. * 1024. * 1024.,
    suffix: " GiB",
}

measures::measurement_unit! {
    name: TeraBit,
    property: Information,
    ratio: 1e12,
    suffix: " Tb",
}

measures::measurement_unit! {
    name: TeraByte,
    property: Information,
    ratio: 8e12,
    suffix: " TB",
}

measures::measurement_unit! {
    name: TebiBit,
    property: Information,
    ratio: 1024. * 1024. * 1024. * 1024.,
    suffix: " TiB",
}

measures::measurement_unit! {
    name: TebiByte,
    property: Information,
    ratio: 8. * 1024. * 1024. * 1024. * 1024.,
    suffix: " TiB",
}

// Property: information rate
measures::measurement_scalar_property! { InformationRate }

measures::measurement_unit! {
    name: BitPerSecond,
    property: InformationRate,
    suffix: " b/s",
}

measures::measurement_unit! {
    name: BytePerSecond,
    property: InformationRate,
    ratio: 8.,
    suffix: " B/s",
}

measures::measurement_unit! {
    name: KiloBitPerSecond,
    property: InformationRate,
    ratio: 1e3,
    suffix: " kb/s",
}

measures::measurement_unit! {
    name: KiloBytePerSecond,
    property: InformationRate,
    ratio: 8e3,
    suffix: " kB/s",
}

measures::measurement_unit! {
    name: KibiBitPerSecond,
    property: InformationRate,
    ratio: 1024.,
    suffix: " kib/s",
}

measures::measurement_unit! {
    name: KibiBytePerSecond,
    property: InformationRate,
    ratio: 8. * 1024.,
    suffix: " kiB/s",
}

measures::measurement_unit! {
    name: MegaBitPerSecond,
    property: InformationRate,
    ratio: 1e6,
    suffix: " Mb/s",
}

measures::measurement_unit! {
    name: MegaBytePerSecond,
    property: InformationRate,
    ratio: 8e6,
    suffix: " MB/s",
}

measures::measurement_unit! {
    name: MebiBitPerSecond,
    property: InformationRate,
    ratio: 1024. * 1024.,
    suffix: " Mib/s",
}

measures::measurement_unit! {
    name: MebiBytePerSecond,
    property: InformationRate,
    ratio: 8. * 1024. * 1024.,
    suffix: " MiB/s",
}

measures::measurement_unit! {
    name: GigaBitPerSecond,
    property: InformationRate,
    ratio: 1e9,
    suffix: " Gb/s",
}

measures::measurement_unit! {
    name: GigaBytePerSecond,
    property: InformationRate,
    ratio: 8e9,
    suffix: " GB/s",
}

measures::measurement_unit! {
    name: GibiBitPerSecond,
    property: InformationRate,
    ratio: 1024. * 1024. * 1024.,
    suffix: " Gib/s",
}

measures::measurement_unit! {
    name: GibiBytePerSecond,
    property: InformationRate,
    ratio: 8. * 1024. * 1024. * 1024.,
    suffix: " GiB/s",
}

measures::measurement_unit! {
    name: TeraBitPerSecond,
    property: InformationRate,
    ratio: 1e12,
    suffix: " Tb/s",
}

measures::measurement_unit! {
    name: TeraBytePerSecond,
    property: InformationRate,
    ratio: 8e12,
    suffix: " TB/s",
}

measures::measurement_unit! {
    name: TebiBitPerSecond,
    property: InformationRate,
    ratio: 1024. * 1024. * 1024. * 1024.,
    suffix: " Tib/s",
}

measures::measurement_unit! {
    name: TebiBytePerSecond,
    property: InformationRate,
    ratio: 8. * 1024. * 1024. * 1024. * 1024.,
    suffix: " TiB/s",
}

// Property: irradiance, heat flux density
measures::measurement_scalar_property! { Irradiance }

measures::measurement_unit! {
    name: WattPerSquareMetre,
    property: Irradiance,
    suffix: " W/m\u{b2}", // W/m²
}

// Property: kinematic viscosity
measures::measurement_scalar_property! { KinematicViscosity }

measures::measurement_unit! {
    name: SquareMetrePerSecond,
    property: KinematicViscosity,
    suffix: " m\u{b2}/s", // m²/s
}

measures::measurement_unit! {
    name: Stoke,
    property: KinematicViscosity,
    ratio: 1e-4,
    suffix: " St",
}

measures::measurement_unit! {
    name: CentiStoke,
    property: KinematicViscosity,
    ratio: 1e-6,
    suffix: " cSt",
}

// Property: length, width, height, depth, space, wavelength
measures::measurement_vector_property! { Length }

measures::measurement_unit! {
    name: Metre,
    property: Length,
    suffix: " m",
}

measures::measurement_unit! {
    name: AstronomicalUnit,
    property: Length,
    ratio: 149597870691.,
    suffix: " a.u.",
}

measures::measurement_unit! {
    name: Parsec,
    property: Length,
    ratio: 3.0856775813e16,
    suffix: " psc",
}

measures::measurement_unit! {
    name: LightYear,
    property: Length,
    ratio: 31557600. * 2.99792458e8,
    suffix: " ly",
}

measures::measurement_unit! {
    name: KiloMetre,
    property: Length,
    ratio: 1e3,
    suffix: " km",
}

measures::measurement_unit! {
    name: HectoMetre,
    property: Length,
    ratio: 100.,
    suffix: " hm",
}

measures::measurement_unit! {
    name: DecaMetre,
    property: Length,
    ratio: 10.,
    suffix: " dam",
}

measures::measurement_unit! {
    name: DeciMetre,
    property: Length,
    ratio: 0.1,
    suffix: " dm",
}

measures::measurement_unit! {
    name: CentiMetre,
    property: Length,
    ratio: 0.01,
    suffix: " cm",
}

measures::measurement_unit! {
    name: MilliMetre,
    property: Length,
    ratio: 1e-3,
    suffix: " mm",
}

measures::measurement_unit! {
    name: MicroMetre,
    property: Length,
    ratio: 1e-6,
    suffix: " \u{b5}m", // µm
}

measures::measurement_unit! {
    name: NanoMetre,
    property: Length,
    ratio: 1e-9,
    suffix: " nm",
}

measures::measurement_unit! {
    name: Angstrom,
    property: Length,
    ratio: 1e-10,
    suffix: " \u{212b}", // Å
}

measures::measurement_unit! {
    name: Inch,
    property: Length,
    ratio: 0.0254,
    suffix: " in",
}

measures::measurement_unit! {
    name: Foot,
    property: Length,
    ratio: 0.3048,
    suffix: " ft",
}

measures::measurement_unit! {
    name: Yard,
    property: Length,
    ratio: 0.9144,
    suffix: " yd",
}

measures::measurement_unit! {
    name: Mile,
    property: Length,
    ratio: 1609.,
    suffix: " mi",
}

measures::measurement_unit! {
    name: NauticalMile,
    property: Length,
    ratio: 1852.,
    suffix: " naut.mi",
}

// Property: linear density
measures::measurement_scalar_property! { LinearDensity }

measures::measurement_unit! {
    name: KiloGramPerMetre,
    property: LinearDensity,
    suffix: " kg/m",
}

measures::measurement_unit! {
    name: GramPerCentiMetre,
    property: LinearDensity,
    ratio: 0.1,
    suffix: " g/cm",
}

// Property: linear electric charge density
measures::measurement_scalar_property! { LinearElectricChargeDensity }

measures::measurement_unit! {
    name: CoulombPerMetre,
    property: LinearElectricChargeDensity,
    suffix: " C/m",
}

// Property: luminance
measures::measurement_scalar_property! { Luminance }

measures::measurement_unit! {
    name: CandelaPerSquareMetre,
    property: Luminance,
    suffix: " cd/m\u{b2}", // cd/m²
}

measures::measurement_unit! {
    name: Nit,
    property: Luminance,
    suffix: " nt",
}

measures::measurement_unit! {
    name: Stilb,
    property: Luminance,
    ratio: 1e4,
    suffix: " sb",
}

measures::measurement_unit! {
    name: CandelaPerSquareFoot,
    property: Luminance,
    ratio: 10.764,
    suffix: " cd/ft\u{b2}", // cd/ft²
}

// Property: luminous flux, luminous power
measures::measurement_vector_property! { LuminousFlux }

measures::measurement_unit! {
    name: Lumen,
    property: LuminousFlux,
    suffix: " lm",
}

// Property: luminous intensity
measures::measurement_scalar_property! { LuminousIntensity }

measures::measurement_unit! {
    name: Candela,
    property: LuminousIntensity,
    suffix: " cd",
}

// Property: magnetic field strength, magnetic field intensity, magnetization
measures::measurement_vector_property! { MagneticFieldStrength }

measures::measurement_unit! {
    name: AmperePerMetre,
    property: MagneticFieldStrength,
    suffix: " A/m",
}

// Property: magnetic flux
measures::measurement_scalar_property! { MagneticFlux }

measures::measurement_unit! {
    name: Weber,
    property: MagneticFlux,
    suffix: " Wb",
}

// Property: magnetic flux density
measures::measurement_vector_property! { MagneticFluxDensity }

measures::measurement_unit! {
    name: Tesla,
    property: MagneticFluxDensity,
    suffix: " T",
}

measures::measurement_unit! {
    name: Gauss,
    property: MagneticFluxDensity,
    ratio: 1e-4,
    suffix: " G",
}

// Property: magnetic permeability
measures::measurement_scalar_property! { MagneticPermeability }

measures::measurement_unit! {
    name: HenryPerMetre,
    property: MagneticPermeability,
    suffix: " H/m",
}

// Property: magnetic reluctance, magnetic resistance
measures::measurement_scalar_property! { MagneticReluctance }

measures::measurement_unit! {
    name: InverseHenry,
    property: MagneticReluctance,
    suffix: " 1/H",
}

// Property: mass
measures::measurement_scalar_property! { Mass }

measures::measurement_unit! {
    name: KiloGram,
    property: Mass,
    suffix: " kg",
}

measures::measurement_unit! {
    name: Tonne,
    property: Mass,
    ratio: 1e3,
    suffix: " t",
}

#[allow(dead_code)]
pub type MetricTon = Tonne;

measures::measurement_unit! {
    name: MegaGram,
    property: Mass,
    ratio: 1e3,
    suffix: " Mg",
}

measures::measurement_unit! {
    name: HectoGram,
    property: Mass,
    ratio: 0.1,
    suffix: " hg",
}

measures::measurement_unit! {
    name: DecaGram,
    property: Mass,
    ratio: 0.01,
    suffix: " dag",
}

measures::measurement_unit! {
    name: Gram,
    property: Mass,
    ratio: 1e-3,
    suffix: " g",
}

measures::measurement_unit! {
    name: MilliGram,
    property: Mass,
    ratio: 1e-6,
    suffix: " mg",
}

measures::measurement_unit! {
    name: MicroGram,
    property: Mass,
    ratio: 1e-9,
    suffix: " \u{b5}g", // µg
}

measures::measurement_unit! {
    name: NanoGram,
    property: Mass,
    ratio: 1e-12,
    suffix: " ng",
}

measures::measurement_unit! {
    name: ImperialTon,
    property: Mass,
    ratio: 1016.0469,
    suffix: " t",
}

#[allow(dead_code)]
pub type LongTon = ImperialTon;

measures::measurement_unit! {
    name: USTon,
    property: Mass,
    ratio: 907.18474,
    suffix: " t",
}

#[allow(dead_code)]
pub type ShortTon = USTon;

measures::measurement_unit! {
    name: Stone,
    property: Mass,
    ratio: 6.35029,
    suffix: " st.",
}

measures::measurement_unit! {
    name: Pound,
    property: Mass,
    ratio: 0.45359237,
    suffix: " lb",
}

measures::measurement_unit! {
    name: Ounce,
    property: Mass,
    ratio: 0.028349523,
    suffix: " oz",
}

measures::measurement_unit! {
    name: Carat,
    property: Mass,
    ratio: 2e-4,
    suffix: " ct",
}

// Property: mass density
measures::measurement_scalar_property! { MassDensity }

measures::measurement_unit! {
    name: KiloGramPerCubicMetre,
    property: MassDensity,
    suffix: " kg/m\u{b3}", // kg/m³
}

measures::measurement_unit! {
    name: GramPerMilliLitre,
    property: MassDensity,
    ratio: 1e3,
    suffix: " g/ml",
}

// Property: mass flow rate
measures::measurement_scalar_property! { MassFlowRate }

measures::measurement_unit! {
    name: KiloGramPerSecond,
    property: MassFlowRate,
    suffix: " kg/s",
}

measures::measurement_unit! {
    name: GramPerSecond,
    property: MassFlowRate,
    ratio: 1e-3,
    suffix: " g/s",
}

// Property: molar concentration
measures::measurement_scalar_property! { MolarConcentration }

measures::measurement_unit! {
    name: MolePerCubicMetre,
    property: MolarConcentration,
    suffix: " mol/m\u{b3}", // mol/m³
}

// Property: molar heat capacity, molar entropy
measures::measurement_scalar_property! { MolarHeatCapacity }

measures::measurement_unit! {
    name: JoulePerKelvinPerMole,
    property: MolarHeatCapacity,
    suffix: " J/\u{b0}K/mol", // J/°K/mol
}

// Property: moment of inertia, rotational inertia
measures::measurement_scalar_property! { MomentOfInertia }

measures::measurement_unit! {
    name: KiloGramSquareMetre,
    property: MomentOfInertia,
    suffix: " kg\u{b7}m\u{b2}", // kg·m²
}

measures::measurement_unit! {
    name: GramSquareCentiMetre,
    property: MomentOfInertia,
    ratio: 1e-7,
    suffix: " g\u{b7}cm\u{b2}", // g·cm²
}

// Property: momentum, impulse
measures::measurement_vector_property! { Momentum }

measures::measurement_unit! {
    name: NewtonSecond,
    property: Momentum,
    suffix: " N\u{b7}s", // N·s
}

measures::measurement_unit! {
    name: KiloGramMetrePerSecond,
    property: Momentum,
    suffix: " kg\u{b7}m/s", // kg·m/s
}

measures::measurement_unit! {
    name: DyneSecond,
    property: Momentum,
    ratio: 1e-5,
    suffix: " dyn\u{b7}s", // dyn·s
}

measures::measurement_unit! {
    name: GramCentiMetrePerSecond,
    property: Momentum,
    ratio: 1e-5,
    suffix: " g\u{b7}cm/s", // g·cm/s
}

// Property: permittivity
measures::measurement_scalar_property! { Permittivity }

measures::measurement_unit! {
    name: FaradPerMetre,
    property: Permittivity,
    suffix: " F/m",
}

// Property: power
measures::measurement_scalar_property! { Power }

measures::measurement_unit! {
    name: Watt,
    property: Power,
    suffix: " W",
}

measures::measurement_unit! {
    name: MilliWatt,
    property: Power,
    ratio: 1e-3,
    suffix: " mW",
}

measures::measurement_unit! {
    name: KiloWatt,
    property: Power,
    ratio: 1e3,
    suffix: " kW",
}

measures::measurement_unit! {
    name: MegaWatt,
    property: Power,
    ratio: 1e6,
    suffix: " MW",
}

measures::measurement_unit! {
    name: GigaWatt,
    property: Power,
    ratio: 1e9,
    suffix: " GW",
}

measures::measurement_unit! {
    name: ErgPerSecond,
    property: Power,
    ratio: 1e-7,
    suffix: " erg/s",
}

measures::measurement_unit! {
    name: HorsePower,
    property: Power,
    ratio: 745.699872,
    suffix: " hp",
}

// Property: pressure, stress
measures::measurement_scalar_property! { Pressure }

measures::measurement_unit! {
    name: Pascal,
    property: Pressure,
    suffix: " Pa",
}

measures::measurement_unit! {
    name: HectoPascal,
    property: Pressure,
    ratio: 100.,
    suffix: " hPa",
}

measures::measurement_unit! {
    name: Atmosphere,
    property: Pressure,
    ratio: 1.013e5,
    suffix: " atm",
}

measures::measurement_unit! {
    name: Bar,
    property: Pressure,
    ratio: 1e5,
    suffix: " bar",
}

measures::measurement_unit! {
    name: MilliBar,
    property: Pressure,
    ratio: 100.,
    suffix: " mbar",
}

measures::measurement_unit! {
    name: MmHg,
    property: Pressure,
    ratio: 133.322,
    suffix: " torr",
}

measures::measurement_unit! {
    name: PoundForcePerSquareInch,
    property: Pressure,
    ratio: 6894.757,
    suffix: " lbf/in\u{b2}", // lbf/in²
}

// Property: radiance
measures::measurement_scalar_property! { Radiance }

measures::measurement_unit! {
    name: WattPerSquareMetrePerSteradian,
    property: Radiance,
    suffix: " W/m\u{b2}/sr", // W/m²/sr
}

// Property: radiant intensity
measures::measurement_scalar_property! { RadiantIntensity }

measures::measurement_unit! {
    name: WattPerSteradian,
    property: RadiantIntensity,
    suffix: " W/sr",
}

// Property: radioactive activity
measures::measurement_scalar_property! { RadioactiveActivity }

measures::measurement_unit! {
    name: Becquerel,
    property: RadioactiveActivity,
    suffix: " Bq",
}

measures::measurement_unit! {
    name: KiloBecquerel,
    property: RadioactiveActivity,
    ratio: 1e3,
    suffix: " kBq",
}

measures::measurement_unit! {
    name: MegaBecquerel,
    property: RadioactiveActivity,
    ratio: 1e6,
    suffix: " MBq",
}

measures::measurement_unit! {
    name: GigaBecquerel,
    property: RadioactiveActivity,
    ratio: 1e9,
    suffix: " GBq",
}

// Property: radioactive dose
measures::measurement_scalar_property! { RadioactiveDose }

measures::measurement_unit! {
    name: Gray,
    property: RadioactiveDose,
    suffix: " Gy",
}

measures::measurement_unit! {
    name: Rad,
    property: RadioactiveDose,
    ratio: 0.01,
    suffix: " rad",
}

// Property: radioactive dose rate
measures::measurement_scalar_property! { RadioactiveDoseRate }

measures::measurement_unit! {
    name: GrayPerSecond,
    property: RadioactiveDoseRate,
    suffix: " Gy/s",
}

// Property: reaction rate, catalytic activity concentration
measures::measurement_scalar_property! { ReactionRate }

measures::measurement_unit! {
    name: MolePerCubicMetrePerSecond,
    property: ReactionRate,
    suffix: " mol/m\u{b3}/s", // mol/m³/s
}

// Property: solid angle
measures::measurement_scalar_property! { SolidAngle }

measures::measurement_unit! {
    name: Steradian,
    property: SolidAngle,
    suffix: " sr",
}

measures::measurement_unit! {
    name: Spat,
    property: SolidAngle,
    ratio: 2. * core::f64::consts::TAU,
    suffix: " sp",
}

measures::measurement_unit! {
    name: Sphere,
    property: SolidAngle,
    ratio: 2. * core::f64::consts::TAU,
    suffix: " sphere",
}

measures::measurement_unit! {
    name: SquareDegree,
    property: SolidAngle,
    ratio: core::f64::consts::TAU * core::f64::consts::TAU / 360. / 360.,
    suffix: " deg\u{b2}", // deg²
}

// Property: specific energy
measures::measurement_scalar_property! { SpecificEnergy }

measures::measurement_unit! {
    name: JoulePerKiloGram,
    property: SpecificEnergy,
    suffix: " J/kg",
}

// Property: specific heat capacity
measures::measurement_scalar_property! { SpecificHeatCapacity }

measures::measurement_unit! {
    name: JoulePerKiloGramPerKelvin,
    property: SpecificHeatCapacity,
    suffix: " J/kg/\u{b0}K", // J/kg/°K
}

// Property: specific volume
measures::measurement_scalar_property! { SpecificVolume }

measures::measurement_unit! {
    name: CubicMetrePerKiloGram,
    property: SpecificVolume,
    suffix: " m\u{b3}/kg", // m³/kg
}

// Property: square time
measures::measurement_scalar_property! { SquareTime }

measures::measurement_unit! {
    name: SquareSecond,
    property: SquareTime,
    suffix: " s\u{b2}", // s²
}

measures::measurement_unit! {
    name: HourSecond,
    property: SquareTime,
    ratio: 3600.,
    suffix: " h\u{b7}s", // h·s
}

measures::measurement_unit! {
    name: HourHour,
    property: SquareTime,
    ratio: 3600. * 3600.,
    suffix: " h\u{b7}h", // h·h
}

// Property: surface density
measures::measurement_scalar_property! { SurfaceDensity }

measures::measurement_unit! {
    name: KiloGramPerSquareMetre,
    property: SurfaceDensity,
    suffix: " kg/m\u{b2}", // kg/m²
}

// Property: surface tension
measures::measurement_scalar_property! { SurfaceTension }

measures::measurement_unit! {
    name: JoulePerSquareMetre,
    property: SurfaceTension,
    suffix: " J/m\u{b2}", // J/m²
}

// Property: temperature
measures::measurement_scalar_property! { Temperature }

measures::measurement_unit! {
    name: Kelvin,
    property: Temperature,
    suffix: " \u{b0}K", // °K
}

measures::measurement_unit! {
    name: Celsius,
    property: Temperature,
    ratio: 1.,
    offset: 273.15,
    suffix: " \u{b0}C", // °C
}

measures::measurement_unit! {
    name: Fahrenheit,
    property: Temperature,
    ratio: 5. / 9.,
    offset: 273.15 - 32. * 5. / 9.,
    suffix: " \u{b0}F", // °F
}

// Property: thermal conductivity
measures::measurement_scalar_property! { ThermalConductivity }

measures::measurement_unit! {
    name: WattPerMetrePerKelvin,
    property: ThermalConductivity,
    suffix: " W/m/\u{b0}K", // W/m/°K
}

// Property: time, mean lifetime
measures::measurement_scalar_property! { Time }

measures::measurement_unit! {
    name: Second,
    property: Time,
    suffix: " s",
}

measures::measurement_unit! {
    name: Year,
    property: Time,
    ratio: 365.24 * 86400.,
    suffix: " Y",
}

measures::measurement_unit! {
    name: Week,
    property: Time,
    ratio: 7. * 86400.,
    suffix: " W",
}

measures::measurement_unit! {
    name: Day,
    property: Time,
    ratio: 86400.,
    suffix: " D",
}

measures::measurement_unit! {
    name: Hour,
    property: Time,
    ratio: 3600.,
    suffix: " h",
}

measures::measurement_unit! {
    name: Minute,
    property: Time,
    ratio: 60.,
    suffix: " min",
}

measures::measurement_unit! {
    name: MilliSecond,
    property: Time,
    ratio: 1e-3,
    suffix: " ms",
}

measures::measurement_unit! {
    name: MicroSecond,
    property: Time,
    ratio: 1e-6,
    suffix: " \u{b5}s", // µs
}

measures::measurement_unit! {
    name: NanoSecond,
    property: Time,
    ratio: 1e-9,
    suffix: " ns",
}

measures::measurement_unit! {
    name: PicoSecond,
    property: Time,
    ratio: 1e-12,
    suffix: " ps",
}

measures::measurement_unit! {
    name: FemtoSecond,
    property: Time,
    ratio: 1e-15,
    suffix: " fs",
}

// Property: torque
measures::measurement_vector_property! { Torque }

measures::measurement_unit! {
    name: NewtonMetre,
    property: Torque,
    suffix: " N\u{b7}m", // N·m
}

measures::measurement_unit! {
    name: PoundFoot,
    property: Torque,
    ratio: 4.448222 * 0.3048,
    suffix: " lbf-ft",
}

measures::measurement_unit! {
    name: PoundInch,
    property: Torque,
    ratio: 4.448222 * 0.0254,
    suffix: " lbf-in",
}

// Property: velocity, speed
measures::measurement_vector_property! { Velocity }

measures::measurement_unit! {
    name: MetrePerSecond,
    property: Velocity,
    suffix: " m/s",
}

measures::measurement_unit! {
    name: Knot,
    property: Velocity,
    ratio: 1852. / 3600.,
    suffix: " kt",
}

measures::measurement_unit! {
    name: KiloMetrePerHour,
    property: Velocity,
    ratio: 1. / 3.6,
    suffix: " km/h",
}

measures::measurement_unit! {
    name: MilePerHour,
    property: Velocity,
    ratio: 1609. / 3600.,
    suffix: " mi/h",
}

measures::measurement_unit! {
    name: CentiMetrePerSecond,
    property: Velocity,
    ratio: 0.01,
    suffix: " cm/s",
}

measures::measurement_unit! {
    name: KiloMetrePerSecond,
    property: Velocity,
    ratio: 1e3,
    suffix: " km/s",
}

// Property: volume
measures::measurement_scalar_property! { Volume }

measures::measurement_unit! {
    name: CubicMetre,
    property: Volume,
    suffix: " m\u{b3}", // m³
}

measures::measurement_unit! {
    name: CubicKiloMetre,
    property: Volume,
    ratio: 1e9,
    suffix: " km\u{b3}", // km³
}

measures::measurement_unit! {
    name: CubicMicroMetre,
    property: Volume,
    ratio: 1e-18,
    suffix: " \u{b5}m\u{b3}?", // µm³
}

measures::measurement_unit! {
    name: CubicNanoMetre,
    property: Volume,
    ratio: 1e-27,
    suffix: " nm\u{b3}", // nm³
}

measures::measurement_unit! {
    name: CubicInch,
    property: Volume,
    ratio: 0.0254 * 0.0254 * 0.0254,
    suffix: " in\u{b3}", // in³
}

measures::measurement_unit! {
    name: CubicFoot,
    property: Volume,
    ratio: 0.3048 * 0.3048 * 0.3048,
    suffix: " ft\u{b3}", // ft³
}

measures::measurement_unit! {
    name: CubicYard,
    property: Volume,
    ratio: 0.9144 * 0.9144 * 0.9144,
    suffix: " yd\u{b3}", // yd³
}

measures::measurement_unit! {
    name: CubicMile,
    property: Volume,
    ratio: 1609. * 1609. * 1609.,
    suffix: " mi\u{b3}", //mi³
}

measures::measurement_unit! {
    name: Litre,
    property: Volume,
    ratio: 1e-3,
    suffix: " l",
}

measures::measurement_unit! {
    name: CubicDecimetre,
    property: Volume,
    ratio: 1e-3,
    suffix: " dm\u{b3}", //dm³
}

measures::measurement_unit! {
    name: MilliLitre,
    property: Volume,
    ratio: 1e-6,
    suffix: " ml",
}

measures::measurement_unit! {
    name: CubicCentimetre,
    property: Volume,
    ratio: 1e-6,
    suffix: " cm\u{b3}", // cm³
}

measures::measurement_unit! {
    name: MicroLitre,
    property: Volume,
    ratio: 1e-9,
    suffix: " \u{b5}l", // µl
}

measures::measurement_unit! {
    name: CubicMillimetre,
    property: Volume,
    ratio: 1e-9,
    suffix: " mm\u{b3}", // mm³
}

measures::measurement_unit! {
    name: NanoLitre,
    property: Volume,
    ratio: 1e-12,
    suffix: " nl",
}

measures::measurement_unit! {
    name: PicoLitre,
    property: Volume,
    ratio: 1e-15,
    suffix: " pl",
}

measures::measurement_unit! {
    name: Pint,
    property: Volume,
    ratio: 473.2e-6,
    suffix: " pt",
}

measures::measurement_unit! {
    name: Gallon,
    property: Volume,
    ratio: 4546e-6,
    suffix: " gal",
}

// Property: volumetric flow rate
measures::measurement_scalar_property! { VolumetricFlowRate }

measures::measurement_unit! {
    name: CubicMetrePerSecond,
    property: VolumetricFlowRate,
    suffix: " m\u{b3}/s", // m³/s
}

measures::measurement_unit! {
    name: MilliLitrePerSecond,
    property: VolumetricFlowRate,
    ratio: 1e-6,
    suffix: " ml/s",
}

measures::measurement_unit! {
    name: CubicCentimetrePerSecond,
    property: VolumetricFlowRate,
    ratio: 1e-6,
    suffix: " cm\u{b3}/s", // cm³/s
}

// Property: wave number
measures::measurement_scalar_property! { WaveNumber }

measures::measurement_unit! {
    name: CyclePerMetre,
    property: WaveNumber,
    suffix: " 1/m",
}

measures::measurement_unit! {
    name: RadianPerMetre,
    property: WaveNumber,
    ratio: 1. / core::f64::consts::TAU,
    suffix: " rad/m",
}
