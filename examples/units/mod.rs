measures::define_measure_types! {
    with_points with_directions with_2d with_3d with_transformations exact with_approx,
    scalar_properties [
        Action [
            JouleSecond {
                suffix: " J\u{b7}s", // J·s
            }
        ]
        Amount [
            Unit {
                suffix: " u.",
            }
            Dozen {
                suffix: " dz.",
                ratio: 12.,
            }
            Mole {
                suffix: " mol",
                ratio: 6.0221413e23,
            }
        ]
        Area [
            SquareMetre {
                suffix: " m\u{b2}", // m²
            }
            SquareKilometre {
                suffix: " km\u{b2}", // km²
                ratio: 1e6,
            }
            Hectare {
                suffix: " ha",
                ratio: 1e4,
            }
            Are {
                suffix: " are",
                ratio: 100.,
            }
            SquareDecimetre {
                suffix: " dm\u{b2}", // dm²
                ratio: 0.01,
            }
            SquareCentimetre {
                suffix: " cm\u{b2}", // cm²
                ratio: 1e-4,
            }
            SquareMillimetre {
                suffix: " mm\u{b2}", // mm²
                ratio: 1e-6,
            }
            SquareMicrometre {
                suffix: " \u{b5}m\u{b2}", // µm²
                ratio: 1e-12,
            }
            SquareNanometre {
                suffix: " nm\u{b2}", // nm²
                ratio: 1e-18,
            }
            SquareInch {
                suffix: " in\u{b2}", // in²
                ratio: 0.0254 * 0.0254,
            }
            SquareFoot {
                suffix: " ft\u{b2}", // ft²
                ratio: 0.3048 * 0.3048,
            }
            SquareYard {
                suffix: " yd\u{b2}", // yd²
                ratio: 0.9144 * 0.9144,
            }
            SquareMile {
                suffix: " mi\u{b2}", // mi²
                ratio: 1609.344 * 1609.344,
            }
        ]
        Capacitance [
            Farad {
                suffix: " F",
            }
            Millifarad {
                suffix: " mF",
                ratio: 1e-3,
            }
            Microfarad {
                suffix: " \u{b5}F", // µF
                ratio: 1e-6,
            }
            Nanofarad {
                suffix: " nF",
                ratio: 1e-9,
            }
            Picofarad {
                suffix: " pF",
                ratio: 1e-12,
            }
        ]
        CatalyticActivity [
            Katal {
                suffix: " kat",
            }
        ]
        ChemicalPotential [
            JoulePerMole {
                suffix: " J/mol",
            }
        ]
        DoseEquivalent [
            Sievert {
                suffix: " Sv",
            }
            Rem {
                suffix: " rem",
                ratio: 0.01,
            }
        ]
        DynamicViscosity [
            PascalSecond {
                suffix: " Pa\u{b7}s", // Pa·s
            }
        ]
        ElectricalConductance [
            Siemens {
                suffix: " S",
            }
        ]
        ElectricalConductivity [
            SiemensPerMetre {
                suffix: " S/m",
            }
        ]
        ElectricalResistance [
            Ohm {
                suffix: " \u{3a9}", // Ω
            }
            Milliohm {
                suffix: " m\u{3a9}", // mΩ
                ratio: 1e-3,
            }
            Kiloohm {
                suffix: " k\u{3a9}", // kΩ
                ratio: 1e3,
            }
        ]
        ElectricalResistivity [
            OhmMetre {
                suffix: " \u{3a9}\u{b7}m", // Ω·m
            }
        ]
        ElectricCharge [
            Coulomb {
                suffix: " C",
            }
            Millicoulomb {
                suffix: " mC",
                ratio: 1e-3,
            }
            Microcoulomb {
                suffix: " \u{b5}C", // µC
                ratio: 1e-6,
            }
            Nanocoulomb {
                suffix: " ",
                ratio: 1e-9,
            }
            Picocoulomb {
                suffix: " pC",
                ratio: 1e-12,
            }
        ]
        ElectricChargeDensity [
            CoulombPerCubicMetre {
                suffix: " C/m\u{b3}", // C/m³
            }
        ]
        ElectricCurrent [
            Ampere {
                suffix: " A",
            }
            Milliampere {
                suffix: " mA",
                ratio: 1e-3,
            }

            Microampere {
                suffix: " \u{b5}A", // µA
                ratio: 1e-6,
            }
        ]
        ElectricDisplacement [
            CoulombPerSquareMetre {
                suffix: " C/m\u{b2}", // C/m²
            }
        ]
        ElectricPotential [
            Volt {
                suffix: " V",
            }
            Kilovolt {
                suffix: " kV",
                ratio: 1e3,
            }
            Millivolt {
                suffix: " mV",
                ratio: 1e-3,
            }
            Microvolt {
                suffix: " \u{b5}V", // µV
                ratio: 1e-6,
            }
        ]
        Energy [
            Joule {
                suffix: " J",
            }
            Erg {
                suffix: " erg",
                ratio: 1e-7,
            }
            WattHour {
                suffix: " W\u{b7}h", // W·h
                ratio: 3600.,
            }
            KilowattHour {
                suffix: " kW\u{b7}h", // kW·h
                ratio: 3.6e6,
            }
            MegawattHour {
                suffix: " MW\u{b7}h", // MW·h
                ratio: 3.6e9,
            }
            Calorie {
                suffix: " cal",
                ratio: 4.187,
            }
            Kilocalorie {
                suffix: " kcal",
                ratio: 4187.,
            }
            Electronvolt {
                suffix: " eV",
                ratio: 1.602176634e-19,
            }
            Kiloelectronvolt {
                suffix: " keV",
                ratio: 1.602176634e-16,
            }
            Megaelectronvolt {
                suffix: " MeV",
                ratio: 1.602176634e-13,
            }
            Gigaelectronvolt {
                suffix: " GeV",
                ratio: 1.602176634e-10,
            }
            Teraelectronvolt {
                suffix: " TeV",
                ratio: 1.602176634e-7,
            }
        ]
        EnergyDensity [
            JoulePerCubicMetre {
                suffix: " J/m\u{b3}", // J/m³
            }
        ]
        Entropy [
            JoulePerKelvin {
                suffix: " J/\u{b0}K", // J/°K
            }
        ]
        Frequency [
            Hertz {
                suffix: " Hz",
            }
            CyclePerSecond { // Equivalent to Hertz, but with different suffix.
                suffix: " c/s",
            }
            Kilohertz {
                suffix: " kHz",
                ratio: 1e3,
            }
            Megahertz {
                suffix: " MHz",
                ratio: 1e6,
            }
            Gigahertz {
                suffix: " GHz",
                ratio: 1e9,
            }
            RadianPerSecond {
                suffix: " rad/s",
                ratio: 1. / core::f64::consts::TAU,
            }
            CyclePerMinute {
                suffix: " rpm",
                ratio: 1. / 60.,
            }
        ]
        Illuminance [
            Lux {
                suffix: " lx",
            }
            Phot {
                suffix: " phot",
                ratio: 1e4,
            }

            FootCandle {
                suffix: " fc",
                ratio: 10.764,
            }
        ]
        Inductance [
            Henry {
                suffix: " H",
            }
        ]
        Information [
            Bit {
                suffix: " b",
            }
            Byte {
                suffix: " B",
                ratio: 8.,
            }
            Kilobit {
                suffix: " kb",
                ratio: 1e3,
            }
            Kilobyte {
                suffix: " kB",
                ratio: 8e3,
            }
            Kibibit {
                suffix: " kib",
                ratio: 1024.,
            }
            Kibibyte {
                suffix: " kiB",
                ratio: 8. * 1024.,
            }
            Megabit {
                suffix: " Mb",
                ratio: 1e6,
            }
            Megabyte {
                suffix: " MB",
                ratio: 8e6,
            }
            Mebibit {
                suffix: " Mib",
                ratio: 1024. * 1024.,
            }
            Mebibyte {
                suffix: " MiB",
                ratio: 8. * 1024. * 1024.,
            }
            Gigabit {
                suffix: " Gb",
                ratio: 1e9,
            }
            Gigabyte {
                suffix: " GB",
                ratio: 8e9,
            }
            Gibibit {
                suffix: " Gib",
                ratio: 1024. * 1024. * 1024.,
            }
            Gibibyte {
                suffix: " GiB",
                ratio: 8. * 1024. * 1024. * 1024.,
            }
            Terabit {
                suffix: " Tb",
                ratio: 1e12,
            }
            Terabyte {
                suffix: " TB",
                ratio: 8e12,
            }
            Tebibit {
                suffix: " TiB",
                ratio: 1024. * 1024. * 1024. * 1024.,
            }
            Tebibyte {
                suffix: " TiB",
                ratio: 8. * 1024. * 1024. * 1024. * 1024.,
            }
        ]
        InformationRate [
            BitPerSecond {
                suffix: " b/s",
            }
            BytePerSecond {
                suffix: " B/s",
                ratio: 8.,
            }
            KilobitPerSecond {
                suffix: " kb/s",
                ratio: 1e3,
            }
            KilobytePerSecond {
                suffix: " kB/s",
                ratio: 8e3,
            }
            KibibitPerSecond {
                suffix: " kib/s",
                ratio: 1024.,
            }
            KibibytePerSecond {
                suffix: " kiB/s",
                ratio: 8. * 1024.,
            }
            MegabitPerSecond {
                suffix: " Mb/s",
                ratio: 1e6,
            }
            MegabytePerSecond {
                suffix: " MB/s",
                ratio: 8e6,
            }
            MebibitPerSecond {
                suffix: " Mib/s",
                ratio: 1024. * 1024.,
            }
            MebibytePerSecond {
                suffix: " MiB/s",
                ratio: 8. * 1024. * 1024.,
            }
            GigabitPerSecond {
                suffix: " Gb/s",
                ratio: 1e9,
            }
            GigabytePerSecond {
                suffix: " GB/s",
                ratio: 8e9,
            }
            GibibitPerSecond {
                suffix: " Gib/s",
                ratio: 1024. * 1024. * 1024.,
            }
            GibibytePerSecond {
                suffix: " GiB/s",
                ratio: 8. * 1024. * 1024. * 1024.,
            }
            TerabitPerSecond {
                suffix: " Tb/s",
                ratio: 1e12,
            }
            TerabytePerSecond {
                suffix: " TB/s",
                ratio: 8e12,
            }
            TebibitPerSecond {
                suffix: " Tib/s",
                ratio: 1024. * 1024. * 1024. * 1024.,
            }
            TebibytePerSecond {
                suffix: " TiB/s",
                ratio: 8. * 1024. * 1024. * 1024. * 1024.,
            }
        ]
        Irradiance [
            WattPerSquareMetre {
                suffix: " W/m\u{b2}", // W/m²
            }
        ]
        KinematicViscosity [
            SquareMetrePerSecond {
                suffix: " m\u{b2}/s", // m²/s
            }
            Stoke {
                suffix: " St",
                ratio: 1e-4,
            }
            Centistoke {
                suffix: " cSt",
                ratio: 1e-6,
            }
        ]
        LinearDensity [
            KilogramPerMetre {
                suffix: " kg/m",
            }
            GramPerCentimetre {
                suffix: " g/cm",
                ratio: 0.1,
            }
        ]
        LinearElectricChargeDensity [
            CoulombPerMetre {
                suffix: " C/m",
            }
        ]
        Luminance [
            CandelaPerSquareMetre {
                suffix: " cd/m\u{b2}", // cd/m²
            }
            Nit { // Equivalent to CandelaPerSquareMetre, but with different suffix.
                suffix: " nt",
            }
            Stilb {
                suffix: " sb",
                ratio: 1e4,
            }
            CandelaPerSquareFoot {
                suffix: " cd/ft\u{b2}", // cd/ft²
                ratio: 10.764,
            }
        ]
        LuminousIntensity [
            Candela {
                suffix: " cd",
            }
        ]
        MagneticFlux [
            Weber {
                suffix: " Wb",
            }
        ]
        MagneticPermeability [
            HenryPerMetre {
                suffix: " H/m",
            }
        ]
        MagneticReluctance [
            InverseHenry {
                suffix: " 1/H",
            }
        ]
        Mass [
            Kilogram {
                suffix: " kg",
            }
            Tonne {
                suffix: " t",
                ratio: 1e3,
            }
            Megagram {
                suffix: " Mg",
                ratio: 1e3,
            }
            Hectogram {
                suffix: " hg",
                ratio: 0.1,
            }
            Decagram {
                suffix: " dag",
                ratio: 0.01,
            }
            Gram {
                suffix: " g",
                ratio: 1e-3,
            }
            Milligram {
                suffix: " mg",
                ratio: 1e-6,
            }
            Microgram {
                suffix: " \u{b5}g", // µg
                ratio: 1e-9,
            }
            Nanogram {
                suffix: " ng",
                ratio: 1e-12,
            }
            ImperialTon {
                suffix: " t",
                ratio: 1016.0469,
            }
            USTon {
                suffix: " t",
                ratio: 907.18474,
            }
            Stone {
                suffix: " st.",
                ratio: 6.35029,
            }
            Pound {
                suffix: " lb",
                ratio: 0.45359237,
            }
            Ounce {
                suffix: " oz",
                ratio: 0.028349523,
            }
            Carat {
                suffix: " ct",
                ratio: 2e-4,
            }
        ]
        MassDensity [
            KilogramPerCubicMetre {
                suffix: " kg/m\u{b3}", // kg/m³
            }
            GramPerMillilitre {
                suffix: " g/ml",
                ratio: 1e3,
            }
        ]
        MassFlowRate [
            KilogramPerSecond {
                suffix: " kg/s",
            }
            GramPerSecond {
                suffix: " g/s",
                ratio: 1e-3,
            }
        ]
        MolarConcentration [
            MolePerCubicMetre {
                suffix: " mol/m\u{b3}", // mol/m³
            }
        ]
        MolarHeatCapacity [
            JoulePerKelvinPerMole {
                suffix: " J/\u{b0}K/mol", // J/°K/mol
            }
        ]
        MomentOfInertia [
            KilogramSquareMetre {
                suffix: " kg\u{b7}m\u{b2}", // kg·m²
            }
            GramSquareCentimetre {
                suffix: " g\u{b7}cm\u{b2}", // g·cm²
                ratio: 1e-7,
            }
        ]
        Permittivity [
            FaradPerMetre {
                suffix: " F/m",
            }
        ]
        Power [
            Watt {
                suffix: " W",
            }
            Milliwatt {
                suffix: " mW",
                ratio: 1e-3,
            }
            Kilowatt {
                suffix: " kW",
                ratio: 1e3,
            }
            Megawatt {
                suffix: " MW",
                ratio: 1e6,
            }
            Gigawatt {
                suffix: " GW",
                ratio: 1e9,
            }
            ErgPerSecond {
                suffix: " erg/s",
                ratio: 1e-7,
            }
            HorsePower {
                suffix: " hp",
                ratio: 745.699872,
            }
        ]
        Pressure [
            Pascal {
                suffix: " Pa",
            }
            Hectopascal {
                suffix: " hPa",
                ratio: 100.,
            }
            Atmosphere {
                suffix: " atm",
                ratio: 1.013e5,
            }
            Bar {
                suffix: " bar",
                ratio: 1e5,
            }
            Millibar {
                suffix: " mbar",
                ratio: 100.,
            }
            MmHg {
                suffix: " torr",
                ratio: 133.322,
            }
            PoundForcePerSquareInch {
                suffix: " lbf/in\u{b2}", // lbf/in²
                ratio: 6894.757,
            }
        ]
        Radiance [
            WattPerSquareMetrePerSteradian {
                suffix: " W/m\u{b2}/sr", // W/m²/sr
            }
        ]
        RadiantIntensity [
            WattPerSteradian {
                suffix: " W/sr",
            }
        ]
        RadioactiveActivity [
            Becquerel {
                suffix: " Bq",
            }
            Kilobecquerel {
                suffix: " kBq",
                ratio: 1e3,
            }
            Megabecquerel {
                suffix: " MBq",
                ratio: 1e6,
            }
            Gigabecquerel {
                suffix: " GBq",
                ratio: 1e9,
            }
        ]
        RadioactiveDose [
            Gray {
                suffix: " Gy",
            }
            Rad {
                suffix: " rad",
                ratio: 0.01,
            }
        ]
        RadioactiveDoseRate [
            GrayPerSecond {
                suffix: " Gy/s",
            }
        ]
        ReactionRate [
            MolePerCubicMetrePerSecond {
                suffix: " mol/m\u{b3}/s", // mol/m³/s
            }
        ]
        SolidAngle [
            Steradian {
                suffix: " sr",
            }
            Spat {
                suffix: " sp",
                ratio: 2. * core::f64::consts::TAU,
            }
            Sphere {
                suffix: " sphere",
                ratio: 2. * core::f64::consts::TAU,
            }
            SquareDegree {
                suffix: " deg\u{b2}", // deg²
                ratio: core::f64::consts::TAU * core::f64::consts::TAU / 360. / 360.,
            }
        ]
        SpecificEnergy [
            JoulePerKilogram {
                suffix: " J/kg",
            }
        ]
        SpecificHeatCapacity [
            JoulePerKilogramPerKelvin {
                suffix: " J/kg/\u{b0}K", // J/kg/°K
            }
        ]
        SpecificVolume [
            CubicMetrePerKilogram {
                suffix: " m\u{b3}/kg", // m³/kg
            }
        ]
        SquareTime [
            SquareSecond {
                suffix: " s\u{b2}", // s²
            }
            HourSecond {
                suffix: " h\u{b7}s", // h·s
                ratio: 3600.,
            }
            HourHour {
                suffix: " h\u{b7}h", // h·h
                ratio: 3600. * 3600.,
            }
        ]
        SurfaceDensity [
            KilogramPerSquareMetre {
                suffix: " kg/m\u{b2}", // kg/m²
            }
        ]
        SurfaceTension [
            JoulePerSquareMetre {
                suffix: " J/m\u{b2}", // J/m²
            }
        ]
        Temperature [
            Kelvin {
                suffix: " \u{b0}K", // °K
            }
            Celsius {
                suffix: " \u{b0}C", // °C
                ratio: 1.,
                offset: 273.15,
            }
            Fahrenheit {
                suffix: " \u{b0}F", // °F
                ratio: 5. / 9.,
                offset: 273.15 - 32. * 5. / 9.,
            }
        ]
        ThermalConductivity [
            WattPerMetrePerKelvin {
                suffix: " W/m/\u{b0}K", // W/m/°K
            }
        ]
        Time [
            Second {
                suffix: " s",
            }
            Year {
                suffix: " Y",
                ratio: 365.24 * 86400.,
            }
            Week {
                suffix: " W",
                ratio: 7. * 86400.,
            }
            Day {
                suffix: " D",
                ratio: 86400.,
            }
            Hour {
                suffix: " h",
                ratio: 3600.,
            }
            Minute {
                suffix: " min",
                ratio: 60.,
            }
            Millisecond {
                suffix: " ms",
                ratio: 1e-3,
            }
            Microsecond {
                suffix: " \u{b5}s", // µs
                ratio: 1e-6,
            }
            Nanosecond {
                suffix: " ns",
                ratio: 1e-9,
            }
            Picosecond {
                suffix: " ps",
                ratio: 1e-12,
            }
            Femtosecond {
                suffix: " fs",
                ratio: 1e-15,
            }
        ]
        Volume [
            CubicMetre {
                suffix: " m\u{b3}", // m³
            }
            CubicKilometre {
                suffix: " km\u{b3}", // km³
                ratio: 1e9,
            }
            CubicMicrometre {
                suffix: " \u{b5}m\u{b3}?", // µm³
                ratio: 1e-18,
            }
            CubicNanometre {
                suffix: " nm\u{b3}", // nm³
                ratio: 1e-27,
            }
            CubicInch {
                suffix: " in\u{b3}", // in³
                ratio: 0.0254 * 0.0254 * 0.0254,
            }
            CubicFoot {
                suffix: " ft\u{b3}", // ft³
                ratio: 0.3048 * 0.3048 * 0.3048,
            }
            CubicYard {
                suffix: " yd\u{b3}", // yd³
                ratio: 0.9144 * 0.9144 * 0.9144,
            }
            CubicMile {
                suffix: " mi\u{b3}", //mi³
                ratio: 1609.344 * 1609.344 * 1609.344,
            }
            Litre {
                suffix: " l",
                ratio: 1e-3,
            }
            CubicDecimetre {
                suffix: " dm\u{b3}", //dm³
                ratio: 1e-3,
            }
            Millilitre {
                suffix: " ml",
                ratio: 1e-6,
            }
            CubicCentimetre {
                suffix: " cm\u{b3}", // cm³
                ratio: 1e-6,
            }
            Microlitre {
                suffix: " \u{b5}l", // µl
                ratio: 1e-9,
            }
            CubicMillimetre {
                suffix: " mm\u{b3}", // mm³
                ratio: 1e-9,
            }
            Nanolitre {
                suffix: " nl",
                ratio: 1e-12,
            }
            Picolitre {
                suffix: " pl",
                ratio: 1e-15,
            }
            Pint {
                suffix: " pt",
                ratio: 473.2e-6,
            }
            Gallon {
                suffix: " gal",
                ratio: 4546e-6,
            }
        ]
        VolumetricFlowRate [
            CubicMetrePerSecond {
                suffix: " m\u{b3}/s", // m³/s
            }
            MillilitrePerSecond {
                suffix: " ml/s",
                ratio: 1e-6,
            }
            CubicCentimetrePerSecond {
                suffix: " cm\u{b3}/s", // cm³/s
                ratio: 1e-6,
            }
        ]
        WaveNumber [
            CyclePerMetre {
                suffix: " 1/m",
            }
            RadianPerMetre {
                suffix: " rad/m",
                ratio: 1. / core::f64::consts::TAU,
            }
        ]
    ]
    vector_properties [
        Acceleration [
            MetrePerSquareSecond {
                suffix: " m/s\u{b2}", // m/s²
            }
            CentimetrePerSquareSecond {
                suffix: " cm/s\u{b2}", // cm/s²
                ratio: 1e-2,
            }
            GForce {
                suffix: " g",
                ratio: 9.80665,
            }
            KilometrePerHourPerSecond {
                suffix: " km/h/s",
                ratio: 1. / 3.6,
            }
        ]
        AngularAcceleration [
            RadianPerSquareSecond {
                suffix: " rad/s\u{b2}", // rad/s²
            }
        ]
        AngularMomentum [
            KilogramSquareMetrePerSecond {
                suffix: " kg\u{b7}m\u{b2}/s", // kg·m²/s
            }
            GramSquareCentimetrePerSecond {
                suffix: " g\u{b7}cm\u{b2}/s", // g·cm²/s
                ratio: 1e-7,
            }
        ]
        CurrentDensity [
            AmperePerSquareMetre {
                suffix: " A/m\u{b2}", // A/m²
            }
        ]
        ElectricFieldStrength [
            VoltPerMetre {
                suffix: " V/m",
            }
            NewtonPerCoulomb {
                suffix: " N/C",
            }
        ]
        Force [
            Newton {
                suffix: " N",
            }
            Dyne {
                suffix: " dyn",
                ratio: 1e-5,
            }
            KilogramForce {
                suffix: " kgf",
                ratio: 9.80665,
            }
            PoundForce {
                suffix: " lbf",
                ratio: 4.448222,
            }
            Poundal {
                suffix: " pdl",
                ratio: 0.138255,
            }
        ]
        Length [
            Metre {
                suffix: " m",
            }
            AstronomicalUnit {
                suffix: " a.u.",
                ratio: 149597870691.,
            }
            Parsec {
                suffix: " psc",
                ratio: 3.0856775813e16,
            }
            LightYear {
                suffix: " ly",
                ratio: 31557600. * 2.99792458e8,
            }
            Kilometre {
                suffix: " km",
                ratio: 1e3,
            }
            Hectometre {
                suffix: " hm",
                ratio: 100.,
            }
            Decametre {
                suffix: " dam",
                ratio: 10.,
            }
            Decimetre {
                suffix: " dm",
                ratio: 0.1,
            }
            Centimetre {
                suffix: " cm",
                ratio: 0.01,
            }
            Millimetre {
                suffix: " mm",
                ratio: 1e-3,
            }
            Micrometre {
                suffix: " \u{b5}m", // µm
                ratio: 1e-6,
            }
            Nanometre {
                suffix: " nm",
                ratio: 1e-9,
            }
            Angstrom {
                suffix: " \u{212b}", // Å
                ratio: 1e-10,
            }
            Inch {
                suffix: " in",
                ratio: 0.0254,
            }
            Foot {
                suffix: " ft",
                ratio: 0.3048,
            }
            Yard {
                suffix: " yd",
                ratio: 0.9144,
            }
            Mile {
                suffix: " mi",
                ratio: 1609.344,
            }
            NauticalMile {
                suffix: " naut.mi",
                ratio: 1852.,
            }
        ]
        LuminousFlux [
            Lumen {
                suffix: " lm",
            }
        ]
        MagneticFieldStrength [
            AmperePerMetre {
                suffix: " A/m",
            }
        ]
        MagneticFluxDensity [
            Tesla {
                suffix: " T",
            }
            Gauss {
                suffix: " G",
                ratio: 1e-4,
            }
        ]
        Momentum [
            NewtonSecond {
                suffix: " N\u{b7}s", // N·s
            }
            KilogramMetrePerSecond {
                suffix: " kg\u{b7}m/s", // kg·m/s
            }
            DyneSecond {
                suffix: " dyn\u{b7}s", // dyn·s
                ratio: 1e-5,
            }
            GramCentimetrePerSecond {
                suffix: " g\u{b7}cm/s", // g·cm/s
                ratio: 1e-5,
            }
        ]
        Torque [
            NewtonMetre {
                suffix: " N\u{b7}m", // N·m
            }
            PoundFoot {
                suffix: " lbf-ft",
                ratio: 4.448222 * 0.3048,
            }
            PoundInch {
                suffix: " lbf-in",
                ratio: 4.448222 * 0.0254,
            }
        ]
        Velocity [
            MetrePerSecond {
                suffix: " m/s",
            }
            Knot {
                suffix: " kt",
                ratio: 1852. / 3600.,
            }
            KilometrePerHour {
                suffix: " km/h",
                ratio: 1. / 3.6,
            }
            MilePerHour {
                suffix: " mi/h",
                ratio: 1609.344 / 3600.,
            }
            CentimetrePerSecond {
                suffix: " cm/s",
                ratio: 0.01,
            }
            KilometrePerSecond {
                suffix: " km/s",
                ratio: 1e3,
            }
        ]
    ]
    angle_measurement_units [
        Cycle {
            suffix: " rev",
            cycle_fraction: 1.,
        }
        Gradian {
            suffix: " grad",
            cycle_fraction: 400.,
        }
        Degree {
            suffix: " deg",
            cycle_fraction: 360.,
        }
        ArcMinute {
            suffix: " deg'",
            cycle_fraction: 360. * 60.,
        }
        ArcSecond {
            suffix: " deg\"",
            cycle_fraction: 360. * 60. * 60.,
        }
    ]
    relationships [
        // Computer science

        // Properties:
        // * Information
        // * InformationRate

        // InformationRate == Information / Time
        Bit 1 == BitPerSecond 1 * Second 1,
        Byte 1 == BytePerSecond 1 * Second 1,
        Kilobit 1 == KilobitPerSecond 1 * Second 1,
        Kilobyte 1 == KilobytePerSecond 1 * Second 1,
        Kibibit 1 == KibibitPerSecond 1 * Second 1,
        Kibibyte 1 == KibibytePerSecond 1 * Second 1,
        Megabit 1 == MegabitPerSecond 1 * Second 1,
        Megabyte 1 == MegabytePerSecond 1 * Second 1,
        Mebibit 1 == MebibitPerSecond 1 * Second 1,
        Mebibyte 1 == MebibytePerSecond 1 * Second 1,
        Gigabit 1 == GigabitPerSecond 1 * Second 1,
        Gigabyte 1 == GigabytePerSecond 1 * Second 1,
        Gibibit 1 == GibibitPerSecond 1 * Second 1,
        Gibibyte 1 == GibibytePerSecond 1 * Second 1,
        Terabit 1 == TerabitPerSecond 1 * Second 1,
        Terabyte 1 == TerabytePerSecond 1 * Second 1,
        Tebibit 1 == TebibitPerSecond 1 * Second 1,
        Tebibyte 1 == TebibytePerSecond 1 * Second 1,

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
        SquareKilometre 1 == Kilometre 1 * __ 1,
        Hectare 1 == Hectometre 1 * __ 1,
        Are 1 == Decametre 1 * __ 1,
        SquareDecimetre 1 == Decimetre 1 * __ 1,
        SquareCentimetre 1 == Centimetre 1 * __ 1,
        SquareMillimetre 1 == Millimetre 1 * __ 1,
        SquareMicrometre 1 == Micrometre 1 * __ 1,
        SquareNanometre 1 == Nanometre 1 * __ 1,
        SquareInch 1 == Inch 1 * __ 1,
        SquareFoot 1 == Foot 1 * __ 1,
        SquareYard 1 == Yard 1 * __ 1,
        SquareMile 1 == Mile 1 * __ 1,

        // Volume == Area * Length
        CubicMetre 1 == SquareMetre 1 * Metre 1,
        CubicKilometre 1 == SquareKilometre 1 * Kilometre 1,
        Litre 1 == SquareDecimetre 1 * Decimetre 1,
        Millilitre 1 == SquareCentimetre 1 * Centimetre 1,
        Microlitre 1 == SquareMillimetre 1 * Millimetre 1,
        CubicMicrometre 1 == SquareMicrometre 1 * Micrometre 1,
        CubicNanometre 1 == SquareNanometre 1 * Nanometre 1,
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
        CentimetrePerSecond 1 == CentimetrePerSquareSecond 1 * Second 1,
        CentimetrePerSecond 2 == CentimetrePerSquareSecond 2 * Second 1,
        CentimetrePerSecond 3 == CentimetrePerSquareSecond 3 * Second 1,
        KilometrePerHour 1 == KilometrePerHourPerSecond 1 * Second 1,
        KilometrePerHour 2 == KilometrePerHourPerSecond 2 * Second 1,
        KilometrePerHour 3 == KilometrePerHourPerSecond 3 * Second 1,

        // Velocity == Length / Time
        Metre 1 == MetrePerSecond 1 * Second 1,
        Metre 2 == MetrePerSecond 2 * Second 1,
        Metre 3 == MetrePerSecond 3 * Second 1,
        NauticalMile 1 == Knot 1 * Hour 1,
        NauticalMile 2 == Knot 2 * Hour 1,
        NauticalMile 3 == Knot 3 * Hour 1,
        Kilometre 1 == KilometrePerHour 1 * Hour 1,
        Kilometre 2 == KilometrePerHour 2 * Hour 1,
        Kilometre 3 == KilometrePerHour 3 * Hour 1,
        Mile 1 == MilePerHour 1 * Hour 1,
        Mile 2 == MilePerHour 2 * Hour 1,
        Mile 3 == MilePerHour 3 * Hour 1,
        Centimetre 1 == CentimetrePerSecond 1 * Second 1,
        Centimetre 2 == CentimetrePerSecond 2 * Second 1,
        Centimetre 3 == CentimetrePerSecond 3 * Second 1,

        // AngularAcceleration == Frequency / Time
        RadianPerSecond 1 == RadianPerSquareSecond 1 * Second 1,

        // Frequency == Angle / Time
        Cycle 1 == Hertz 1 * Second 1,
        Radian 1 == RadianPerSecond 1 * Second 1,
        Cycle 1 == CyclePerMinute 1 * Minute 1,

        // KinematicViscosity == Area / Time
        SquareMetre 1 == SquareMetrePerSecond 1 * Second 1,
        SquareMillimetre 1 == SquareMetrePerSecond 1 * Microsecond 1,
        SquareMicrometre 1 == SquareMetrePerSecond 1 * Picosecond 1,
        SquareCentimetre 1 == Stoke 1 * Second 1,
        SquareMillimetre 1 == Centistoke 1 * Second 1,
        SquareMicrometre 1 == Centistoke 1 * Microsecond 1,
        SquareNanometre 1 == Centistoke 1 * Picosecond 1,

        // KinematicViscosity == Length * Velocity
        SquareMetrePerSecond 1 == Metre 1 * MetrePerSecond 1,
        SquareMetrePerSecond 1 == Hectometre 1 * CentimetrePerSecond 1,
        Stoke 1 == Centimetre 1 * CentimetrePerSecond 1,

        // SquareTime == Time * Time
        SquareSecond 1 == Second 1 * __ 1,
        HourSecond 1 == Hour 1 * Second 1,
        HourHour 1 == Hour 1 * __ 1,

        // VolumetricFlowRate == Volume / Time
        CubicMetre 1 == CubicMetrePerSecond 1 * Second 1,
        Millilitre 1 == MillilitrePerSecond 1 * Second 1,

        // VolumetricFlowRate == Area * Velocity
        CubicMetrePerSecond 1 == SquareMetre 1 * MetrePerSecond 1,
        MillilitrePerSecond 1 == SquareCentimetre 1 * CentimetrePerSecond 1,

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
        KilogramSquareMetrePerSecond 1 == KilogramMetrePerSecond 2 X Metre 2,
        KilogramSquareMetrePerSecond 3 == KilogramMetrePerSecond 3 X Metre 3,

        // AngularMomentum == MomentOfInertia / Time
        KilogramSquareMetre 1 == KilogramSquareMetrePerSecond 1 * Second 1,

        // DynamicViscosity == Pressure * Time
        PascalSecond 1 == Pascal 1 * Second 1,

        // Force
        //Newton 1 == Newton 1 * One 1,
        //Newton 2 == Newton 2 * One 1,
        //Newton 3 == Newton 3 * One 1,

        //Newton 2 == Newton 1 * One 2,
        //Newton 3 == Newton 1 * One 3,

        // Energy == Force * Length
        Joule 1 == Newton 1 * Metre 1,
        Joule 1 == Newton 2 * Metre 2,
        Joule 1 == Newton 3 * Metre 3,
        Erg 1 == Dyne 1 * Centimetre 1,
        Erg 1 == Dyne 2 * Centimetre 2,
        Erg 1 == Dyne 3 * Centimetre 3,

        // Energy == Momentum * Velocity
        Joule 1 == NewtonSecond 1 * MetrePerSecond 1,
        Joule 1 == NewtonSecond 2 * MetrePerSecond 2,
        Joule 1 == NewtonSecond 3 * MetrePerSecond 3,
        Erg 1 == DyneSecond 1 * CentimetrePerSecond 1,
        Erg 1 == DyneSecond 2 * CentimetrePerSecond 2,
        Erg 1 == DyneSecond 3 * CentimetrePerSecond 3,

        // Energy == MomentOfInertia / SquareTime
        KilogramSquareMetre 1 == Joule 1 * SquareSecond 1,

        // EnergyDensity == Energy / Volume
        Joule 1 == JoulePerCubicMetre 1 * CubicMetre 1,

        // Force == Mass * Acceleration
        Newton 1 == Kilogram 1 * MetrePerSquareSecond 1,
        Newton 2 == Kilogram 1 * MetrePerSquareSecond 2,
        Newton 3 == Kilogram 1 * MetrePerSquareSecond 3,
        Dyne 1 == Gram 1 * CentimetrePerSquareSecond 1,
        Dyne 2 == Gram 1 * CentimetrePerSquareSecond 2,
        Dyne 3 == Gram 1 * CentimetrePerSquareSecond 3,
        KilogramForce 1 == Kilogram 1 * GForce 1,
        KilogramForce 2 == Kilogram 1 * GForce 2,
        KilogramForce 3 == Kilogram 1 * GForce 3,

        // LinearDensity == Mass / Length
        Kilogram 1 == KilogramPerMetre 1 * Metre 1,
        Gram 1 == GramPerCentimetre 1 * Centimetre 1,

        // MassDensity == Mass / Volume
        Kilogram 1 == KilogramPerCubicMetre 1 * CubicMetre 1,
        Gram 1 == GramPerMillilitre 1 * Millilitre 1,

        // MassFlowRate == Mass / Time
        Kilogram 1 == KilogramPerSecond 1 * Second 1,
        Gram 1 == GramPerSecond 1 * Second 1,

        // MomentOfInertia == Mass * Area
        KilogramSquareMetre 1 == Kilogram 1 * SquareMetre 1,
        GramSquareCentimetre 1 == Gram 1 * SquareCentimetre 1,

        // Momentum == Force * Time
        NewtonSecond 1 == Newton 1 * Second 1,
        NewtonSecond 2 == Newton 2 * Second 1,
        NewtonSecond 3 == Newton 3 * Second 1,
        DyneSecond 1 == Dyne 1 * Second 1,
        DyneSecond 2 == Dyne 2 * Second 1,
        DyneSecond 3 == Dyne 3 * Second 1,

        // Momentum == Mass * Velocity
        NewtonSecond 1 == Kilogram 1 * MetrePerSecond 1,
        NewtonSecond 2 == Kilogram 1 * MetrePerSecond 2,
        NewtonSecond 3 == Kilogram 1 * MetrePerSecond 3,
        DyneSecond 1 == Gram 1 * CentimetrePerSecond 1,
        DyneSecond 2 == Gram 1 * CentimetrePerSecond 2,
        DyneSecond 3 == Gram 1 * CentimetrePerSecond 3,

        // Power == Energy / Time
        Joule 1 == Watt 1 * Second 1,
        WattHour 1 == Watt 1 * Hour 1,
        KilowattHour 1 == Kilowatt 1 * Hour 1,
        Erg 1 == ErgPerSecond 1 * Second 1,

        // Pressure == Force / Area
        Newton 1 == Pascal 1 * SquareMetre 1,
        PoundForce 1 == PoundForcePerSquareInch 1 * SquareInch 1,
        Newton 1 == Hectopascal 1 * SquareDecimetre 1,

        // SpecificEnergy == Energy / Mass
        Joule 1 == JoulePerKilogram 1 * Kilogram 1,

        // SpecificVolume == Volume / Mass
        CubicMetre 1 == CubicMetrePerKilogram 1 * Kilogram 1,

        // SpecificVolume == 1 / MassDensity
        One 1 == CubicMetrePerKilogram 1 * KilogramPerCubicMetre 1,

        // SurfaceDensity == Mass / Area
        Kilogram 1 == KilogramPerSquareMetre 1 * SquareMetre 1,

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
        JoulePerKelvin 1 == JoulePerKilogramPerKelvin 1 * Kilogram 1,

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
        Lumen 1 == Phot 1 * SquareCentimetre 1,
        Phot 1 == Stilb 1 * Steradian 1,
        Lumen 1 == FootCandle 1 * SquareFoot 1,
        FootCandle 1 == CandelaPerSquareFoot 1 * Steradian 1,

        // Irradiance == Power / Area
        Watt 1 == WattPerSquareMetre 1 * SquareMetre 1,

        // Luminance == LuminousIntensity / Area
        Candela 1 == CandelaPerSquareMetre 1 * SquareMetre 1,
        Candela 1 == Stilb 1 * SquareCentimetre 1,
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
        Millicoulomb 1 == Millifarad 1 * Volt 1,
        Coulomb 1 == Millifarad 1 * Kilovolt 1,
        Microcoulomb 1 == Microfarad 1 * Volt 1,
        Millicoulomb 1 == Microfarad 1 * Kilovolt 1,
        Nanocoulomb 1 == Nanofarad 1 * Volt 1,
        Microcoulomb 1 == Nanofarad 1 * Kilovolt 1,
        Picocoulomb 1 == Picofarad 1 * Volt 1,
        Nanocoulomb 1 == Picofarad 1 * Kilovolt 1,

        // CurrentDensity == ElectricCurrent * Area
        Ampere 1 == AmperePerSquareMetre 1 * SquareMetre 1,

        // ElectricalConductance == ElectricCurrent / ElectricPotential
        Ampere 1 == Siemens 1 * Volt 1,
        Milliampere 1 == Siemens 1 * Kilovolt 1,

        // ElectricalConductance == 1 / ElectricalResistance
        One 1 == Siemens 1 * Ohm 1,

        // ElectricalConductivity == ElectricalConductance / Length
        Siemens 1 == SiemensPerMetre 1 * Metre 1,

        // ElectricalResistance == ElectricPotential / ElectricCurrent
        Volt 1 == Ohm 1 * Ampere 1,

        // ElectricCurrent == ElectricCharge / Time
        Coulomb 1 == Ampere 1 * Second 1,
        Millicoulomb 1 == Ampere 1 * Millisecond 1,
        Millicoulomb 1 == Milliampere 1 * Second 1,
        Microcoulomb 1 == Milliampere 1 * Millisecond 1,
        Microcoulomb 1 == Microampere 1 * Second 1,

        // ElectricChargeDensity == ElectricCharge / Volume
        Coulomb 1 == CoulombPerCubicMetre 1 * CubicMetre 1,
        Millicoulomb 1 == CoulombPerCubicMetre 1 * Litre 1,
        Microcoulomb 1 == CoulombPerCubicMetre 1 * Millilitre 1,

        // ElectricDisplacement == ElectricCharge / Area
        Coulomb 1 == CoulombPerSquareMetre 1 * SquareMetre 1,
        Microcoulomb 1 == CoulombPerSquareMetre 1 * SquareMillimetre 1,

        // ElectricFieldStrength == ElectricPotential / Length
        Volt 1 == VoltPerMetre 1 * Metre 1,

        // ElectricFieldStrength == Force / ElectricCharge
        Newton 1 == NewtonPerCoulomb 1 * Coulomb 1,

        // ElectricPotential == Power / ElectricCurrent
        Watt 1 == Volt 1 * Ampere 1,
        Milliwatt 1 == Volt 1 * Milliampere 1,
        Watt 1 == Kilovolt 1 * Milliampere 1,
        Milliwatt 1 == Kilovolt 1 * Microampere 1,
        Kilowatt 1 == Kilovolt 1 * Ampere 1,
        Milliwatt 1 == Millivolt 1 * Ampere 1,

        // LinearElectricChargeDensity == ElectricCharge / Length
        Coulomb 1 == CoulombPerMetre 1 * Metre 1,
        Millicoulomb 1 == CoulombPerMetre 1 * Millimetre 1,
        Microcoulomb 1 == CoulombPerMetre 1 * Micrometre 1,

        // Permittivity == Capacitance / Length
        Farad 1 == FaradPerMetre 1 * Metre 1,
        Millifarad 1 == FaradPerMetre 1 * Millimetre 1,
        Microfarad 1 == FaradPerMetre 1 * Micrometre 1,
        Nanofarad 1 == FaradPerMetre 1 * Nanometre 1,

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
        Milliampere 1 == AmperePerMetre 1 * Millimetre 1,
        Microampere 1 == AmperePerMetre 1 * Micrometre 1,

        // TODO  MagneticFlux == Mass * Area / SquareTime / Current

        // ElectricFieldStrength == Velocity X MagneticFluxDensity
        VoltPerMetre 1 == MetrePerSecond 1 * Tesla 1,
        VoltPerMetre 1 == MetrePerSecond 2 X Tesla 2,
        VoltPerMetre 3 == MetrePerSecond 3 X Tesla 3,

        // MagneticFluxDensity == MagneticFlux / Area
        Weber 1 == Tesla 1 * SquareMetre 1,

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

#[allow(dead_code)]
pub type MetricTon = Tonne;

#[allow(dead_code)]
pub type LongTon = ImperialTon;

#[allow(dead_code)]
pub type ShortTon = USTon;

#[allow(dead_code)]
pub type Mach = One;

#[allow(dead_code)]
pub type WeberPerSquareMetre = Tesla;
