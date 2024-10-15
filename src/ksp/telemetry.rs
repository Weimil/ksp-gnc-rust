use krpc_client::services::space_center::{SpaceCenter, Vessel};
use krpc_client::stream::Stream;

pub struct TelemetryStream {
    pub ut: Stream<f64>,
    pub met: Stream<f64>,
    pub periapsis: Stream<f64>,
    pub time_to_periapsis: Stream<f64>,
    pub apoapsis: Stream<f64>,
    pub time_to_apoapsis: Stream<f64>,
    pub inclination: Stream<f64>,
    pub dynamic_pressure: Stream<f32>,
    pub static_pressure: Stream<f32>,
    pub surface_speed: Stream<f64>,
    pub surface_vertical_speed: Stream<f64>,
    pub surface_horizontal_speed: Stream<f64>,
    pub orbital_speed: Stream<f64>,
    pub orbital_vertical_speed: Stream<f64>,
    pub orbital_horizontal_speed: Stream<f64>,
    pub mean_altitude: Stream<f64>,
    pub surface_altitude: Stream<f64>,
}

impl TelemetryStream {
    pub(crate) fn new(vessel: &Vessel, space_center: &SpaceCenter) -> Result<TelemetryStream, Box<dyn std::error::Error>> {
        let vessel_orbit = vessel.get_orbit()?;
        let vessel_resources = vessel.get_resources()?;
        let vessel_orbit_body = vessel_orbit.get_body()?;

        let surface_reference_frame = vessel_orbit_body.get_reference_frame()?;
        let surface_vessel_flight = vessel.flight(Option::from(&surface_reference_frame))?;

        let orbital_reference_frame = vessel_orbit_body.get_orbital_reference_frame()?;
        let orbital_vessel_flight = vessel.flight(Option::from(&orbital_reference_frame))?;

        // ──────────────────────────────────────────────────────────────────

        let ut_stream = space_center.get_ut_stream()?;
        let met_stream = vessel.get_met_stream()?;

        let periapsis_stream = vessel_orbit.get_periapsis_altitude_stream()?;
        let time_to_periapsis_stream = vessel_orbit.get_time_to_periapsis_stream()?;
        let apoapsis_stream = vessel_orbit.get_apoapsis_altitude_stream()?;
        let time_to_apoapsis_stream = vessel_orbit.get_time_to_apoapsis_stream()?;
        let inclination_stream = vessel_orbit.get_inclination_stream()?;

        let static_pressure_stream = surface_vessel_flight.get_static_pressure_stream()?;
        let dynamic_pressure_stream = surface_vessel_flight.get_dynamic_pressure_stream()?;

        let surface_speed_stream = surface_vessel_flight.get_speed_stream()?;
        let surface_vertical_speed_stream = surface_vessel_flight.get_vertical_speed_stream()?;
        let surface_horizontal_speed_stream = surface_vessel_flight.get_horizontal_speed_stream()?;

        let orbital_speed_stream = orbital_vessel_flight.get_speed_stream()?;
        let orbital_vertical_speed_stream = orbital_vessel_flight.get_vertical_speed_stream()?;
        let orbital_horizontal_speed_stream = orbital_vessel_flight.get_horizontal_speed_stream()?;

        let mean_altitude_stream = orbital_vessel_flight.get_mean_altitude_stream()?;
        let surface_altitude_stream = orbital_vessel_flight.get_surface_altitude_stream()?;

        let resources_fuel_stream = vessel_resources.amount_stream("LiquidFuel".to_string())?;
        let resources_oxidizer_stream = vessel_resources.amount_stream("Oxidizer".to_string())?;
        let resources_electric_stream = vessel_resources.amount_stream("ElectricCharge".to_string())?;

        // ──────────────────────────────────────────────────────────────────

        Ok(Self {
            ut: ut_stream,
            met: met_stream,
            periapsis: periapsis_stream,
            time_to_periapsis: time_to_periapsis_stream,
            apoapsis: apoapsis_stream,
            time_to_apoapsis: time_to_apoapsis_stream,
            inclination: inclination_stream,
            dynamic_pressure: dynamic_pressure_stream,
            static_pressure: static_pressure_stream,
            surface_speed: surface_speed_stream,
            surface_vertical_speed: surface_vertical_speed_stream,
            surface_horizontal_speed: surface_horizontal_speed_stream,
            orbital_speed: orbital_speed_stream,
            orbital_vertical_speed: orbital_vertical_speed_stream,
            orbital_horizontal_speed: orbital_horizontal_speed_stream,
            mean_altitude: mean_altitude_stream,
            surface_altitude: surface_altitude_stream,
        })
    }

    pub(crate) fn data(&self) -> Result<Telemetry, Box<dyn std::error::Error>> {
        Ok(Telemetry {
            ut: self.ut.get()?,
            met: self.met.get()?,
            periapsis: self.periapsis.get()?,
            time_to_periapsis: self.time_to_periapsis.get()?,
            apoapsis: self.apoapsis.get()?,
            time_to_apoapsis: self.time_to_apoapsis.get()?,
            inclination: self.inclination.get()?,
            dynamic_pressure: self.dynamic_pressure.get()?,
            static_pressure: self.static_pressure.get()?,
            surface_speed: self.surface_speed.get()?,
            surface_vertical_speed: self.surface_vertical_speed.get()?,
            surface_horizontal_speed: self.surface_horizontal_speed.get()?,
            orbital_speed: self.orbital_speed.get()?,
            orbital_vertical_speed: self.orbital_vertical_speed.get()?,
            orbital_horizontal_speed: self.orbital_horizontal_speed.get()?,
            mean_altitude: self.mean_altitude.get()?,
            surface_altitude: self.surface_altitude.get()?,
        })
    }

    /*
    fn clear_and_print(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.print()?;
        print!("\x1b[24A");
        Ok(())
    }

    fn print(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("╭────────────────────────────────────────────────────────────────╮");
        println!("│ {:^62} │", "Orion Alpha Block 2");
        // println!("├────────────────────────────────────────────────────────────────┤");
        println!("├────────────────────────────────────────────────────────────────╯");
        println!("│ Universal Time: {} ", self.fmt_ut()?);
        println!("│ Mission elapsed time: {} ", self.met.get()?);
        // println!("├────────────────────────────────────────────────────────────────┤");
        println!("├─────────────────────────────────────────────────────────────────");
        println!("│ Periapsis: {:8.2}m ", self.periapsis.get()?);
        println!("│ Time to Periapsis: {:5.0}s ", self.time_to_periapsis.get()?);
        println!("│ Apoapsis: {:8.2}m ", self.apoapsis.get()?);
        println!("│ Time to Apoapsis: {:5.0}s ", self.time_to_apoapsis.get()?);
        // println!("├────────────────────────────────────────────────────────────────┤");
        println!("├─────────────────────────────────────────────────────────────────");
        println!("│ Dynamic Pressure          {:8.2}pa  │", self.dynamic_pressure.get()?);
        // println!("├────────────────────────────────────────────────────────────────┤");
        println!("├─────────────────────────────────────────────────────────────────");
        println!("│ Mean Altitude             {:8.2}m   │", self.mean_altitude.get()?);
        println!("│ Surface Altitude          {:8.2}m   │", self.surface_altitude.get()?);
        // println!("├────────────────────────────────────────────────────────────────┤");
        println!("├─────────────────────────────────────────────────────────────────");
        println!("│ Surface Speed             {:8.2}m/s │", self.surface_speed.get()?);
        println!("│ Surface Vertical Speed    {:8.2}m/s │", self.surface_vertical_speed.get()?);
        println!("│ Surface Horizontal Speed  {:8.2}m/s │", self.surface_horizontal_speed.get()?);
        // println!("├────────────────────────────────────────────────────────────────┤");
        println!("├─────────────────────────────────────────────────────────────────");
        println!("│ Orbital Speed             {:8.2}m/s │", self.orbital_speed.get()?);
        println!("│ Orbital Vertical Speed    {:8.2}m/s │", self.orbital_vertical_speed.get()?);
        println!("│ Orbital Horizontal Speed  {:8.2}m/s │", self.orbital_horizontal_speed.get()?);
        // println!("╰─────────────────────────────────────────────────────────────────");
        println!();

        return Ok(());
    }*/
}

#[derive(Debug)]
pub(crate) struct Telemetry {
    pub ut: f64,
    pub met: f64,
    pub periapsis: f64,
    pub time_to_periapsis: f64,
    pub apoapsis: f64,
    pub time_to_apoapsis: f64,
    pub inclination: f64,
    pub dynamic_pressure: f32,
    pub static_pressure: f32,
    pub surface_speed: f64,
    pub surface_vertical_speed: f64,
    pub surface_horizontal_speed: f64,
    pub orbital_speed: f64,
    pub orbital_vertical_speed: f64,
    pub orbital_horizontal_speed: f64,
    pub mean_altitude: f64,
    pub surface_altitude: f64,
}

// impl Telemetry {
//     pub(crate) fn new() -> Telemetry {
//         Telemetry {}
//     }
// }
