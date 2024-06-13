use crate::journeys::GpsTrace;
use crate::journeys::Trace;
use anyhow::Result;
use geo::FrechetDistance;
use geo::HaversineLength;

pub fn traces_validation(t1: GpsTrace, t2: GpsTrace) -> Result<(Trace, Trace)> {
    let line_string1: Trace = t1.into();
    let line_string2: Trace = t2.into();

    let dist = line_string1
        .as_ref()
        .frechet_distance(line_string2.as_ref());
    println!("Frechet distance: {}", dist);

    let [line_string1, line_string2] = [line_string1, line_string2]
        .iter()
        .map(|trace| {
            let line_string = trace.clone().simplified();
            let length = line_string.as_ref().haversine_length();
            println!("Line length: {} meters", length);

            line_string
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Ok((line_string1, line_string2))
}
